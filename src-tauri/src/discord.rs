//! Discord Rich Presence integration via `discord-rich-presence`.

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use tauri::{AppHandle, Manager};

use crate::mock::Track;
use crate::settings::SettingsManager;

#[derive(Debug, Clone)]
pub struct DiscordUpdate {
    pub track: Option<Track>,
    pub is_playing: bool,
    pub position_ms: u64,
}

pub type DiscordSender = Sender<DiscordUpdate>;

fn sanitize_str(s: &str, fallback: &str) -> String {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        fallback.to_string()
    } else if trimmed.chars().count() < 2 {
        format!("{trimmed} ")
    } else {
        trimmed.chars().take(128).collect()
    }
}

pub fn spawn(app: &AppHandle) -> Option<DiscordSender> {
    let (tx, rx) = mpsc::channel::<DiscordUpdate>();
    let handle = app.clone();

    thread::Builder::new()
        .name("discord-rpc".into())
        .spawn(move || run_loop(handle, rx))
        .ok()?;

    Some(tx)
}

fn run_loop(app: AppHandle, rx: Receiver<DiscordUpdate>) {
    let mut client: Option<DiscordIpcClient> = None;
    let mut last_connect_attempt = Instant::now() - Duration::from_secs(10);
    let mut current_client_id = String::new();
    let mut last_update: Option<DiscordUpdate> = None;
    let mut last_send = Instant::now() - Duration::from_secs(5);

    loop {
        // Fetch current settings
        let (enabled, client_id, show_cover, show_time, show_button, clear_on_pause) = {
            if let Some(mgr) = app.try_state::<Arc<SettingsManager>>() {
                let s = mgr.get();
                (
                    s.discord_rpc,
                    s.discord_client_id.trim().to_string(),
                    s.discord_show_cover,
                    s.discord_show_time,
                    s.discord_show_button,
                    s.discord_clear_on_pause,
                )
            } else {
                (true, "1569629580694655006".to_string(), true, true, true, false)
            }
        };

        // If client ID changed while connected, close client to re-connect with new ID
        if !current_client_id.is_empty() && current_client_id != client_id {
            if let Some(mut c) = client.take() {
                let _ = c.clear_activity();
                let _ = c.close();
            }
        }
        current_client_id = client_id.clone();

        // If disabled, disconnect if currently connected
        if !enabled || current_client_id.is_empty() {
            if let Some(mut c) = client.take() {
                let _ = c.clear_activity();
                let _ = c.close();
            }
            // Sleep a bit before checking channel or settings
            match rx.recv_timeout(Duration::from_millis(500)) {
                Ok(upd) => last_update = Some(upd),
                Err(_) => {}
            }
            continue;
        }

        // Connect if not connected (throttle connection attempts to every 5s)
        if client.is_none() {
            if last_connect_attempt.elapsed() >= Duration::from_secs(5) {
                last_connect_attempt = Instant::now();
                let mut c = DiscordIpcClient::new(&current_client_id);
                match c.connect() {
                    Ok(()) => {
                        client = Some(c);
                        // Force re-send activity on fresh connection
                        last_send = Instant::now() - Duration::from_secs(5);
                    }
                    Err(e) => {
                        // Discord not running or connection refused; silently retry later
                        let _ = e;
                    }
                }
            }
        }

        // Wait for update from channel or tick
        let has_new_update = match rx.recv_timeout(Duration::from_millis(400)) {
            Ok(upd) => {
                last_update = Some(upd);
                true
            }
            Err(mpsc::RecvTimeoutError::Timeout) => false,
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                // App shutting down
                if let Some(mut c) = client.take() {
                    let _ = c.clear_activity();
                    let _ = c.close();
                }
                break;
            }
        };

        let Some(c) = client.as_mut() else {
            continue;
        };

        let Some(upd) = &last_update else {
            continue;
        };

        // Throttle updates to at most once per 600ms to avoid Discord rate-limits
        if !has_new_update && last_send.elapsed() < Duration::from_secs(5) {
            continue;
        }
        if last_send.elapsed() < Duration::from_millis(600) {
            continue;
        }

        let Some(track) = &upd.track else {
            let _ = c.clear_activity();
            last_send = Instant::now();
            continue;
        };

        if !upd.is_playing && clear_on_pause {
            let _ = c.clear_activity();
            last_send = Instant::now();
            continue;
        }

        let title = sanitize_str(&track.title, "Müzik");
        let artist = sanitize_str(&track.artist, "Bilinmeyen Sanatçı");
        let album = if track.album.trim().is_empty() {
            title.clone()
        } else {
            sanitize_str(&track.album, &title)
        };

        let mut act = activity::Activity::new();
        act = act.details(&title);

        let now_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        let pos_sec = (upd.position_ms / 1000) as i64;
        let dur_sec = (track.duration_ms / 1000) as i64;

        let paused_state = format!("Duraklatıldı • {artist}");
        let safe_paused_state = sanitize_str(&paused_state, &artist);

        if upd.is_playing {
            act = act.state(&artist);
            if show_time {
                let start_ts = now_sec - pos_sec;
                let mut ts = activity::Timestamps::new().start(start_ts);
                if dur_sec > pos_sec && dur_sec > 0 {
                    ts = ts.end(start_ts + dur_sec);
                }
                act = act.timestamps(ts);
            }
        } else {
            act = act.state(&safe_paused_state);
        }

        let mut assets = activity::Assets::new();
        if show_cover {
            if let Some(thumb) = &track.thumbnail {
                if thumb.starts_with("http://") || thumb.starts_with("https://") {
                    assets = assets.large_image(thumb);
                }
            }
        }
        assets = assets.large_text(&album);
        act = act.assets(assets);

        let yt_url = if show_button && !track.id.is_empty() {
            format!("https://music.youtube.com/watch?v={}", track.id)
        } else {
            String::new()
        };

        if !yt_url.is_empty() {
            let buttons = vec![activity::Button::new("YouTube Music'te Aç", &yt_url)];
            act = act.buttons(buttons);
        }

        match c.set_activity(act) {
            Ok(_) => {
                last_send = Instant::now();
            }
            Err(e) => {
                eprintln!("[discord-rpc] set_activity failed: {e:?}");
                // Discord may have been closed
                let _ = c.close();
                client = None;
                last_connect_attempt = Instant::now();
            }
        }
    }
}
