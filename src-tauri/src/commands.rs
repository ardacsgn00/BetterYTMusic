//! IPC commands exposed to the frontend. Thin wrappers over `PlayerController` and `Innertube`.

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use tauri::webview::PageLoadEvent;
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};

use crate::innertube::{
    ArtistCard, ArtistData, AuthStatus, Innertube, LyricsData, MoodCategory, PlaylistCard,
    UpdateCheckResult,
};
use crate::mock::Track;
use crate::player::{PlayerController, PlayerSnapshot};
use crate::settings::{AppSettings, SettingsManager};

type Ctrl<'a> = State<'a, PlayerController>;
type Yt<'a> = State<'a, Arc<Innertube>>;
type Cfg<'a> = State<'a, Arc<SettingsManager>>;

#[tauri::command]
pub fn player_get_state(ctrl: Ctrl<'_>) -> PlayerSnapshot {
    ctrl.snapshot()
}

#[tauri::command]
pub fn player_play(app: AppHandle, ctrl: Ctrl<'_>) {
    ctrl.with(&app, |p| p.play());
}

#[tauri::command]
pub fn player_pause(app: AppHandle, ctrl: Ctrl<'_>) {
    ctrl.with(&app, |p| p.pause());
}

#[tauri::command]
pub fn player_toggle(app: AppHandle, ctrl: Ctrl<'_>) {
    ctrl.with(&app, |p| p.toggle());
}

#[tauri::command]
pub fn player_next(app: AppHandle, ctrl: Ctrl<'_>) {
    ctrl.with(&app, |p| p.next());
}

#[tauri::command]
pub fn player_previous(app: AppHandle, ctrl: Ctrl<'_>) {
    ctrl.with(&app, |p| p.previous());
}

#[tauri::command]
pub fn player_seek(app: AppHandle, ctrl: Ctrl<'_>, position_ms: u64) {
    ctrl.with(&app, |p| p.seek(position_ms));
}

#[tauri::command]
pub fn player_set_volume(app: AppHandle, ctrl: Ctrl<'_>, volume: f32) {
    ctrl.with(&app, |p| p.set_volume(volume));
}

#[tauri::command]
pub fn player_list_output_devices() -> Vec<String> {
    crate::player::engine::list_output_devices()
}

#[tauri::command]
pub fn player_set_output_device(app: AppHandle, ctrl: Ctrl<'_>, device: Option<String>) {
    ctrl.with(&app, |p| p.set_output_device(device));
}

#[tauri::command]
pub fn player_play_index(app: AppHandle, ctrl: Ctrl<'_>, index: usize) {
    ctrl.with(&app, |p| p.play_index(index));
}

#[tauri::command]
pub fn player_cycle_repeat(app: AppHandle, ctrl: Ctrl<'_>) {
    ctrl.with(&app, |p| p.cycle_repeat());
}

#[tauri::command]
pub fn player_toggle_shuffle(app: AppHandle, ctrl: Ctrl<'_>) {
    ctrl.with(&app, |p| p.toggle_shuffle());
}

#[tauri::command]
pub fn player_play_track(
    app: AppHandle,
    ctrl: Ctrl<'_>,
    yt: Yt<'_>,
    track: Track,
    queue: Option<Vec<Track>>,
    source: Option<String>,
) {
    let video_id = track.id.clone();
    let should_fetch_radio = queue.is_none() && !video_id.starts_with("mock-");
    let resolved_source = source.or_else(|| {
        if should_fetch_radio {
            Some(format!("{} Radyosu", track.title))
        } else {
            None
        }
    });

    ctrl.with(&app, |p| {
        if let Some(q) = queue {
            let idx = q.iter().position(|t| t.id == track.id).unwrap_or(0);
            p.set_queue(q, idx, resolved_source);
        } else {
            p.set_queue(vec![track], 0, resolved_source);
        }
    });

    if should_fetch_radio {
        let yt = yt.inner().clone();
        thread::spawn(move || {
            if let Ok(radio) = yt.radio_queue(&video_id) {
                let ctrl = app.state::<PlayerController>();
                ctrl.with(&app, |p| p.append_unique(radio));
            }
        });
    }
}

#[tauri::command]
pub async fn player_start_radio(
    ctrl: Ctrl<'_>,
    app: AppHandle,
    yt: Yt<'_>,
    track: Track,
) -> Result<(), String> {
    let video_id = track.id.clone();
    let source_title = format!("Radyo: {}", track.title);
    ctrl.with(&app, |p| {
        p.set_queue(vec![track.clone()], 0, Some(source_title));
    });
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn(async move {
        if let Ok(radio) = yt.radio_queue(&video_id) {
            let ctrl = app.state::<PlayerController>();
            ctrl.with(&app, |p| {
                p.append_unique(radio);
            });
        }
    });
    Ok(())
}

#[tauri::command]
pub fn player_append_queue(ctrl: Ctrl<'_>, app: AppHandle, tracks: Vec<Track>) {
    ctrl.with(&app, |p| {
        p.append_unique(tracks);
    });
}

#[tauri::command]
pub async fn yt_radio(yt: Yt<'_>, video_id: String) -> Result<Vec<Track>, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.radio_queue(&video_id))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn yt_mood_categories(yt: Yt<'_>) -> Result<Vec<MoodCategory>, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.mood_categories())
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn yt_mood_playlists(yt: Yt<'_>, params: String) -> Result<Vec<PlaylistCard>, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.mood_playlists(&params))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn yt_artist_details(yt: Yt<'_>, browse_id: String) -> Result<ArtistData, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.artist_details(&browse_id))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn yt_artist_search(yt: Yt<'_>, query: String) -> Result<Option<ArtistCard>, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.search_artist(&query))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn yt_search(yt: Yt<'_>, query: String) -> Result<Vec<Track>, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.search(&query))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn yt_search_playlists(yt: Yt<'_>, query: String) -> Result<Vec<PlaylistCard>, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.search_playlists(&query))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn yt_home(yt: Yt<'_>) -> Result<Vec<Track>, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.home_tracks())
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn yt_liked(yt: Yt<'_>) -> Result<Vec<Track>, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.liked_tracks())
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn yt_home_playlists(yt: Yt<'_>) -> Result<Vec<PlaylistCard>, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.home_playlists())
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn yt_playlist_tracks(yt: Yt<'_>, playlist_id: String) -> Result<Vec<Track>, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.playlist_tracks(&playlist_id))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn yt_lyrics(
    yt: Yt<'_>,
    video_id: String,
    title: String,
    artist: String,
    duration_ms: u64,
) -> Result<Option<LyricsData>, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        yt.fetch_lyrics(&video_id, &title, &artist, duration_ms)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn settings_get(cfg: Cfg<'_>) -> AppSettings {
    cfg.get()
}

#[tauri::command]
pub fn settings_update(cfg: Cfg<'_>, settings: AppSettings) -> AppSettings {
    cfg.update(settings)
}

#[tauri::command]
pub async fn app_check_update(yt: Yt<'_>) -> Result<UpdateCheckResult, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.check_for_updates())
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn app_open_url(url: String) -> Result<(), String> {
    let trimmed = url.trim();
    if !trimmed.starts_with("https://") && !trimmed.starts_with("http://") {
        return Err("Geçersiz bağlantı adresi".to_string());
    }
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        std::process::Command::new("rundll32.exe")
            .args(["url.dll,FileProtocolHandler", trimmed])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = std::process::Command::new("xdg-open").arg(trimmed).spawn();
    }
    Ok(())
}

#[tauri::command]
pub fn auth_get_status(yt: Yt<'_>) -> AuthStatus {
    yt.auth_status()
}

#[tauri::command]
pub fn auth_logout(app: AppHandle, yt: Yt<'_>) -> AuthStatus {
    let status = yt.clear_auth();
    let _ = app.emit("auth:changed", &status);
    status
}

#[tauri::command]
pub async fn auth_login(app: AppHandle) -> Result<(), String> {
    if let Some(existing) = app.get_webview_window("yt-login") {
        let _ = existing.show();
        let _ = existing.set_focus();
        return Ok(());
    }

    let login_url: tauri::Url = "https://accounts.google.com/ServiceLogin?ltmpl=music&service=youtube&continue=https%3A%2F%2Fmusic.youtube.com%2F"
        .parse()
        .map_err(|e: <tauri::Url as std::str::FromStr>::Err| e.to_string())?;

    let builder = WebviewWindowBuilder::new(&app, "yt-login", WebviewUrl::External(login_url))
        .title("YouTube Music — Google Hesabı ile Giriş Yap")
        .inner_size(520.0, 680.0)
        .center();

    let app_for_load = app.clone();
    builder
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .on_page_load(move |_win, payload| {
            if payload.event() != PageLoadEvent::Finished {
                return;
            }
            let host = payload.url().host_str().unwrap_or("");
            if !host.ends_with("youtube.com") {
                return;
            }
            let app_bg = app_for_load.clone();
            // Important: never call `win.cookies_for_url(...)` synchronously inside
            // `on_page_load` (which runs on the main UI thread), because `cookies_for_url`
            // blocks waiting for a main-thread message and would deadlock.
            thread::spawn(move || {
                for _ in 0..10 {
                    let Some(w) = app_bg.get_webview_window("yt-login") else {
                        return;
                    };
                    let Ok(target_url) = "https://music.youtube.com/".parse::<tauri::Url>() else {
                        return;
                    };
                    if let Ok(cookies) = w.cookies_for_url(target_url) {
                        let sapisid = cookies
                            .iter()
                            .find(|c| {
                                c.name() == "__Secure-3PAPISID"
                                    || c.name() == "SAPISID"
                                    || c.name() == "__Secure-1PAPISID"
                            })
                            .map(|c| c.value().to_string());
                        let has_sid = cookies.iter().any(|c| {
                            c.name() == "SID"
                                || c.name() == "__Secure-3PSID"
                                || c.name() == "__Secure-1PSID"
                        });

                        if let (Some(sapisid_val), true) = (sapisid, has_sid) {
                            let cookie_header = cookies
                                .iter()
                                .map(|c| format!("{}={}", c.name(), c.value()))
                                .collect::<Vec<_>>()
                                .join("; ");

                            let _ = w.close();
                            let yt = app_bg.state::<Arc<Innertube>>();
                            let status = yt.set_cookies(cookie_header, sapisid_val);
                            let _ = app_bg.emit("auth:changed", &status);
                            if let Ok(tracks) = yt.home_tracks() {
                                if !tracks.is_empty() {
                                    let ctrl = app_bg.state::<PlayerController>();
                                    ctrl.with(&app_bg, |p| p.set_initial_queue(tracks));
                                }
                            }
                            return;
                        }
                    }
                    thread::sleep(Duration::from_millis(500));
                }
            });
        })
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}
