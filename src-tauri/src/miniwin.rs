//! Mini player mode: the main window itself shrinks into a small, borderless,
//! always-on-top widget (no second WebView, so no extra RAM), and restores its
//! previous geometry when expanded again.

use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{
    AppHandle, LogicalSize, Manager, PhysicalPosition, PhysicalSize, Position, Size, State,
    WebviewWindow,
};

/// Must match `minWidth` / `minHeight` in `tauri.conf.json`.
const NORMAL_MIN: (f64, f64) = (380.0, 560.0);
/// Margin (logical px) from the work-area edge for the default bottom-right spot.
const EDGE_MARGIN: f64 = 16.0;

#[derive(Clone, Copy)]
struct NormalGeometry {
    pos: PhysicalPosition<i32>,
    size: PhysicalSize<u32>,
    maximized: bool,
}

#[derive(Default)]
pub struct MiniState {
    normal: Mutex<Option<NormalGeometry>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MiniPos {
    pub x: i32,
    pub y: i32,
}

fn main_window(app: &AppHandle) -> Result<WebviewWindow, String> {
    app.get_webview_window("main")
        .ok_or_else(|| "Ana pencere bulunamadı".to_string())
}

fn layout_size(layout: &str) -> (f64, f64) {
    match layout {
        "square" => (300.0, 300.0),
        _ => (380.0, 108.0),
    }
}

/// Default position: bottom-right corner of the current monitor's work area.
fn default_position(win: &WebviewWindow, w: f64, h: f64) -> Option<PhysicalPosition<i32>> {
    let mon = win.current_monitor().ok().flatten()?;
    let scale = mon.scale_factor();
    let area = mon.work_area();
    let x = area.position.x + area.size.width as i32 - ((w + EDGE_MARGIN) * scale) as i32;
    let y = area.position.y + area.size.height as i32 - ((h + EDGE_MARGIN) * scale) as i32;
    Some(PhysicalPosition::new(x, y))
}

/// Keep a saved position on a currently connected monitor (e.g. after unplugging a screen).
fn position_visible(win: &WebviewWindow, pos: MiniPos) -> bool {
    win.available_monitors()
        .map(|mons| {
            mons.iter().any(|m| {
                let p = m.position();
                let s = m.size();
                pos.x >= p.x - 40
                    && pos.y >= p.y - 40
                    && pos.x < p.x + s.width as i32 - 60
                    && pos.y < p.y + s.height as i32 - 60
            })
        })
        .unwrap_or(false)
}

#[tauri::command]
pub fn window_enter_mini(
    app: AppHandle,
    state: State<'_, MiniState>,
    layout: String,
    pos: Option<MiniPos>,
) -> Result<(), String> {
    let win = main_window(&app)?;

    {
        let mut normal = state.normal.lock().map_err(|e| e.to_string())?;
        if normal.is_none() {
            let maximized = win.is_maximized().unwrap_or(false);
            if maximized {
                let _ = win.unmaximize();
            }
            *normal = Some(NormalGeometry {
                pos: win.outer_position().map_err(|e| e.to_string())?,
                size: win.inner_size().map_err(|e| e.to_string())?,
                maximized,
            });
        }
    }

    let (w, h) = layout_size(&layout);
    let _ = win.set_fullscreen(false);
    win.set_decorations(false).map_err(|e| e.to_string())?;
    win.set_min_size(None::<Size>).map_err(|e| e.to_string())?;
    win.set_resizable(false).map_err(|e| e.to_string())?;
    win.set_size(Size::Logical(LogicalSize::new(w, h)))
        .map_err(|e| e.to_string())?;

    let target = pos
        .filter(|p| position_visible(&win, *p))
        .map(|p| PhysicalPosition::new(p.x, p.y))
        .or_else(|| default_position(&win, w, h));
    if let Some(p) = target {
        win.set_position(Position::Physical(p)).map_err(|e| e.to_string())?;
    }

    win.set_always_on_top(true).map_err(|e| e.to_string())?;
    let _ = win.show();
    let _ = win.set_focus();
    Ok(())
}

/// Resize the mini window when switching layouts while already in mini mode,
/// keeping the bottom-right corner anchored.
#[tauri::command]
pub fn window_set_mini_layout(app: AppHandle, layout: String) -> Result<(), String> {
    let win = main_window(&app)?;
    let scale = win.scale_factor().unwrap_or(1.0);
    let old_pos = win.outer_position().map_err(|e| e.to_string())?;
    let old_size = win.outer_size().map_err(|e| e.to_string())?;
    let (w, h) = layout_size(&layout);
    win.set_size(Size::Logical(LogicalSize::new(w, h)))
        .map_err(|e| e.to_string())?;
    let right = old_pos.x + old_size.width as i32;
    let bottom = old_pos.y + old_size.height as i32;
    let nx = right - (w * scale) as i32;
    let ny = bottom - (h * scale) as i32;
    win.set_position(Position::Physical(PhysicalPosition::new(nx, ny)))
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Restore the normal window. Returns the mini window's last position so the
/// frontend can remember it for next time.
#[tauri::command]
pub fn window_exit_mini(
    app: AppHandle,
    state: State<'_, MiniState>,
) -> Result<Option<MiniPos>, String> {
    let win = main_window(&app)?;
    let last = win
        .outer_position()
        .ok()
        .map(|p| MiniPos { x: p.x, y: p.y });

    let normal = state.normal.lock().map_err(|e| e.to_string())?.take();

    win.set_always_on_top(false).map_err(|e| e.to_string())?;
    win.set_resizable(true).map_err(|e| e.to_string())?;
    win.set_decorations(true).map_err(|e| e.to_string())?;
    win.set_min_size(Some(Size::Logical(LogicalSize::new(NORMAL_MIN.0, NORMAL_MIN.1))))
        .map_err(|e| e.to_string())?;

    match normal {
        Some(g) => {
            win.set_size(Size::Physical(g.size)).map_err(|e| e.to_string())?;
            win.set_position(Position::Physical(g.pos))
                .map_err(|e| e.to_string())?;
            if g.maximized {
                let _ = win.maximize();
            }
        }
        None => {
            win.set_size(Size::Logical(LogicalSize::new(1280.0, 800.0)))
                .map_err(|e| e.to_string())?;
            let _ = win.center();
        }
    }
    let _ = win.show();
    let _ = win.set_focus();
    Ok(last)
}

#[tauri::command]
pub fn window_set_always_on_top(app: AppHandle, on: bool) -> Result<(), String> {
    main_window(&app)?
        .set_always_on_top(on)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn window_start_drag(app: AppHandle) -> Result<(), String> {
    main_window(&app)?
        .start_dragging()
        .map_err(|e| e.to_string())
}

