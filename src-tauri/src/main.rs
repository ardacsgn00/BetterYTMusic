// Prevents an extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    configure_linux_env();

    betterytmusic_lib::run()
}

#[cfg(target_os = "linux")]
fn configure_linux_env() {
    // WebKitGTK's DMABUF renderer often crashes or renders blank windows on NVIDIA drivers.
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none()
        && std::path::Path::new("/proc/driver/nvidia/version").exists()
    {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
}

