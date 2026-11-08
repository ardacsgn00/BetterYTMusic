//! Audio engine abstraction + real `rodio`/`symphonia` engine with background
//! Innertube stream resolution and sample-accurate packet seeking.

use std::io::{Cursor, Read, Seek, SeekFrom};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::{OutputStream, Sink, Source};
use symphonia::core::audio::{SampleBuffer, SignalSpec};
use symphonia::core::codecs::{Decoder as SymDecoder, DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error as SymError;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::{MediaSource, MediaSourceStream};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::innertube::Innertube;
use crate::mock::Track;

/// Enumerate all available audio output device names on the host OS.
pub fn list_output_devices() -> Vec<String> {
    let host = rodio::cpal::default_host();
    let mut out = Vec::new();
    if let Ok(devices) = host.output_devices() {
        for dev in devices {
            if let Ok(name) = dev.name() {
                let trimmed = name.trim().to_string();
                if !trimmed.is_empty() && !out.contains(&trimmed) {
                    out.push(trimmed);
                }
            }
        }
    }
    out
}

fn open_output_stream(
    device_name: Option<&str>,
) -> Option<(OutputStream, rodio::OutputStreamHandle)> {
    if let Some(target) = device_name.map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let host = rodio::cpal::default_host();
        if let Ok(mut devices) = host.output_devices() {
            if let Some(dev) = devices.find(|d| d.name().ok().as_deref().map(str::trim) == Some(target)) {
                if let Ok(pair) = OutputStream::try_from_device(&dev) {
                    return Some(pair);
                }
            }
        }
    }
    OutputStream::try_default().ok()
}

/// Abstraction over the audio backend.
pub trait AudioEngine: Send {
    fn load(&mut self, track: &Track);
    fn play(&mut self);
    fn pause(&mut self);
    fn seek(&mut self, position_ms: u64);
    fn set_volume(&mut self, volume: f32);
    fn set_output_device(&mut self, device_name: Option<String>);
    fn is_playing(&self) -> bool;
    fn position_ms(&self) -> u64;
    fn duration_ms(&self) -> u64;
    fn is_finished(&self) -> bool;
}

enum EngineCmd {
    Load(Track),
    Play,
    Pause,
    Seek(u64),
    SetVolume(f32),
    SetOutputDevice(Option<String>),
}

struct SharedClock {
    base_ms: AtomicU64,
    started_epoch_ms: AtomicU64, // 0 when paused or waiting for buffer
    duration_ms: AtomicU64,
    playing: AtomicBool,
    has_audio: AtomicBool,
    finished: AtomicBool,
    epoch: Instant,
}

impl SharedClock {
    fn new() -> Self {
        Self {
            base_ms: AtomicU64::new(0),
            started_epoch_ms: AtomicU64::new(0),
            duration_ms: AtomicU64::new(0),
            playing: AtomicBool::new(false),
            has_audio: AtomicBool::new(false),
            finished: AtomicBool::new(false),
            epoch: Instant::now(),
        }
    }

    fn now_ms(&self) -> u64 {
        self.epoch.elapsed().as_millis() as u64
    }

    fn position_ms(&self) -> u64 {
        let base = self.base_ms.load(Ordering::Relaxed);
        let started = self.started_epoch_ms.load(Ordering::Relaxed);
        let running = if started > 0 {
            self.now_ms().saturating_sub(started)
        } else {
            0
        };
        let dur = self.duration_ms.load(Ordering::Relaxed);
        if dur > 0 {
            (base + running).min(dur)
        } else {
            base + running
        }
    }

    fn mark_playing(&self) {
        if self.has_audio.load(Ordering::Relaxed)
            && self.started_epoch_ms.load(Ordering::Relaxed) == 0
        {
            let now = self.now_ms().max(1);
            self.started_epoch_ms.store(now, Ordering::Relaxed);
        }
        self.playing.store(true, Ordering::Relaxed);
    }

    fn mark_paused(&self) {
        let pos = self.position_ms();
        self.base_ms.store(pos, Ordering::Relaxed);
        self.started_epoch_ms.store(0, Ordering::Relaxed);
        self.playing.store(false, Ordering::Relaxed);
    }

    fn reset(&self, duration_ms: u64) {
        self.base_ms.store(0, Ordering::Relaxed);
        self.started_epoch_ms.store(0, Ordering::Relaxed);
        self.duration_ms.store(duration_ms, Ordering::Relaxed);
        self.playing.store(false, Ordering::Relaxed);
        self.has_audio.store(false, Ordering::Relaxed);
        self.finished.store(false, Ordering::Relaxed);
    }

    fn set_position(&self, pos_ms: u64) {
        let dur = self.duration_ms.load(Ordering::Relaxed);
        let clamped = if dur > 0 { pos_ms.min(dur) } else { pos_ms };
        self.base_ms.store(clamped, Ordering::Relaxed);
        self.finished.store(false, Ordering::Relaxed);
        if self.started_epoch_ms.load(Ordering::Relaxed) > 0 {
            let now = self.now_ms().max(1);
            self.started_epoch_ms.store(now, Ordering::Relaxed);
        }
    }
}

/// Custom `MediaSource` over `Arc<[u8]>` that reports its exact `byte_len()`.
struct ArcBytesMediaSource {
    cursor: Cursor<Arc<[u8]>>,
    len: u64,
}

impl ArcBytesMediaSource {
    fn new(bytes: Arc<[u8]>) -> Self {
        let len = bytes.len() as u64;
        Self {
            cursor: Cursor::new(bytes),
            len,
        }
    }
}

impl Read for ArcBytesMediaSource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.cursor.read(buf)
    }
}

impl Seek for ArcBytesMediaSource {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.cursor.seek(pos)
    }
}

impl MediaSource for ArcBytesMediaSource {
    fn is_seekable(&self) -> bool {
        true
    }

    fn byte_len(&self) -> Option<u64> {
        Some(self.len)
    }
}

/// Symphonia-backed `rodio::Source` that supports fragmented MP4 (`itag=140`)
/// and fast packet-level seeking (`new_at`) without `rodio::skip_duration` bugs.
pub struct SymphoniaSource {
    decoder: Box<dyn SymDecoder>,
    format: Box<dyn FormatReader>,
    track_id: u32,
    buffer: SampleBuffer<i16>,
    spec: SignalSpec,
    current_frame_offset: usize,
    total_duration: Option<Duration>,
}

impl SymphoniaSource {
    /// Creates a decoder positioned at `start_pos` by fast-skipping demuxer packets
    /// before AAC decoding, landing accurately at any timestamp in < 2 ms.
    pub fn new_at(bytes: Arc<[u8]>, start_pos: Duration) -> Result<Self, String> {
        let source = Box::new(ArcBytesMediaSource::new(bytes)) as Box<dyn MediaSource>;
        let mss = MediaSourceStream::new(source, Default::default());

        let mut hint = Hint::new();
        hint.with_extension("m4a");
        let format_opts = FormatOptions {
            enable_gapless: true,
            ..Default::default()
        };
        let metadata_opts = MetadataOptions::default();

        let mut probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .map_err(|e| format!("format probe error: {e}"))?;

        let track = probed
            .format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or_else(|| "no supported audio track found".to_string())?;

        let track_id = track.id;
        let sample_rate = track.codec_params.sample_rate.unwrap_or(44_100);
        let time_base = track.codec_params.time_base;
        let total_duration = time_base
            .zip(track.codec_params.n_frames)
            .map(|(base, frames)| {
                let t = base.calc_time(frames);
                Duration::from_secs(t.seconds) + Duration::from_secs_f64(t.frac)
            });

        // Target timestamp in track time_base units (for AAC M4A, time_base is 1/sample_rate).
        let target_ts: u64 = if start_pos > Duration::ZERO {
            if let Some(tb) = time_base {
                tb.calc_timestamp(symphonia::core::units::Time {
                    seconds: start_pos.as_secs(),
                    frac: start_pos.subsec_nanos() as f64 / 1_000_000_000.0,
                })
            } else {
                (start_pos.as_secs_f64() * sample_rate as f64) as u64
            }
        } else {
            0
        };

        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &DecoderOptions::default())
            .map_err(|e| format!("codec init error: {e}"))?;

        let mut retries = 0usize;
        let mut accum_ts: u64 = 0;

        let (spec, buffer, initial_offset) = loop {
            let packet = probed
                .format
                .next_packet()
                .map_err(|e| format!("packet read error: {e}"))?;
            if packet.track_id() != track_id {
                continue;
            }

            let pkt_ts = if packet.ts() > 0 || accum_ts == 0 {
                packet.ts()
            } else {
                accum_ts
            };
            let pkt_dur = if packet.dur() > 0 { packet.dur() } else { 1024 };
            let pkt_end = pkt_ts.saturating_add(pkt_dur);
            accum_ts = pkt_end;

            // Fast-skip packets that end well before our target seek position.
            // Decode 1 warm-up packet right before `target_ts` so AAC MDCT overlap is clean.
            if target_ts > 0 && pkt_end.saturating_add(pkt_dur) <= target_ts {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(decoded) => {
                    if target_ts > 0 && pkt_end <= target_ts {
                        // This was the warm-up packet immediately preceding target_ts;
                        // now decode the target packet on the next iteration.
                        continue;
                    }
                    let spec = *decoded.spec();
                    let channels = spec.channels.count().max(1);
                    let mut buf = SampleBuffer::<i16>::new(decoded.capacity() as u64, spec);
                    buf.copy_interleaved_ref(decoded);

                    let skip_frames = target_ts.saturating_sub(pkt_ts) as usize;
                    let offset = (skip_frames * channels).min(buf.len().saturating_sub(channels));
                    break (spec, buf, offset);
                }
                Err(SymError::DecodeError(_)) if retries < 6 => {
                    retries += 1;
                    continue;
                }
                Err(e) => return Err(format!("decode error: {e}")),
            }
        };

        Ok(Self {
            decoder,
            format: probed.format,
            track_id,
            buffer,
            spec,
            current_frame_offset: initial_offset,
            total_duration,
        })
    }
}

impl Iterator for SymphoniaSource {
    type Item = i16;

    #[inline]
    fn next(&mut self) -> Option<i16> {
        while self.current_frame_offset >= self.buffer.len() {
            let packet = self.format.next_packet().ok()?;
            if packet.track_id() != self.track_id {
                continue;
            }
            match self.decoder.decode(&packet) {
                Ok(decoded) => {
                    self.spec = *decoded.spec();
                    self.buffer = SampleBuffer::<i16>::new(decoded.capacity() as u64, self.spec);
                    self.buffer.copy_interleaved_ref(decoded);
                    self.current_frame_offset = 0;
                }
                Err(SymError::DecodeError(_)) => continue,
                Err(_) => return None,
            }
        }

        let sample = *self.buffer.samples().get(self.current_frame_offset)?;
        self.current_frame_offset += 1;
        Some(sample)
    }
}

impl Source for SymphoniaSource {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        // Returning `None` indicates constant sample_rate and channels for the entire stream,
        // preventing rodio from misinterpreting packet boundaries as end-of-stream.
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.spec.channels.count() as u16
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.spec.rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.total_duration
    }
}

/// Real audio engine driven by `rodio` + `symphonia` on a dedicated worker thread.
pub struct RodioEngine {
    tx: Sender<EngineCmd>,
    clock: Arc<SharedClock>,
    load_seq: Arc<AtomicU64>,
}

impl RodioEngine {
    pub fn new(innertube: Arc<Innertube>) -> Self {
        let (tx, rx) = mpsc::channel::<EngineCmd>();
        let clock = Arc::new(SharedClock::new());
        let load_seq = Arc::new(AtomicU64::new(0));

        let worker_clock = clock.clone();
        let worker_seq = load_seq.clone();

        thread::Builder::new()
            .name("audio-engine".into())
            .spawn(move || {
                run_worker(rx, worker_clock, worker_seq, innertube);
            })
            .expect("failed to spawn audio engine thread");

        Self {
            tx,
            clock,
            load_seq,
        }
    }
}

impl AudioEngine for RodioEngine {
    fn load(&mut self, track: &Track) {
        self.load_seq.fetch_add(1, Ordering::SeqCst);
        self.clock.reset(track.duration_ms);
        let _ = self.tx.send(EngineCmd::Load(track.clone()));
    }

    fn play(&mut self) {
        self.clock.mark_playing();
        let _ = self.tx.send(EngineCmd::Play);
    }

    fn pause(&mut self) {
        self.clock.mark_paused();
        let _ = self.tx.send(EngineCmd::Pause);
    }

    fn seek(&mut self, position_ms: u64) {
        self.clock.set_position(position_ms);
        let _ = self.tx.send(EngineCmd::Seek(position_ms));
    }

    fn set_volume(&mut self, volume: f32) {
        let _ = self.tx.send(EngineCmd::SetVolume(volume));
    }

    fn set_output_device(&mut self, device_name: Option<String>) {
        let _ = self.tx.send(EngineCmd::SetOutputDevice(device_name));
    }

    fn is_playing(&self) -> bool {
        self.clock.playing.load(Ordering::Relaxed)
    }

    fn position_ms(&self) -> u64 {
        self.clock.position_ms()
    }

    fn duration_ms(&self) -> u64 {
        self.clock.duration_ms.load(Ordering::Relaxed)
    }

    fn is_finished(&self) -> bool {
        // Only report finished when the audio sink has actually finished playing all samples.
        self.clock.finished.load(Ordering::Relaxed)
    }
}

struct DecodedBuffer {
    seq: u64,
    bytes: Arc<[u8]>,
    duration_ms: Option<u64>,
}

fn run_worker(
    rx: mpsc::Receiver<EngineCmd>,
    clock: Arc<SharedClock>,
    load_seq: Arc<AtomicU64>,
    innertube: Arc<Innertube>,
) {
    let mut stream_pair = open_output_stream(None);
    let mut sink: Option<Sink> = stream_pair
        .as_ref()
        .and_then(|(_, handle)| Sink::try_new(handle).ok());

    let (buf_tx, buf_rx) = mpsc::channel::<DecodedBuffer>();

    let mut current_bytes: Option<Arc<[u8]>> = None;
    let mut volume: f32 = 0.8;
    let mut desired_playing = false;
    let mut is_mock = true;
    let mut active_seq: u64 = 0;

    if let Some(s) = &sink {
        s.set_volume(volume);
    }

    loop {
        // Check if a background stream download completed.
        while let Ok(ready) = buf_rx.try_recv() {
            if ready.seq != active_seq {
                continue;
            }
            if let Some(dur) = ready.duration_ms {
                if dur > 0 {
                    clock.duration_ms.store(dur, Ordering::Relaxed);
                }
            }
            current_bytes = Some(ready.bytes.clone());

            if let Some((_, handle)) = &stream_pair {
                if let Ok(new_sink) = Sink::try_new(handle) {
                    new_sink.set_volume(volume);
                    let seek_pos = clock.base_ms.load(Ordering::Relaxed);
                    match SymphoniaSource::new_at(ready.bytes, Duration::from_millis(seek_pos)) {
                        Ok(decoder) => {
                            if let Some(total) = decoder.total_duration() {
                                let ms = total.as_millis() as u64;
                                if ms > 0 {
                                    clock.duration_ms.store(ms, Ordering::Relaxed);
                                }
                            }
                            new_sink.append(decoder);
                            clock.has_audio.store(true, Ordering::Relaxed);
                            clock.finished.store(false, Ordering::Relaxed);
                            if desired_playing {
                                let now = clock.now_ms().max(1);
                                clock.started_epoch_ms.store(now, Ordering::Relaxed);
                                clock.playing.store(true, Ordering::Relaxed);
                                new_sink.play();
                            } else {
                                clock.started_epoch_ms.store(0, Ordering::Relaxed);
                                new_sink.pause();
                            }
                            sink = Some(new_sink);
                        }
                        Err(e) => {
                            eprintln!("[audio] decode error: {e}");
                        }
                    }
                }
            }
        }

        // Detect natural end-of-sink for real audio tracks.
        if !is_mock && desired_playing {
            if let Some(s) = &sink {
                if current_bytes.is_some() && clock.has_audio.load(Ordering::Relaxed) && s.empty() {
                    clock.finished.store(true, Ordering::Relaxed);
                }
            }
        }

        match rx.recv_timeout(Duration::from_millis(40)) {
            Ok(cmd) => match cmd {
                EngineCmd::Load(track) => {
                    active_seq = load_seq.load(Ordering::SeqCst);
                    current_bytes = None;
                    desired_playing = false;
                    if let Some(s) = &sink {
                        s.stop();
                    }
                    is_mock = track.id.starts_with("mock-");
                    if !is_mock {
                        let tx_done = buf_tx.clone();
                        let yt = innertube.clone();
                        let seq = active_seq;
                        let vid = track.id.clone();
                        let seq_ref = load_seq.clone();
                        thread::spawn(move || {
                            if let Some(buf) = fetch_audio_bytes(&yt, &vid, seq, &seq_ref) {
                                let _ = tx_done.send(buf);
                            }
                        });
                    }
                }
                EngineCmd::Play => {
                    desired_playing = true;
                    if let Some(s) = &sink {
                        s.play();
                    }
                }
                EngineCmd::Pause => {
                    desired_playing = false;
                    if let Some(s) = &sink {
                        s.pause();
                    }
                }
                EngineCmd::Seek(pos_ms) => {
                    let dur = clock.duration_ms.load(Ordering::Relaxed);
                    // Clamp seek target at least 500ms before end so decoder doesn't EOF on seek.
                    let clamped_ms = if dur > 600 {
                        pos_ms.min(dur - 500)
                    } else {
                        pos_ms
                    };
                    let target = Duration::from_millis(clamped_ms);
                    if let (Some(bytes), Some((_, handle))) = (&current_bytes, &stream_pair) {
                        if let Some(old_sink) = sink.take() {
                            old_sink.stop();
                        }
                        if let Ok(new_sink) = Sink::try_new(handle) {
                            new_sink.set_volume(volume);
                            if let Ok(dec) = SymphoniaSource::new_at(bytes.clone(), target) {
                                new_sink.append(dec);
                                clock.base_ms.store(clamped_ms, Ordering::Relaxed);
                                clock.finished.store(false, Ordering::Relaxed);
                                if desired_playing {
                                    let now = clock.now_ms().max(1);
                                    clock.started_epoch_ms.store(now, Ordering::Relaxed);
                                    new_sink.play();
                                } else {
                                    clock.started_epoch_ms.store(0, Ordering::Relaxed);
                                    new_sink.pause();
                                }
                                sink = Some(new_sink);
                            }
                        }
                    }
                }
                EngineCmd::SetVolume(v) => {
                    volume = v;
                    if let Some(s) = &sink {
                        s.set_volume(volume);
                    }
                }
                EngineCmd::SetOutputDevice(dev_name) => {
                    let pos_ms = clock.position_ms();
                    let dur = clock.duration_ms.load(Ordering::Relaxed);
                    let clamped_ms = if dur > 600 {
                        pos_ms.min(dur - 500)
                    } else {
                        pos_ms
                    };
                    if let Some(old_sink) = sink.take() {
                        old_sink.stop();
                    }
                    drop(stream_pair.take());
                    stream_pair = open_output_stream(dev_name.as_deref());
                    if let Some((_, handle)) = &stream_pair {
                        if let Ok(new_sink) = Sink::try_new(handle) {
                            new_sink.set_volume(volume);
                            if let Some(bytes) = &current_bytes {
                                if clock.has_audio.load(Ordering::Relaxed) {
                                    let target = Duration::from_millis(clamped_ms);
                                    if let Ok(dec) = SymphoniaSource::new_at(bytes.clone(), target) {
                                        new_sink.append(dec);
                                        clock.base_ms.store(clamped_ms, Ordering::Relaxed);
                                        clock.finished.store(false, Ordering::Relaxed);
                                        if desired_playing {
                                            let now = clock.now_ms().max(1);
                                            clock.started_epoch_ms.store(now, Ordering::Relaxed);
                                            new_sink.play();
                                        } else {
                                            clock.started_epoch_ms.store(0, Ordering::Relaxed);
                                            new_sink.pause();
                                        }
                                    }
                                }
                            }
                            sink = Some(new_sink);
                        }
                    }
                }
            },
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
}

fn fetch_audio_bytes(
    innertube: &Innertube,
    video_id: &str,
    seq: u64,
    latest_seq: &AtomicU64,
) -> Option<DecodedBuffer> {
    let resolved = match innertube.resolve_stream(video_id) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[audio] stream resolve error for {video_id}: {e}");
            return None;
        }
    };

    if latest_seq.load(Ordering::SeqCst) != seq {
        return None;
    }

    let mut req = innertube
        .agent()
        .get(&resolved.url)
        .set("User-Agent", resolved.user_agent);

    if let Some(clen) = resolved.content_length {
        if clen > 0 {
            req = req.set("Range", &format!("bytes=0-{}", clen - 1));
        }
    } else {
        req = req.set("Range", "bytes=0-15728639");
    }

    let res = req.call().ok()?;

    let mut reader = res.into_reader();
    let cap = resolved.content_length.unwrap_or(4 * 1024 * 1024) as usize;
    let mut data = Vec::with_capacity(cap.min(32 * 1024 * 1024));
    let mut chunk = [0u8; 64 * 1024];

    loop {
        if latest_seq.load(Ordering::SeqCst) != seq {
            return None;
        }
        let n = reader.read(&mut chunk).ok()?;
        if n == 0 {
            break;
        }
        data.extend_from_slice(&chunk[..n]);
    }

    Some(DecodedBuffer {
        seq,
        bytes: Arc::from(data),
        duration_ms: resolved.duration_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_seek_t2() {
        let p = std::env::temp_dir().join("t2_full.m4a");
        if !p.exists() {
            return;
        }
        let bytes: Arc<[u8]> = std::fs::read(&p).unwrap().into();
        // Seek to 100s inside a 191.1s track — remaining samples should be ~91.1s (~8,035,000 stereo samples)
        let dec = SymphoniaSource::new_at(bytes, Duration::from_secs(100))
            .expect("SymphoniaSource::new_at failed");
        let remaining_samples = dec.count();
        let remaining_secs = remaining_samples as f64 / (44100.0 * 2.0);
        println!("remaining after seek(100s) = {:.2}s", remaining_secs);
        assert!(remaining_secs > 89.0 && remaining_secs < 93.0);
    }
}
