//! OS media integration via `souvlaki`.
//!
//! Windows: System Media Transport Controls (SMTC) — media overlay + hardware keys.
//! Linux:   MPRIS over D-Bus.
//! macOS:   MPNowPlayingInfoCenter / MPRemoteCommandCenter.
//!
//! `MediaControls` lives on its own thread and is fed through a channel, so the
//! player never blocks on OS calls and we don't depend on it being `Send`.

use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use souvlaki::{
    MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, MediaPosition, PlatformConfig,
    SeekDirection,
};
use tauri::{AppHandle, Manager};

use crate::player::PlayerController;

pub type MediaSender = Sender<MediaUpdate>;

pub enum MediaUpdate {
    Metadata {
        title: String,
        artist: String,
        album: String,
        cover_url: Option<String>,
        duration_ms: u64,
    },
    Playback {
        playing: bool,
        position_ms: u64,
    },
}

const SEEK_STEP_MS: u64 = 10_000;

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn spawn(_app: &AppHandle) -> Option<MediaSender> {
    None
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn spawn(app: &AppHandle) -> Option<MediaSender> {
    // SMTC needs the HWND of our window; raw pointers aren't Send, so pass as usize.
    #[cfg(target_os = "windows")]
    let hwnd: Option<usize> = app
        .get_webview_window("main")
        .and_then(|w| w.hwnd().ok())
        .map(|h| h.0 as usize);
    #[cfg(not(target_os = "windows"))]
    let hwnd: Option<usize> = None;

    let (tx, rx) = mpsc::channel::<MediaUpdate>();
    let app = app.clone();

    thread::Builder::new()
        .name("media-controls".into())
        .spawn(move || {
            let config = PlatformConfig {
                display_name: "BetterYTMusic",
                dbus_name: "betterytmusic",
                hwnd: hwnd.map(|h| h as *mut std::ffi::c_void),
            };

            let mut controls = match MediaControls::new(config) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[media] controls unavailable: {e:?}");
                    return;
                }
            };

            let handler_app = app.clone();
            if let Err(e) = controls.attach(move |event| handle_event(&handler_app, event)) {
                eprintln!("[media] attach failed: {e:?}");
                return;
            }

            // Blocks until the sender is dropped (app exit).
            for update in rx {
                let res = match update {
                    MediaUpdate::Metadata { title, artist, album, cover_url, duration_ms } => {
                        controls.set_metadata(MediaMetadata {
                            title: Some(&title),
                            artist: Some(&artist),
                            album: Some(&album),
                            cover_url: cover_url.as_deref(),
                            duration: Some(Duration::from_millis(duration_ms)),
                        })
                    }
                    MediaUpdate::Playback { playing, position_ms } => {
                        let progress = Some(MediaPosition(Duration::from_millis(position_ms)));
                        controls.set_playback(if playing {
                            MediaPlayback::Playing { progress }
                        } else {
                            MediaPlayback::Paused { progress }
                        })
                    }
                };
                if let Err(e) = res {
                    eprintln!("[media] update failed: {e:?}");
                }
            }
        })
        .ok()?;

    Some(tx)
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn handle_event(app: &AppHandle, event: MediaControlEvent) {
    let ctrl = app.state::<PlayerController>();
    match event {
        MediaControlEvent::Play => ctrl.with(app, |p| p.play()),
        MediaControlEvent::Pause | MediaControlEvent::Stop => ctrl.with(app, |p| p.pause()),
        MediaControlEvent::Toggle => ctrl.with(app, |p| p.toggle()),
        MediaControlEvent::Next => ctrl.with(app, |p| p.next()),
        MediaControlEvent::Previous => ctrl.with(app, |p| p.previous()),
        MediaControlEvent::SetPosition(MediaPosition(pos)) => {
            ctrl.with(app, |p| p.seek(pos.as_millis() as u64))
        }
        MediaControlEvent::Seek(dir) => seek_by(app, dir, SEEK_STEP_MS),
        MediaControlEvent::SeekBy(dir, d) => seek_by(app, dir, d.as_millis() as u64),
        MediaControlEvent::Raise => crate::tray::show_main_window(app),
        MediaControlEvent::Quit => app.exit(0),
        _ => {}
    }
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn seek_by(app: &AppHandle, dir: SeekDirection, step_ms: u64) {
    let ctrl = app.state::<PlayerController>();
    let pos = ctrl.snapshot().position_ms;
    let target = match dir {
        SeekDirection::Forward => pos.saturating_add(step_ms),
        SeekDirection::Backward => pos.saturating_sub(step_ms),
    };
    ctrl.with(app, |p| p.seek(target));
}

