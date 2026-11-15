//! Persistent application behavior settings (`settings.json`) + Windows Startup registry sync.

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_discord_id() -> String {
    "1569629580694655006".to_string()
}

fn default_discord_app_name() -> String {
    "BetterYTMusic".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub close_to_tray: bool,
    pub launch_on_startup: bool,
    pub start_minimized: bool,
    #[serde(default = "default_true")]
    pub autoplay: bool,
    #[serde(default = "default_true")]
    pub discord_rpc: bool,
    #[serde(default = "default_discord_id")]
    pub discord_client_id: String,
    #[serde(default = "default_discord_app_name")]
    pub discord_app_name: String,
    #[serde(default = "default_true")]
    pub discord_show_cover: bool,
    #[serde(default = "default_true")]
    pub discord_show_time: bool,
    #[serde(default = "default_false")]
    pub discord_clear_on_pause: bool,
    #[serde(default = "default_true")]
    pub loudness_normalization: bool,
    #[serde(default = "default_true")]
    pub resume_session: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            close_to_tray: true,
            launch_on_startup: false,
            start_minimized: false,
            autoplay: true,
            discord_rpc: true,
            discord_client_id: default_discord_id(),
            discord_app_name: default_discord_app_name(),
            discord_show_cover: true,
            discord_show_time: true,
            discord_clear_on_pause: false,
            loudness_normalization: true,
            resume_session: true,
        }
    }
}

pub struct SettingsManager {
    inner: Mutex<AppSettings>,
    file: Mutex<Option<PathBuf>>,
}

impl Default for SettingsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsManager {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(AppSettings::default()),
            file: Mutex::new(None),
        }
    }

    pub fn init(&self, dir: PathBuf) {
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("settings.json");
        if let Ok(raw) = fs::read_to_string(&path) {
            if let Ok(saved) = serde_json::from_str::<AppSettings>(&raw) {
                if let Ok(mut g) = self.inner.lock() {
                    *g = saved.clone();
                }
                apply_platform_startup(saved.launch_on_startup, saved.start_minimized);
            }
        }
        if let Ok(mut f) = self.file.lock() {
            *f = Some(path);
        }
    }

    pub fn get(&self) -> AppSettings {
        self.inner
            .lock()
            .map(|g| g.clone())
            .unwrap_or_default()
    }

    pub fn update(&self, next: AppSettings) -> AppSettings {
        if let Ok(mut g) = self.inner.lock() {
            *g = next.clone();
        }
        if let Ok(f) = self.file.lock() {
            if let Some(path) = f.as_ref() {
                if let Ok(json_str) = serde_json::to_string_pretty(&next) {
                    let _ = fs::write(path, json_str);
                }
            }
        }
        apply_platform_startup(next.launch_on_startup, next.start_minimized);
        next
    }
}

#[cfg(target_os = "windows")]
fn apply_platform_startup(enable: bool, minimized: bool) {
    use std::os::windows::process::CommandExt;
    use std::process::Command;

    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    const RUN_KEY: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
    const VALUE_NAME: &str = "BetterYTMusic";

    if enable {
        if let Ok(exe) = std::env::current_exe() {
            let exe_str = exe.to_string_lossy();
            let cmd_val = if minimized {
                format!("\"{exe_str}\" --minimized")
            } else {
                format!("\"{exe_str}\"")
            };
            let _ = Command::new("reg.exe")
                .args([
                    "add",
                    RUN_KEY,
                    "/v",
                    VALUE_NAME,
                    "/t",
                    "REG_SZ",
                    "/d",
                    &cmd_val,
                    "/f",
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .status();
        }
    } else {
        let _ = Command::new("reg.exe")
            .args(["delete", RUN_KEY, "/v", VALUE_NAME, "/f"])
            .creation_flags(CREATE_NO_WINDOW)
            .status();
    }
}

#[cfg(target_os = "macos")]
fn apply_platform_startup(enable: bool, minimized: bool) {
    // macOS LaunchAgent (.plist) in ~/Library/LaunchAgents/com.betterytmusic.app.plist
    if let Ok(home) = std::env::var("HOME") {
        let plist_path = std::path::PathBuf::from(home)
            .join("Library")
            .join("LaunchAgents")
            .join("com.betterytmusic.app.plist");

        if enable {
            if let Ok(exe) = std::env::current_exe() {
                let exe_str = exe.to_string_lossy();
                let arg_tag = if minimized {
                    "        <string>--minimized</string>\n"
                } else {
                    ""
                };
                let plist_content = format!(
                    r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.betterytmusic.app</string>
    <key>ProgramArguments</key>
    <array>
        <string>{exe_str}</string>
{arg_tag}    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
"#
                );
                if let Some(parent) = plist_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(&plist_path, plist_content);
            }
        } else {
            let _ = std::fs::remove_file(&plist_path);
        }
    }
}

#[cfg(target_os = "linux")]
fn escape_desktop_exec_arg(arg: &str) -> String {
    let mut out = String::with_capacity(arg.len() + 2);
    out.push('"');
    for ch in arg.chars() {
        match ch {
            '\\' | '"' | '$' | '`' => {
                out.push('\\');
                out.push(ch);
            }
            '%' => out.push_str("%%"),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

#[cfg(target_os = "linux")]
fn linux_autostart_path() -> Option<std::path::PathBuf> {
    let config_dir = std::env::var_os("XDG_CONFIG_HOME")
        .filter(|v| !v.is_empty())
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .filter(|v| !v.is_empty())
                .map(|home| std::path::PathBuf::from(home).join(".config"))
        })?;
    Some(config_dir.join("autostart").join("betterytmusic.desktop"))
}

#[cfg(target_os = "linux")]
fn apply_platform_startup(enable: bool, minimized: bool) {
    let Some(desktop_path) = linux_autostart_path() else {
        return;
    };
    if !enable {
        let _ = std::fs::remove_file(&desktop_path);
        return;
    }

    // AppImage mounts into an ephemeral /tmp/.mount_* directory, so prefer $APPIMAGE when set.
    let exe_path = std::env::var_os("APPIMAGE")
        .filter(|v| !v.is_empty())
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::current_exe().ok());
    let Some(exe) = exe_path else {
        return;
    };

    if let Some(parent) = desktop_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let quoted_exe = escape_desktop_exec_arg(&exe.to_string_lossy());
    let exec_line = if minimized {
        format!("{quoted_exe} --minimized")
    } else {
        quoted_exe
    };
    let desktop_entry = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Version=1.0\n\
         Name=BetterYTMusic\n\
         Comment=Lightweight YouTube Music desktop client\n\
         Exec={exec_line}\n\
         Icon=betterytmusic\n\
         Terminal=false\n\
         Categories=AudioVideo;Audio;Music;Player;\n\
         X-GNOME-Autostart-enabled=true\n"
    );
    let _ = std::fs::write(&desktop_path, desktop_entry);
}

#[cfg(all(
    not(target_os = "windows"),
    not(target_os = "macos"),
    not(target_os = "linux")
))]
fn apply_platform_startup(_enable: bool, _minimized: bool) {}

