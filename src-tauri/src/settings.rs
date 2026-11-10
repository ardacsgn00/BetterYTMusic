//! Persistent application behavior settings (`settings.json`) + Windows Startup registry sync.

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

fn default_discord_id() -> String {
    "1569629580694655006".to_string()
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
                apply_windows_startup(saved.launch_on_startup, saved.start_minimized);
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
        apply_windows_startup(next.launch_on_startup, next.start_minimized);
        next
    }
}

#[cfg(target_os = "windows")]
fn apply_windows_startup(enable: bool, minimized: bool) {
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

#[cfg(not(target_os = "windows"))]
fn apply_windows_startup(_enable: bool, _minimized: bool) {}

