//! Persistent playback session (`session.json`) for resuming the last queue,
//! active track, position, volume, and repeat/shuffle state across restarts.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::mock::Track;

use super::RepeatMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSession {
    pub queue: Vec<Track>,
    pub index: Option<usize>,
    pub position_ms: u64,
    pub volume: f32,
    pub repeat: RepeatMode,
    pub shuffle: bool,
    pub source_name: Option<String>,
    pub output_device: Option<String>,
}

pub fn load_session(path: &Path) -> Option<SavedSession> {
    let raw = fs::read_to_string(path).ok()?;
    let session = serde_json::from_str::<SavedSession>(&raw).ok()?;
    if session.queue.is_empty() {
        return None;
    }
    // Never restore internal mock placeholder tracks.
    if session.queue.iter().all(|t| t.id.starts_with("mock-")) {
        return None;
    }
    Some(session)
}

pub fn save_session(path: &Path, session: &SavedSession) {
    if session.queue.is_empty() || session.queue.iter().all(|t| t.id.starts_with("mock-")) {
        return;
    }
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string(session) {
        let _ = fs::write(path, json);
    }
}

