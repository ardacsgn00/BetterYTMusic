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
    let track_artist_for_radio = track.artist.clone();
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
        let fallback_artist = track_artist_for_radio;
        thread::spawn(move || {
            let mut radio = yt.radio_queue(&video_id).unwrap_or_default();
            if radio.len() < 4 && !fallback_artist.is_empty() {
                if let Ok(extra) = yt.artist_radio(&fallback_artist, None) {
                    radio.extend(extra);
                }
            }
            if !radio.is_empty() {
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
    let artist = track.artist.clone();
    let source_title = format!("Radyo: {}", track.title);
    ctrl.with(&app, |p| {
        p.set_queue(vec![track.clone()], 0, Some(source_title));
    });
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let mut radio = yt.radio_queue(&video_id).unwrap_or_default();
        if radio.len() < 5 && !artist.is_empty() {
            if let Ok(extra) = yt.artist_radio(&artist, None) {
                radio.extend(extra);
            }
        }
        if !radio.is_empty() {
            let ctrl = app.state::<PlayerController>();
            ctrl.with(&app, |p| {
                p.append_unique(radio);
            });
        }
    });
    Ok(())
}

#[tauri::command]
pub async fn player_start_artist_radio(
    ctrl: Ctrl<'_>,
    app: AppHandle,
    yt: Yt<'_>,
    artist_name: String,
    channel_id: Option<String>,
) -> Result<usize, String> {
    let yt = yt.inner().clone();
    let name_clone = artist_name.clone();
    let cid_clone = channel_id.clone();
    let tracks = tauri::async_runtime::spawn_blocking(move || {
        yt.artist_radio(&name_clone, cid_clone.as_deref())
    })
    .await
    .map_err(|e| e.to_string())??;

    let count = tracks.len();
    if count > 0 {
        let source_title = format!("Sanatçı Radyosu: {}", artist_name.trim());
        ctrl.with(&app, |p| {
            p.set_queue(tracks, 0, Some(source_title));
        });
    }
    Ok(count)
}

#[tauri::command]
pub fn player_play_next(ctrl: Ctrl<'_>, app: AppHandle, track: Track) {
    ctrl.with(&app, |p| {
        p.play_next(track);
    });
}

#[tauri::command]
pub fn player_remove_from_queue(ctrl: Ctrl<'_>, app: AppHandle, index: usize) {
    ctrl.with(&app, |p| {
        p.remove_from_queue(index);
    });
}

#[tauri::command]
pub fn player_clear_upcoming(ctrl: Ctrl<'_>, app: AppHandle) {
    ctrl.with(&app, |p| {
        p.clear_upcoming();
    });
}

#[tauri::command]
pub fn player_shuffle_upcoming(ctrl: Ctrl<'_>, app: AppHandle) {
    ctrl.with(&app, |p| {
        p.shuffle_upcoming();
    });
}

#[tauri::command]
pub async fn player_discover_more(
    ctrl: Ctrl<'_>,
    app: AppHandle,
    yt: Yt<'_>,
) -> Result<usize, String> {
    let seed = {
        let snap = ctrl.snapshot();
        snap.track.or_else(|| snap.queue.last().cloned())
    };
    let Some(seed_track) = seed else {
        return Ok(0);
    };
    let yt = yt.inner().clone();
    let vid = seed_track.id.clone();
    let artist = seed_track.artist.clone();
    let tracks = tauri::async_runtime::spawn_blocking(move || {
        let mut pool = yt.radio_queue(&vid).unwrap_or_default();
        if pool.len() < 8 && !artist.is_empty() {
            if let Ok(extra) = yt.artist_radio(&artist, None) {
                pool.extend(extra);
            }
        }
        pool
    })
    .await
    .map_err(|e| e.to_string())?;

    let added = ctrl.with(&app, |p| p.append_unique(tracks));
    Ok(added)
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
pub async fn yt_library_playlists(yt: Yt<'_>) -> Result<Vec<PlaylistCard>, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.library_playlists())
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
pub fn settings_update(
    app: AppHandle,
    cfg: Cfg<'_>,
    ctrl: Ctrl<'_>,
    settings: AppSettings,
) -> AppSettings {
    let updated = cfg.update(settings);
    ctrl.with(&app, |p| p.set_normalization(updated.loudness_normalization));
    updated
}

#[tauri::command]
pub async fn app_check_update(yt: Yt<'_>) -> Result<UpdateCheckResult, String> {
    let yt = yt.inner().clone();
    tauri::async_runtime::spawn_blocking(move || yt.check_for_updates())
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn app_open_url(app: AppHandle, url: String) -> Result<(), String> {
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
    #[cfg(target_os = "macos")]
    spawn_detached_opener("open", trimmed)?;
    #[cfg(target_os = "android")]
    {
        let url_owned = trimmed.to_string();
        let win = app
            .get_webview_window("main")
            .or_else(|| app.webview_windows().into_values().next())
            .ok_or_else(|| "Bağlantıyı açacak pencere bulunamadı".to_string())?;
        win.with_webview(move |wv| {
            wv.jni_handle().exec(move |env, activity, _webview| {
                if let (Ok(uri_cls), Ok(intent_cls), Ok(jurl), Ok(jaction)) = (
                    env.find_class("android/net/Uri"),
                    env.find_class("android/content/Intent"),
                    env.new_string(&url_owned),
                    env.new_string("android.intent.action.VIEW"),
                ) {
                    if let Ok(uri_val) = env.call_static_method(
                        uri_cls,
                        "parse",
                        "(Ljava/lang/String;)Landroid/net/Uri;",
                        &[(&jurl).into()],
                    ) {
                        if let Ok(uri_obj) = uri_val.l() {
                            if let Ok(intent) = env.new_object(
                                intent_cls,
                                "(Ljava/lang/String;Landroid/net/Uri;)V",
                                &[(&jaction).into(), (&uri_obj).into()],
                            ) {
                                let _ = env.call_method(
                                    activity,
                                    "startActivity",
                                    "(Landroid/content/Intent;)V",
                                    &[(&intent).into()],
                                );
                            }
                        }
                    }
                }
                let _ = env.exception_clear();
            });
        })
        .map_err(|e| e.to_string())?;
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos"), not(target_os = "android")))]
    {
        let _ = &app;
        spawn_detached_opener("xdg-open", trimmed)?;
    }
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    let _ = &app;
    Ok(())
}

#[cfg(all(not(target_os = "windows"), not(target_os = "android")))]
fn spawn_detached_opener(program: &str, url: &str) -> Result<(), String> {
    std::process::Command::new(program)
        .arg(url)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("{program}: {e}"))
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
    #[cfg(not(target_os = "android"))]
    if let Some(existing) = app.get_webview_window("yt-login") {
        let _ = existing.show();
        let _ = existing.set_focus();
        return Ok(());
    }

    #[cfg(target_os = "android")]
    let label = {
        use std::sync::atomic::{AtomicU64, Ordering};
        static LOGIN_SEQ: AtomicU64 = AtomicU64::new(1);
        // Clean up any stale Android login window if the user pressed Hardware Back.
        for (existing_label, existing_win) in app.webview_windows() {
            if existing_label.starts_with("yt-login") {
                let _ = existing_win.with_webview(|wv| {
                    wv.jni_handle().exec(|env, activity, _webview| {
                        let _ = env.call_method(activity, "finish", "()V", &[]);
                        let _ = env.exception_clear();
                    });
                });
                let _ = existing_win.destroy();
            }
        }
        format!("yt-login-{}", LOGIN_SEQ.fetch_add(1, Ordering::Relaxed))
    };
    #[cfg(not(target_os = "android"))]
    let label = "yt-login".to_string();

    let login_url: tauri::Url = "https://accounts.google.com/ServiceLogin?ltmpl=music&service=youtube&continue=https%3A%2F%2Fmusic.youtube.com%2F"
        .parse()
        .map_err(|e: <tauri::Url as std::str::FromStr>::Err| e.to_string())?;

    #[cfg(target_os = "macos")]
    const AUTH_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
    #[cfg(target_os = "linux")]
    const AUTH_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
    #[cfg(target_os = "android")]
    const AUTH_USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 11; Mobile) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36";
    #[cfg(all(not(target_os = "macos"), not(target_os = "linux"), not(target_os = "android")))]
    const AUTH_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

    let app_build = app.clone();
    tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
        let mut builder = WebviewWindowBuilder::new(&app_build, label.clone(), WebviewUrl::External(login_url))
            .title("YouTube Music — Google Hesabı ile Giriş Yap");

        #[cfg(not(target_os = "android"))]
        {
            builder = builder.inner_size(520.0, 680.0).center();
        }
        #[cfg(target_os = "android")]
        {
            builder = builder.activity_name("LoginActivity");
        }

        let app_for_load = app_build.clone();
        builder
            .user_agent(AUTH_USER_AGENT)
            .on_page_load(move |_win, payload| {
                if payload.event() != PageLoadEvent::Finished {
                    return;
                }
                let host = payload.url().host_str().unwrap_or("");
                if !host.ends_with("youtube.com") {
                    return;
                }
                let app_bg = app_for_load.clone();
                let target_label = label.clone();
                // Important: never call `win.cookies_for_url(...)` synchronously inside
                // `on_page_load` (which runs on the main UI thread), because `cookies_for_url`
                // blocks waiting for a main-thread message and would deadlock.
                thread::spawn(move || {
                    for _ in 0..15 {
                        let Some(w) = app_bg.get_webview_window(&target_label) else {
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

                                #[cfg(target_os = "android")]
                                {
                                    let _ = w.with_webview(|wv| {
                                        wv.jni_handle().exec(|env, activity, _webview| {
                                            let _ = env.call_method(activity, "finish", "()V", &[]);
                                            let _ = env.exception_clear();
                                        });
                                    });
                                    let _ = w.destroy();
                                }
                                #[cfg(not(target_os = "android"))]
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
    })
    .await
    .map_err(|e| e.to_string())?
}
