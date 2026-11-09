pub mod engine;

use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};

use crate::innertube::Innertube;
use crate::media::{MediaSender, MediaUpdate};
use crate::mock::Track;
use crate::taskbar::TaskbarHandle;
use engine::{AudioEngine, RodioEngine};

/// Event emitted to the frontend whenever player state changes.
pub const STATE_EVENT: &str = "player:state";

/// "Previous" restarts the current track if we're past this point.
const RESTART_THRESHOLD_MS: u64 = 3_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum RepeatMode {
    #[default]
    Off,
    All,
    One,
}

/// Serializable view of the player sent over IPC.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSnapshot {
    pub track: Option<Track>,
    pub queue: Vec<Track>,
    pub index: Option<usize>,
    pub is_playing: bool,
    pub position_ms: u64,
    pub duration_ms: u64,
    pub volume: f32,
    pub repeat: RepeatMode,
    pub shuffle: bool,
    pub source_name: Option<String>,
    pub output_device: Option<String>,
}

pub struct Player {
    engine: Box<dyn AudioEngine>,
    queue: Vec<Track>,
    index: Option<usize>,
    volume: f32,
    repeat: RepeatMode,
    shuffle: bool,
    source_name: Option<String>,
    output_device: Option<String>,
}

impl Player {
    pub fn new(engine: Box<dyn AudioEngine>, queue: Vec<Track>) -> Self {
        let mut p = Self {
            engine,
            queue,
            index: None,
            volume: 0.8,
            repeat: RepeatMode::Off,
            shuffle: false,
            source_name: None,
            output_device: None,
        };
        p.engine.set_volume(p.volume);
        if !p.queue.is_empty() {
            p.load(0, false);
        }
        p
    }

    pub fn current(&self) -> Option<&Track> {
        self.index.and_then(|i| self.queue.get(i))
    }

    fn load(&mut self, index: usize, autoplay: bool) {
        let Some(track) = self.queue.get(index) else { return };
        self.engine.load(track);
        self.index = Some(index);
        if autoplay {
            self.engine.play();
        }
    }

    pub fn play(&mut self) {
        if self.index.is_none() && !self.queue.is_empty() {
            self.load(0, false);
        }
        if self.engine.is_finished() {
            self.engine.seek(0);
        }
        self.engine.play();
    }

    pub fn pause(&mut self) {
        self.engine.pause();
    }

    pub fn toggle(&mut self) {
        if self.engine.is_playing() {
            self.pause();
        } else {
            self.play();
        }
    }

    /// User-initiated "next": always moves forward, wrapping at the end.
    pub fn next(&mut self) {
        if self.queue.is_empty() {
            return;
        }
        let was_playing = self.engine.is_playing();
        let next = self.index.map_or(0, |i| (i + 1) % self.queue.len());
        self.load(next, was_playing);
    }

    pub fn previous(&mut self) {
        if self.queue.is_empty() {
            return;
        }
        if self.engine.position_ms() > RESTART_THRESHOLD_MS {
            self.engine.seek(0);
            return;
        }
        let was_playing = self.engine.is_playing();
        let len = self.queue.len();
        let prev = self.index.map_or(0, |i| (i + len - 1) % len);
        self.load(prev, was_playing);
    }

    pub fn play_index(&mut self, index: usize) {
        if index < self.queue.len() {
            self.load(index, true);
        }
    }

    pub fn set_queue(
        &mut self,
        queue: Vec<Track>,
        start_index: usize,
        source_name: Option<String>,
    ) {
        self.queue = queue;
        self.source_name = source_name;
        if !self.queue.is_empty() {
            let idx = start_index.min(self.queue.len() - 1);
            self.load(idx, true);
        } else {
            self.index = None;
            self.engine.pause();
        }
    }

    pub fn set_initial_queue(&mut self, queue: Vec<Track>) {
        if self.queue.is_empty() && !queue.is_empty() {
            self.queue = queue;
            self.source_name = Some("Ana Sayfa".to_string());
            self.load(0, false);
        }
    }

    pub fn append_unique(&mut self, tracks: Vec<Track>) {
        for t in tracks {
            if !self.queue.iter().any(|x| x.id == t.id) {
                self.queue.push(t);
            }
        }
    }

    pub fn seek(&mut self, position_ms: u64) {
        self.engine.seek(position_ms);
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        self.engine.set_volume(self.volume);
    }

    pub fn set_output_device(&mut self, device: Option<String>) {
        let clean = device
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        self.output_device = clean.clone();
        self.engine.set_output_device(clean);
    }

    pub fn cycle_repeat(&mut self) {
        self.repeat = match self.repeat {
            RepeatMode::Off => RepeatMode::All,
            RepeatMode::All => RepeatMode::One,
            RepeatMode::One => RepeatMode::Off,
        };
    }

    pub fn toggle_shuffle(&mut self) {
        self.shuffle = !self.shuffle;
        if self.shuffle && self.queue.len() > 2 {
            // Keep the currently playing track at index 0 and shuffle the remaining tracks.
            if let Some(cur_idx) = self.index {
                if cur_idx < self.queue.len() {
                    self.queue.swap(0, cur_idx);
                    self.index = Some(0);
                }
            }
            let mut seed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0x9E37_79B9_7F4A_7C15);
            let start = if self.index.is_some() { 1 } else { 0 };
            for i in (start + 1..self.queue.len()).rev() {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                let j = start + (seed as usize % (i - start + 1));
                self.queue.swap(i, j);
            }
        }
    }

    /// Called by the ticker when the current track ends naturally.
    /// Returns `true` if state changed.
    fn on_track_end(&mut self) -> bool {
        let Some(i) = self.index else { return false };
        match self.repeat {
            RepeatMode::One => {
                self.engine.seek(0);
                self.engine.play();
            }
            _ if i + 1 < self.queue.len() => self.load(i + 1, true),
            RepeatMode::All => self.load(0, true),
            // End of queue: rewind to the first track and stop.
            RepeatMode::Off => self.load(0, false),
        }
        true
    }

    pub fn snapshot(&self) -> PlayerSnapshot {
        PlayerSnapshot {
            track: self.current().cloned(),
            queue: self.queue.clone(),
            index: self.index,
            is_playing: self.engine.is_playing(),
            position_ms: self.engine.position_ms(),
            duration_ms: self.engine.duration_ms(),
            volume: self.volume,
            repeat: self.repeat,
            shuffle: self.shuffle,
            source_name: self.source_name.clone(),
            output_device: self.output_device.clone(),
        }
    }
}

/// Thread-safe owner of the [`Player`], managed as Tauri state.
/// Every mutation goes through [`PlayerController::with`], which re-broadcasts
/// state to the webview and the OS media controls.
pub struct PlayerController {
    innertube: Arc<Innertube>,
    player: Mutex<Player>,
    media: Mutex<Option<MediaSender>>,
    taskbar: Mutex<Option<TaskbarHandle>>,
    last_track_id: Mutex<Option<String>>,
    last_autoplay_id: Mutex<Option<String>>,
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

impl PlayerController {
    pub fn new(innertube: Arc<Innertube>, queue: Vec<Track>) -> Self {
        Self {
            innertube: innertube.clone(),
            player: Mutex::new(Player::new(Box::new(RodioEngine::new(innertube)), queue)),
            media: Mutex::new(None),
            taskbar: Mutex::new(None),
            last_track_id: Mutex::new(None),
            last_autoplay_id: Mutex::new(None),
        }
    }

    pub fn attach_media(&self, tx: Option<MediaSender>) {
        *lock(&self.media) = tx;
    }

    pub fn attach_taskbar(&self, tb: Option<TaskbarHandle>) {
        *lock(&self.taskbar) = tb;
    }

    pub fn snapshot(&self) -> PlayerSnapshot {
        lock(&self.player).snapshot()
    }

    /// Mutate the player, then broadcast the new state.
    pub fn with<R>(&self, app: &AppHandle, f: impl FnOnce(&mut Player) -> R) -> R {
        let (result, snap) = {
            let mut p = lock(&self.player);
            let r = f(&mut p);
            (r, p.snapshot())
        };
        self.publish(app, &snap);
        result
    }

    pub fn broadcast(&self, app: &AppHandle) {
        let snap = self.snapshot();
        self.publish(app, &snap);
    }

    fn publish(&self, app: &AppHandle, snap: &PlayerSnapshot) {
        let _ = app.emit(STATE_EVENT, snap);

        if let Some(tb) = *lock(&self.taskbar) {
            tb.set_playing(snap.is_playing);
        }

        let media = lock(&self.media);
        let Some(tx) = media.as_ref() else { return };

        // Only push metadata when the track actually changes.
        let id = snap.track.as_ref().map(|t| t.id.clone());
        let mut last = lock(&self.last_track_id);
        if *last != id {
            *last = id;
            if let Some(t) = &snap.track {
                let _ = tx.send(MediaUpdate::Metadata {
                    title: t.title.clone(),
                    artist: t.artist.clone(),
                    album: t.album.clone(),
                    cover_url: t.thumbnail.clone(),
                    duration_ms: t.duration_ms,
                });
            }
        }
        let _ = tx.send(MediaUpdate::Playback {
            playing: snap.is_playing,
            position_ms: snap.position_ms,
        });
    }

    /// Detects natural end-of-track and advances the queue, and updates duration if newly decoded.
    fn tick(&self, app: &AppHandle) {
        let (ended, dur_changed, needs_autoplay_id) = {
            let mut p = lock(&self.player);
            let ended = p.engine.is_playing() && p.engine.is_finished();
            let eng_dur = p.engine.duration_ms();
            let mut dur_changed = false;
            let mut needs_autoplay_id = None;
            if let Some(idx) = p.index {
                if let Some(t) = p.queue.get_mut(idx) {
                    if eng_dur > 0 && t.duration_ms != eng_dur {
                        t.duration_ms = eng_dur;
                        dur_changed = true;
                    }
                }
                // Proactively trigger autoplay when on the last track of queue
                if p.repeat == RepeatMode::Off && idx + 1 >= p.queue.len() && (ended || p.engine.is_playing()) {
                    needs_autoplay_id = Some(p.queue[idx].id.clone());
                }
            }
            (ended, dur_changed, needs_autoplay_id)
        };

        if let Some(vid) = needs_autoplay_id {
            let autoplay = app
                .try_state::<Arc<crate::settings::SettingsManager>>()
                .map(|s| s.get().autoplay)
                .unwrap_or(true);

            if autoplay {
                let mut last_ap = lock(&self.last_autoplay_id);
                if *last_ap != Some(vid.clone()) {
                    *last_ap = Some(vid.clone());
                    let yt = self.innertube.clone();
                    let app_clone = app.clone();
                    tauri::async_runtime::spawn(async move {
                        if let Ok(radio) = yt.radio_queue(&vid) {
                            let ctrl = app_clone.state::<PlayerController>();
                            ctrl.with(&app_clone, |p| {
                                p.append_unique(radio);
                            });
                        }
                    });
                }
            }
        }

        if ended {
            self.with(app, |p| p.on_track_end());
        } else if dur_changed {
            self.broadcast(app);
        }
    }
}

/// Background loop that watches for end-of-track. Cheap: one lock every 250 ms.
/// The frontend interpolates the progress bar locally, so no per-tick IPC.
pub fn spawn_ticker(app: AppHandle) {
    thread::Builder::new()
        .name("player-ticker".into())
        .spawn(move || loop {
            thread::sleep(Duration::from_millis(250));
            app.state::<PlayerController>().tick(&app);
        })
        .expect("failed to spawn player ticker");
}

