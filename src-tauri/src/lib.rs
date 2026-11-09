mod commands;
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
use tauri::Manager;
#[cfg(desktop)]
use tauri::WindowEvent;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("[RUST PANIC] {info}");
        #[cfg(target_os = "android")]
        {
            use std::ffi::{c_char, CString};
            let msg = format!("BetterYTMusic PANIC: {info}\0");
            if let (Ok(tag), Ok(text)) = (CString::new("BetterYTMusic"), CString::new(msg)) {
                unsafe {
                    extern "C" {
                        fn __android_log_write(prio: i32, tag: *const c_char, text: *const c_char) -> i32;
                    }
                    __android_log_write(6, tag.as_ptr(), text.as_ptr());
                }
            }
        }
    }));

    #[cfg(desktop)]
    if !taskbar::acquire_single_instance() {
        return;
    }

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();

            let data_dir = app.path().app_data_dir().ok();

            let yt = Arc::new(Innertube::new());
            let settings = Arc::new(SettingsManager::new());
            if let Some(dir) = data_dir {
                yt.init_storage(dir.clone());
                settings.init(dir);
            }
            app.manage(yt.clone());
            app.manage(settings.clone());
            app.manage(miniwin::MiniState::default());

            app.manage(PlayerController::new(yt.clone(), Vec::new()));

            tray::init(&handle)?;

            // OS media controls (SMTC / MPRIS / NowPlaying) + hardware media keys.
            let media_tx = media::spawn(&handle);
            app.state::<PlayerController>().attach_media(media_tx);

            player::spawn_ticker(handle.clone());

            // Populate initial queue with real YouTube Music tracks in the background.
            let init_handle = handle.clone();
            std::thread::spawn(move || {
                if let Ok(tracks) = yt.home_tracks() {
                    if !tracks.is_empty() {
                        let ctrl = init_handle.state::<PlayerController>();
                        ctrl.with(&init_handle, |p| p.set_initial_queue(tracks));
                    }
                }
            });

            #[cfg(desktop)]
            {
                // Only show the main window if not started with `--minimized`.
                let start_hidden = std::env::args().any(|a| a == "--minimized");
                if !start_hidden {
                    tray::show_main_window(&handle);
                }
            }
            let tb = taskbar::init(&handle);
            app.state::<PlayerController>().attach_taskbar(tb);
            app.state::<PlayerController>().broadcast(&handle);
            Ok(())
        })
        .on_window_event(|_window, _event| {
            #[cfg(desktop)]
            {
                // Closing the window either hides to the tray or exits, based on settings.
                if let WindowEvent::CloseRequested { api, .. } = _event {
                    if _window.label() == "main" {
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running BetterYTMusic");
}
