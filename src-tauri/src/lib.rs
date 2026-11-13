mod commands;
mod discord;
mod innertube;
mod media;
mod miniwin;
mod mock;
mod player;
mod settings;
mod taskbar;
mod tray;

use std::sync::Arc;

use innertube::Innertube;
use player::PlayerController;
use settings::SettingsManager;
use tauri::{Manager, WindowEvent};

pub fn run() {
    if !taskbar::acquire_single_instance() {
        return;
    }

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();

            let data_dir = app.path().app_data_dir().ok();

            let yt = Arc::new(Innertube::new());
            let settings = Arc::new(SettingsManager::new());
            if let Some(dir) = &data_dir {
                yt.init_storage(dir.clone());
                settings.init(dir.clone());
            }
            app.manage(yt.clone());
            app.manage(settings.clone());
            app.manage(miniwin::MiniState::default());

            let ctrl = PlayerController::new(yt.clone(), Vec::new());
            let cur_settings = settings.get();
            ctrl.with(&handle, |p| p.set_normalization(cur_settings.loudness_normalization));
            let session_restored = if let Some(dir) = &data_dir {
                ctrl.init_session(dir.join("session.json"), cur_settings.resume_session)
            } else {
                false
            };
            app.manage(ctrl);

            tray::init(&handle)?;

            // OS media controls (SMTC / MPRIS / NowPlaying) + hardware media keys.
            let media_tx = media::spawn(&handle);
            app.state::<PlayerController>().attach_media(media_tx);

            // Discord Rich Presence
            let discord_tx = discord::spawn(&handle);
            app.state::<PlayerController>().attach_discord(discord_tx);

            player::spawn_ticker(handle.clone());

            // Populate initial queue with real YouTube Music tracks if no session was restored.
            if !session_restored {
                let init_handle = handle.clone();
                std::thread::spawn(move || {
                    if let Ok(tracks) = yt.home_tracks() {
                        if !tracks.is_empty() {
                            let ctrl = init_handle.state::<PlayerController>();
                            ctrl.with(&init_handle, |p| p.set_initial_queue(tracks));
                        }
                    }
                });
            }

            // Only show the main window if not started with `--minimized`.
            let start_hidden = std::env::args().any(|a| a == "--minimized");
            if !start_hidden {
                tray::show_main_window(&handle);
            }
            let tb = taskbar::init(&handle);
            app.state::<PlayerController>().attach_taskbar(tb);
            app.state::<PlayerController>().broadcast(&handle);
            Ok(())
        })
        .on_window_event(|_window, _event| {
            // Closing the window either hides to the tray or exits, based on settings.
            if let WindowEvent::CloseRequested { api, .. } = _event {
                if _window.label() == "main" {
                    if let Some(ctrl) = _window.app_handle().try_state::<PlayerController>() {
                        ctrl.save_session_now();
                    }
                    let close_to_tray = _window
                        .app_handle()
                        .try_state::<Arc<SettingsManager>>()
                        .map(|s| s.get().close_to_tray)
                        .unwrap_or(true);
                    if close_to_tray {
                        api.prevent_close();
                        let _ = _window.hide();
                    } else {
                        _window.app_handle().exit(0);
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::player_get_state,
            commands::player_play,
            commands::player_pause,
            commands::player_toggle,
            commands::player_next,
            commands::player_previous,
            commands::player_seek,
            commands::player_set_volume,
            commands::player_list_output_devices,
            commands::player_set_output_device,
            commands::player_play_index,
            commands::player_cycle_repeat,
            commands::player_toggle_shuffle,
            commands::player_play_track,
            commands::player_start_radio,
            commands::player_append_queue,
            commands::yt_search,
            commands::yt_search_playlists,
            commands::yt_radio,
            commands::yt_mood_categories,
            commands::yt_mood_playlists,
            commands::yt_artist_details,
            commands::yt_artist_search,
            commands::yt_home,
            commands::yt_home_playlists,
            commands::yt_playlist_tracks,
            commands::yt_lyrics,
            commands::yt_liked,
            commands::auth_get_status,
            commands::auth_login,
            commands::auth_logout,
            commands::settings_get,
            commands::settings_update,
            commands::app_check_update,
            commands::app_open_url,
            miniwin::window_enter_mini,
            miniwin::window_exit_mini,
            miniwin::window_set_mini_layout,
            miniwin::window_set_always_on_top,
            miniwin::window_start_drag,
            miniwin::window_set_fullscreen,
        ])
        .build(tauri::generate_context!())
        .expect("error while building BetterYTMusic")
        .run(|app_handle, event| {
            if let tauri::RunEvent::Exit | tauri::RunEvent::ExitRequested { .. } = event {
                if let Some(ctrl) = app_handle.try_state::<PlayerController>() {
                    ctrl.save_session_now();
                }
            }
        });
}
