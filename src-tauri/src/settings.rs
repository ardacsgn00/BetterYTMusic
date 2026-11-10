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

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
fn apply_platform_startup(_enable: bool, _minimized: bool) {}

