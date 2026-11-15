//! Windows Taskbar Thumbnail Toolbar (`ITaskbarList3`) integration.
//!
//! Displays Spotify-style transport buttons (Previous, Play/Pause, Next) directly
//! inside the Windows taskbar hover thumbnail preview.

#[cfg(all(not(target_os = "windows"), not(target_os = "linux")))]
use tauri::AppHandle;

#[cfg(target_os = "windows")]
mod imp {
    use std::mem::{size_of, zeroed};
    use std::ptr::null_mut;

    use tauri::{AppHandle, Manager};
    use windows::core::w;
    use windows::Win32::Foundation::{ERROR_ALREADY_EXISTS, GetLastError, HWND, LPARAM, LRESULT, WPARAM};
    use windows::Win32::Graphics::Gdi::{
        CreateBitmap, CreateDIBSection, DeleteObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
        DIB_RGB_COLORS, HBITMAP, HDC,
    };
    use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER};
    use windows::Win32::System::Threading::CreateMutexW;
    use windows::Win32::UI::Shell::{
        DefSubclassProc, ITaskbarList3, SetWindowSubclass, TaskbarList, THBF_ENABLED, THB_FLAGS,
        THB_ICON, THB_TOOLTIP, THBN_CLICKED, THUMBBUTTON,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        CreateIconIndirect, PostMessageW, RegisterWindowMessageW, HICON, HWND_BROADCAST, ICONINFO,
        WM_APP, WM_COMMAND,
    };

    use crate::player::PlayerController;

    const BTN_PREV: u32 = 1001;
    const BTN_PLAY_PAUSE: u32 = 1002;
    const BTN_NEXT: u32 = 1003;

    const WM_TASKBAR_SYNC: u32 = WM_APP + 77;
    const SUBCLASS_ID: usize = 0xB77E_2026;

    /// Ensures WebView2 stores its user data inside `%LOCALAPPDATA%\com.betterytmusic.app\EBWebView`
    /// (never creating a `.exe.WebView2` folder next to the executable on Desktop/Downloads)
    /// and prevents multiple instances from opening when the `.exe` is clicked repeatedly.
    pub fn acquire_single_instance() -> bool {
        if std::env::var_os("WEBVIEW2_USER_DATA_FOLDER").is_none() {
            if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
                let wv_dir = std::path::PathBuf::from(local_app_data)
                    .join("com.betterytmusic.app")
                    .join("EBWebView");
                let _ = std::fs::create_dir_all(&wv_dir);
                std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &wv_dir);
            }
        }

        unsafe {
            let _ = CreateMutexW(None, false, w!("Local\\BetterYTMusic_SingleInstance_Mutex_v1"));
            if GetLastError() == ERROR_ALREADY_EXISTS {
                let msg_show = RegisterWindowMessageW(w!("BetterYTMusic_ShowMainWindow"));
                if msg_show != 0 {
                    let _ = PostMessageW(Some(HWND_BROADCAST), msg_show, WPARAM(0), LPARAM(0));
                }
                return false;
            }
        }
        true
    }

    struct TaskbarContext {
        app: AppHandle,
        taskbar: Option<ITaskbarList3>,
        icon_prev: HICON,
        icon_play: HICON,
        icon_pause: HICON,
        icon_next: HICON,
        msg_btn_created: u32,
        msg_show_main: u32,
        added: bool,
        playing: bool,
    }

    #[derive(Clone, Copy)]
    pub struct TaskbarHandle {
        hwnd: usize,
    }

    impl TaskbarHandle {
        pub fn set_playing(&self, playing: bool) {
            unsafe {
                let _ = PostMessageW(
                    Some(HWND(self.hwnd as *mut _)),
                    WM_TASKBAR_SYNC,
                    WPARAM(usize::from(playing)),
                    LPARAM(0),
                );
            }
        }
    }

    pub fn init(app: &AppHandle) -> Option<TaskbarHandle> {
        let win = app.get_webview_window("main")?;
        let raw_hwnd = win.hwnd().ok()?.0 as usize;
        let hwnd = HWND(raw_hwnd as *mut _);

        unsafe {
            let msg_btn_created = RegisterWindowMessageW(w!("TaskbarButtonCreated"));
            let msg_show_main = RegisterWindowMessageW(w!("BetterYTMusic_ShowMainWindow"));
            let taskbar: Option<ITaskbarList3> =
                CoCreateInstance(&TaskbarList, None, CLSCTX_INPROC_SERVER).ok();
            if let Some(tb) = &taskbar {
                let _ = tb.HrInit();
            }

            let mut ctx = Box::new(TaskbarContext {
                app: app.clone(),
                taskbar,
                icon_prev: create_transport_icon(IconKind::Prev),
                icon_play: create_transport_icon(IconKind::Play),
                icon_pause: create_transport_icon(IconKind::Pause),
                icon_next: create_transport_icon(IconKind::Next),
                msg_btn_created,
                msg_show_main,
                added: false,
                playing: false,
            });

            ensure_buttons(hwnd, &mut ctx);

            let ctx_ptr = Box::into_raw(ctx) as usize;
            let _ = SetWindowSubclass(hwnd, Some(subclass_proc), SUBCLASS_ID, ctx_ptr);
        }

        Some(TaskbarHandle { hwnd: raw_hwnd })
    }

    unsafe fn ensure_buttons(hwnd: HWND, ctx: &mut TaskbarContext) {
        if ctx.taskbar.is_none() {
            ctx.taskbar = CoCreateInstance(&TaskbarList, None, CLSCTX_INPROC_SERVER).ok();
            if let Some(tb) = &ctx.taskbar {
                let _ = tb.HrInit();
            }
        }
        let Some(tb) = &ctx.taskbar else { return };

        let buttons = [
            make_button(BTN_PREV, ctx.icon_prev, "Önceki"),
            make_button(
                BTN_PLAY_PAUSE,
                if ctx.playing {
                    ctx.icon_pause
                } else {
                    ctx.icon_play
                },
                if ctx.playing { "Duraklat" } else { "Oynat" },
            ),
            make_button(BTN_NEXT, ctx.icon_next, "Sonraki"),
        ];

        if !ctx.added {
            if tb.ThumbBarAddButtons(hwnd, &buttons).is_ok() {
                ctx.added = true;
            }
        } else {
            let _ = tb.ThumbBarUpdateButtons(hwnd, &buttons);
        }
    }

    fn make_button(id: u32, icon: HICON, tip: &str) -> THUMBBUTTON {
        let mut btn: THUMBBUTTON = unsafe { zeroed() };
        btn.dwMask = THB_ICON | THB_TOOLTIP | THB_FLAGS;
        btn.iId = id;
        btn.hIcon = icon;
        btn.dwFlags = THBF_ENABLED;
        for (i, ch) in tip.encode_utf16().take(259).enumerate() {
            btn.szTip[i] = ch;
        }
        btn
    }

    unsafe extern "system" fn subclass_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        _uid: usize,
        ref_data: usize,
    ) -> LRESULT {
        if ref_data != 0 {
            let ctx = &mut *(ref_data as *mut TaskbarContext);

            if msg != 0 && msg == ctx.msg_btn_created {
                ctx.added = false;
                ensure_buttons(hwnd, ctx);
            } else if msg != 0 && msg == ctx.msg_show_main {
                crate::tray::show_main_window(&ctx.app);
                return LRESULT(0);
            } else if msg == WM_TASKBAR_SYNC {
                let new_playing = wparam.0 != 0;
                if !ctx.added || ctx.playing != new_playing {
                    ctx.playing = new_playing;
                    ensure_buttons(hwnd, ctx);
                }
                return LRESULT(0);
            } else if msg == WM_COMMAND {
                let notify_code = ((wparam.0 >> 16) & 0xFFFF) as u32;
                let btn_id = (wparam.0 & 0xFFFF) as u32;
                if notify_code == THBN_CLICKED {
                    let app = ctx.app.clone();
                    let ctrl = app.state::<PlayerController>();
                    match btn_id {
                        BTN_PREV => ctrl.with(&app, |p| p.previous()),
                        BTN_PLAY_PAUSE => ctrl.with(&app, |p| p.toggle()),
                        BTN_NEXT => ctrl.with(&app, |p| p.next()),
                        _ => {}
                    }
                    return LRESULT(0);
                }
            }
        }

        DefSubclassProc(hwnd, msg, wparam, lparam)
    }

    #[derive(Clone, Copy)]
    enum IconKind {
        Prev,
        Play,
        Pause,
        Next,
    }

    /// Renders a crisp 32x32 32-bit ARGB white transport icon.
    unsafe fn create_transport_icon(kind: IconKind) -> HICON {
        const SIZE: i32 = 32;
        let mut bi: BITMAPINFO = zeroed();
        bi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
        bi.bmiHeader.biWidth = SIZE;
        bi.bmiHeader.biHeight = -SIZE; // top-down
        bi.bmiHeader.biPlanes = 1;
        bi.bmiHeader.biBitCount = 32;
        bi.bmiHeader.biCompression = BI_RGB.0;

        let mut bits: *mut core::ffi::c_void = null_mut();
        let Ok(hbm_color) =
            CreateDIBSection(Some(HDC(null_mut())), &bi, DIB_RGB_COLORS, &mut bits, None, 0)
        else {
            return HICON(null_mut());
        };

        if !bits.is_null() {
            let pixels = std::slice::from_raw_parts_mut(bits as *mut u32, (SIZE * SIZE) as usize);
            for y in 0..SIZE {
                for x in 0..SIZE {
                    let fx = x as f32 + 0.5;
                    let fy = y as f32 + 0.5;
                    let inside = match kind {
                        IconKind::Play => {
                            // Right-pointing triangle: x in [9..25], y centered at 16
                            if (9.0..=25.0).contains(&fx) {
                                let t = (25.0 - fx) / 16.0;
                                let half_h = 9.5 * t;
                                (fy - 16.0).abs() <= half_h
                            } else {
                                false
                            }
                        }
                        IconKind::Pause => {
                            // Two vertical bars: x in [8..13.5] and [18.5..24], y in [7..25]
                            (7.0..=25.0).contains(&fy)
                                && ((8.0..=13.5).contains(&fx) || (18.5..=24.0).contains(&fx))
                        }
                        IconKind::Prev => {
                            // Left bar [6.5..9.5, 8..24] + left-pointing triangle [10.5..24.5]
                            let bar = (8.0..=24.0).contains(&fy) && (6.5..=9.5).contains(&fx);
                            let tri = if (10.5..=24.5).contains(&fx) {
                                let t = (fx - 10.5) / 14.0;
                                (fy - 16.0).abs() <= 8.5 * t
                            } else {
                                false
                            };
                            bar || tri
                        }
                        IconKind::Next => {
                            // Right-pointing triangle [7.5..21.5] + right bar [22.5..25.5, 8..24]
                            let bar = (8.0..=24.0).contains(&fy) && (22.5..=25.5).contains(&fx);
                            let tri = if (7.5..=21.5).contains(&fx) {
                                let t = (21.5 - fx) / 14.0;
                                (fy - 16.0).abs() <= 8.5 * t
                            } else {
                                false
                            };
                            bar || tri
                        }
                    };
                    pixels[(y * SIZE + x) as usize] = if inside { 0xFFFF_FFFF } else { 0x0000_0000 };
                }
            }
        }

        let mask_bytes = [0u8; 128];
        let hbm_mask: HBITMAP = CreateBitmap(SIZE, SIZE, 1, 1, Some(mask_bytes.as_ptr() as *const _));

        let icon_info = ICONINFO {
            fIcon: true.into(),
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: hbm_mask,
            hbmColor: hbm_color,
        };

        let hicon = CreateIconIndirect(&icon_info).unwrap_or(HICON(null_mut()));
        let _ = DeleteObject(hbm_color.into());
        let _ = DeleteObject(hbm_mask.into());
        hicon
    }
}

#[cfg(target_os = "windows")]
pub use imp::{acquire_single_instance, init, TaskbarHandle};

#[cfg(target_os = "linux")]
mod linux_imp {
    use std::io::{ErrorKind, Read, Write};
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::path::{Path, PathBuf};
    use std::sync::{Mutex, OnceLock};
    use tauri::AppHandle;

    static LISTENER: OnceLock<Mutex<Option<UnixListener>>> = OnceLock::new();

    #[derive(Clone, Copy)]
    pub struct TaskbarHandle;

    impl TaskbarHandle {
        pub fn set_playing(&self, _playing: bool) {}
    }

    fn socket_path() -> PathBuf {
        if let Some(dir) = std::env::var_os("XDG_RUNTIME_DIR").filter(|v| !v.is_empty()) {
            return PathBuf::from(dir).join("betterytmusic.sock");
        }
        let uid = std::fs::read_to_string("/proc/self/status")
            .ok()
            .and_then(|s| {
                s.lines()
                    .find_map(|l| l.strip_prefix("Uid:"))
                    .and_then(|v| v.split_whitespace().next())
                    .map(str::to_string)
            })
            .unwrap_or_else(|| "0".to_string());
        std::env::temp_dir().join(format!("betterytmusic-{uid}.sock"))
    }

    fn notify_existing(path: &Path) -> Result<(), std::io::Error> {
        let mut stream = UnixStream::connect(path)?;
        let _ = stream.write_all(b"SHOW\n");
        let _ = stream.flush();
        Ok(())
    }

    pub fn acquire_single_instance() -> bool {
        let path = socket_path();
        match notify_existing(&path) {
            Ok(()) => return false,
            Err(e) if e.kind() == ErrorKind::ConnectionRefused => {
                let _ = std::fs::remove_file(&path);
            }
            Err(_) => {}
        }

        let listener = match UnixListener::bind(&path) {
            Ok(l) => l,
            Err(e) if e.kind() == ErrorKind::AddrInUse => {
                if notify_existing(&path).is_ok() {
                    return false;
                }
                let _ = std::fs::remove_file(&path);
                match UnixListener::bind(&path) {
                    Ok(l) => l,
                    Err(err) => {
                        eprintln!("[single-instance] unix socket unavailable: {err}");
                        return true;
                    }
                }
            }
            Err(e) => {
                eprintln!("[single-instance] unix socket unavailable: {e}");
                return true;
            }
        };

        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
        let _ = LISTENER.set(Mutex::new(Some(listener)));
        true
    }

    pub fn init(app: &AppHandle) -> Option<TaskbarHandle> {
        let listener = LISTENER.get()?.lock().ok()?.take()?;
        let handle = app.clone();
        let _ = std::thread::Builder::new()
            .name("betterytmusic-ipc".to_string())
            .spawn(move || {
                for incoming in listener.incoming() {
                    let Ok(mut stream) = incoming else {
                        continue;
                    };
                    let mut buf = [0u8; 32];
                    let n = stream.read(&mut buf).unwrap_or(0);
                    if n == 0 || buf[..n].starts_with(b"SHOW") {
                        let h = handle.clone();
                        let _ = handle.run_on_main_thread(move || {
                            crate::tray::show_main_window(&h);
                        });
                    }
                }
            });
        None
    }
}

#[cfg(target_os = "linux")]
pub use linux_imp::{acquire_single_instance, init, TaskbarHandle};

#[cfg(all(not(target_os = "windows"), not(target_os = "linux")))]
pub fn acquire_single_instance() -> bool {
    true
}

#[cfg(all(not(target_os = "windows"), not(target_os = "linux")))]
#[derive(Clone, Copy)]
pub struct TaskbarHandle;

#[cfg(all(not(target_os = "windows"), not(target_os = "linux")))]
impl TaskbarHandle {
    pub fn set_playing(&self, _playing: bool) {}
}

#[cfg(all(not(target_os = "windows"), not(target_os = "linux")))]
pub fn init(_app: &AppHandle) -> Option<TaskbarHandle> {
    None
}

