use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

use crate::player::PlayerController;

pub fn show_main_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
    }
}

pub fn init(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Pencereyi göster", true, None::<&str>)?;
    let toggle = MenuItem::with_id(app, "toggle", "Oynat / Duraklat", true, None::<&str>)?;
    let next = MenuItem::with_id(app, "next", "Sonraki", true, None::<&str>)?;
    let prev = MenuItem::with_id(app, "prev", "Önceki", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Çıkış", true, None::<&str>)?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let sep2 = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(app, &[&show, &sep1, &toggle, &prev, &next, &sep2, &quit])?;

    let mut builder = TrayIconBuilder::with_id("main")
        .tooltip("BetterYTMusic")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            let ctrl = app.state::<PlayerController>();
            match event.id().as_ref() {
                "show" => show_main_window(app),
                "toggle" => ctrl.with(app, |p| p.toggle()),
                "next" => ctrl.with(app, |p| p.next()),
                "prev" => ctrl.with(app, |p| p.previous()),
                "quit" => app.exit(0),
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;
    Ok(())
}

