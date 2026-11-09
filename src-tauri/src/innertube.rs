//! Innertube API client for YouTube Music.
//!
//! Uses `WEB_REMIX` (music.youtube.com) for search, home feed, and radio queues,
//! and `VISIONOS` / `ANDROID_VR` with `X-Goog-Visitor-Id` for direct, cipher-free
//! M4A (`audio/mp4`) stream URLs.

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha1::{Digest, Sha1};

use crate::mock::Track;

const WEB_REMIX_VERSION: &str = "1.20260707.12.00";
const WEB_REMIX_UA: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

const VISIONOS_VERSION: &str = "1.02";
const VISIONOS_UA: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15";

const ANDROID_VR_VERSION: &str = "1.65.10";
const ANDROID_VR_UA: &str =
    "com.google.android.apps.youtube.vr.oculus/1.65.10 (Linux; U; Android 12L; eureka-user Build/SQ3A.220605.009.A1) gzip";

/// Filter param for "Songs" tab in YouTube Music search.
const SONGS_FILTER_PARAM: &str = "EgWKAQIIAWoKEAkQBRAKEAMQBA%3D%3D";
/// Filter param for "Featured Playlists" in YouTube Music search.
const FEATURED_PLAYLISTS_FILTER_PARAM: &str = "EgeKAQQoADgBahIQDhAFEAMQChAJEAQQFRAQEBE%3D";
/// Filter param for "Community Playlists" in YouTube Music search.
const COMMUNITY_PLAYLISTS_FILTER_PARAM: &str = "EgeKAQQoAEABahIQDhAFEAMQChAJEAQQFRAQEBE%3D";
/// Filter param for "Albums" in YouTube Music search.
const ALBUMS_FILTER_PARAM: &str = "EgWKAQIYAWoSEA4QBRADEAoQCRAEEBUQEBAR";
/// Filter param for "Artists" in YouTube Music search.
const ARTISTS_FILTER_PARAM: &str = "EgWKAQIgAWoSEA4QBRADEAoQCRAEEBUQEBAR";

const PALETTE: &[&str] = &[
    "#e4572e", "#3a86ff", "#8338ec", "#ffbe0b", "#06d6a0", "#ef476f", "#118ab2", "#f4a261",
];

pub struct ResolvedStream {
    pub url: String,
    pub user_agent: &'static str,
    pub duration_ms: Option<u64>,
    pub content_length: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatus {
    pub logged_in: bool,
    pub account_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistCard {
    pub playlist_id: String,
    pub title: String,
    pub subtitle: String,
    pub thumbnail: Option<String>,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoodCategory {
    pub title: String,
    pub params: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistCard {
    pub channel_id: String,
    pub name: String,
    pub subscribers: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistData {
    pub channel_id: String,
    pub name: String,
    pub description: Option<String>,
    pub subscribers: Option<String>,
    pub monthly_listeners: Option<String>,
    pub thumbnail: Option<String>,
    pub top_tracks: Vec<Track>,
    pub albums: Vec<PlaylistCard>,
    pub singles: Vec<PlaylistCard>,
    pub similar_artists: Vec<ArtistCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricLine {
    pub time_ms: u64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsData {
    pub synced: bool,
    pub lines: Vec<LyricLine>,
    pub plain: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheckResult {
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub release_url: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct StoredAuth {
    cookie: String,
    sapisid: String,
    #[serde(default)]
    user_session_id: Option<String>,
    #[serde(default)]
    delegated_session_id: Option<String>,
    #[serde(default)]
    session_index: Option<String>,
    account_name: Option<String>,
}

pub struct Innertube {
    agent: ureq::Agent,
    visitor_data: Mutex<Option<String>>,
    auth: Mutex<Option<StoredAuth>>,
    auth_file: Mutex<Option<PathBuf>>,
}

impl Default for Innertube {
    fn default() -> Self {
        Self::new()
    }
}

impl Innertube {
    pub fn new() -> Self {
        let tls = native_tls::TlsConnector::new().expect("failed to initialize native-tls");
        let agent = ureq::AgentBuilder::new()
            .tls_connector(std::sync::Arc::new(tls))
            .timeout_connect(Duration::from_secs(8))
            .timeout_read(Duration::from_secs(20))
            .build();
        Self {
            agent,
            visitor_data: Mutex::new(None),
            auth: Mutex::new(None),
            auth_file: Mutex::new(None),
        }
    }

    pub fn init_storage(&self, dir: PathBuf) {
        let _ = fs::create_dir_all(&dir);
        let file = dir.join("auth.json");
        if let Ok(mut f) = self.auth_file.lock() {
            *f = Some(file.clone());
        }
        if let Ok(raw) = fs::read_to_string(&file) {
            if let Ok(mut saved) = serde_json::from_str::<StoredAuth>(&raw) {
                if !saved.cookie.is_empty() && !saved.sapisid.is_empty() {
                    let needs_refresh =
                        saved.user_session_id.is_none() || saved.account_name.is_none();
                    if let Ok(mut g) = self.auth.lock() {
                        *g = Some(saved.clone());
                    }
                    if needs_refresh {
                        if saved.user_session_id.is_none() {
                            self.populate_session_metadata(&mut saved);
                            if let Ok(mut g) = self.auth.lock() {
                                *g = Some(saved.clone());
                            }
                        }
                        if saved.account_name.is_none() {
                            if let Some(name) = self.fetch_account_name() {
                                saved.account_name = Some(name);
                                if let Ok(mut g) = self.auth.lock() {
                                    *g = Some(saved.clone());
                                }
                            }
                        }
                        if let Ok(json_str) = serde_json::to_string_pretty(&saved) {
                            let _ = fs::write(&file, json_str);
                        }
                    }
                }
            }
        }
    }

    pub fn auth_status(&self) -> AuthStatus {
        if let Ok(g) = self.auth.lock() {
            if let Some(a) = g.as_ref() {
                return AuthStatus {
                    logged_in: true,
                    account_name: a.account_name.clone(),
                };
            }
        }
        AuthStatus {
            logged_in: false,
            account_name: None,
        }
    }

    pub fn set_cookies(&self, cookie_header: String, sapisid: String) -> AuthStatus {
        let mut stored = StoredAuth {
            cookie: cookie_header,
            sapisid,
            user_session_id: None,
            delegated_session_id: None,
            session_index: None,
            account_name: None,
        };
        self.populate_session_metadata(&mut stored);
        if let Ok(mut g) = self.auth.lock() {
            *g = Some(stored.clone());
        }
        if let Some(name) = self.fetch_account_name() {
            stored.account_name = Some(name);
            if let Ok(mut g) = self.auth.lock() {
                *g = Some(stored.clone());
            }
        }
        if let Ok(f) = self.auth_file.lock() {
            if let Some(path) = f.as_ref() {
                if let Ok(json_str) = serde_json::to_string_pretty(&stored) {
                    let _ = fs::write(path, json_str);
                }
            }
        }
        self.auth_status()
    }

    fn populate_session_metadata(&self, stored: &mut StoredAuth) {
        if let Ok(res) = self
            .agent
            .get("https://music.youtube.com/")
            .set("User-Agent", WEB_REMIX_UA)
            .set("Cookie", &stored.cookie)
            .call()
        {
            if let Ok(html) = res.into_string() {
                if let Some(v) = extract_json_str_field(&html, "USER_SESSION_ID") {
                    stored.user_session_id = Some(v);
                }
                if let Some(v) = extract_json_str_field(&html, "DELEGATED_SESSION_ID") {
                    stored.delegated_session_id = Some(v);
                }
                if let Some(v) = extract_json_str_field(&html, "SESSION_INDEX") {
                    stored.session_index = Some(v);
                }
                if let Some(v) = extract_json_str_field(&html, "VISITOR_DATA") {
                    if let Ok(mut vg) = self.visitor_data.lock() {
                        *vg = Some(v);
                    }
                }
            }
        }
    }

    pub fn clear_auth(&self) -> AuthStatus {
        if let Ok(mut g) = self.auth.lock() {
            *g = None;
        }
        if let Ok(f) = self.auth_file.lock() {
            if let Some(path) = f.as_ref() {
                let _ = fs::remove_file(path);
            }
        }
        self.auth_status()
    }

    fn fetch_account_name(&self) -> Option<String> {
        let body = json!({ "context": web_remix_context() });
        let req = self.authed_post("https://music.youtube.com/youtubei/v1/account/account_menu?prettyPrint=false");
        let val: Value = req.send_json(body).ok()?.into_json().ok()?;
        find_account_name(&val)
    }

    fn authed_post(&self, url: &str) -> ureq::Request {
        let origin = "https://music.youtube.com";
        let mut req = self
            .agent
            .post(url)
            .set("User-Agent", WEB_REMIX_UA)
            .set("Origin", origin)
            .set("X-Origin", origin)
            .set("X-YouTube-Client-Name", "67")
            .set("X-YouTube-Client-Version", WEB_REMIX_VERSION);

        if let Ok(vg) = self.visitor_data.lock() {
            if let Some(v) = vg.as_ref() {
                req = req.set("X-Goog-Visitor-Id", v);
            }
        }

        if let Ok(g) = self.auth.lock() {
            if let Some(a) = g.as_ref() {
                let ts = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let (hash_input, suffix) = match a.user_session_id.as_deref() {
                    Some(usid) if !usid.is_empty() => {
                        (format!("{usid} {ts} {} {origin}", a.sapisid), "_u")
                    }
                    _ => (format!("{ts} {} {origin}", a.sapisid), ""),
                };
                let mut hasher = Sha1::new();
                hasher.update(hash_input.as_bytes());
                let digest = hasher
                    .finalize()
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect::<String>();
                let token = format!("{ts}_{digest}{suffix}");
                let auth_header =
                    format!("SAPISIDHASH {token} SAPISID1PHASH {token} SAPISID3PHASH {token}");

                req = req
                    .set("Cookie", &a.cookie)
                    .set("Authorization", &auth_header)
                    .set(
                        "X-Goog-AuthUser",
                        a.session_index.as_deref().unwrap_or("0"),
                    )
                    .set("X-Youtube-Bootstrap-Logged-In", "true");

                if let Some(page_id) = a.delegated_session_id.as_deref() {
                    if !page_id.is_empty() {
                        req = req.set("X-Goog-PageId", page_id);
                    }
                }
            }
        }
        req
    }

    pub fn agent(&self) -> &ureq::Agent {
        &self.agent
    }

    fn save_visitor_data(&self, root: &Value) {
        if let Some(v) = root
            .pointer("/responseContext/visitorData")
            .and_then(Value::as_str)
        {
            if let Ok(mut guard) = self.visitor_data.lock() {
                *guard = Some(v.to_string());
            }
        }
    }

    fn ensure_visitor_data(&self) -> Option<String> {
        if let Ok(guard) = self.visitor_data.lock() {
            if guard.is_some() {
                return guard.clone();
            }
        }
        // Lightweight call to obtain a fresh visitorData token if we don't have one yet.
        let body = json!({
            "context": web_remix_context(),
            "browseId": "FEmusic_home"
        });
        if let Ok(res) = self
            .agent
            .post("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false")
            .set("User-Agent", WEB_REMIX_UA)
            .set("Origin", "https://music.youtube.com")
            .send_json(body)
        {
            if let Ok(val) = res.into_json::<Value>() {
                self.save_visitor_data(&val);
            }
        }
        self.visitor_data.lock().ok().and_then(|g| g.clone())
    }

    /// Search YouTube Music songs.
    pub fn search(&self, query: &str) -> Result<Vec<Track>, String> {
        let q = query.trim();
        if q.is_empty() {
            return Ok(Vec::new());
        }
        let body = json!({
            "context": web_remix_context(),
            "query": q,
            "params": SONGS_FILTER_PARAM
        });

        let val: Value = self
            .authed_post("https://music.youtube.com/youtubei/v1/search?prettyPrint=false")
            .send_json(body)
            .map_err(|e| format!("Arama isteği başarısız: {e}"))?
            .into_json()
            .map_err(|e| format!("Arama yanıtı okunamadı: {e}"))?;

        self.save_visitor_data(&val);

        let mut out = Vec::new();
        collect_responsive_items(&val, &mut out);
        Ok(out)
    }

    /// Search YouTube Music playlists and albums.
    pub fn search_playlists(&self, query: &str) -> Result<Vec<PlaylistCard>, String> {
        let q = query.trim();
        if q.is_empty() {
            return Ok(Vec::new());
        }

        let mut out = Vec::new();
        for param in &[
            COMMUNITY_PLAYLISTS_FILTER_PARAM,
            FEATURED_PLAYLISTS_FILTER_PARAM,
            ALBUMS_FILTER_PARAM,
        ] {
            let body = json!({
                "context": web_remix_context(),
                "query": q,
                "params": param
            });

            if let Ok(res) = self
                .authed_post("https://music.youtube.com/youtubei/v1/search?prettyPrint=false")
                .send_json(body)
            {
                if let Ok(val) = res.into_json::<Value>() {
                    self.save_visitor_data(&val);
                    collect_playlist_cards(&val, &mut out);
                }
            }
            if out.len() >= 36 {
                break;
            }
        }

        Ok(out)
    }

    /// Fetch home/trending tracks from YouTube Music (`FEmusic_home`), falling
    /// back to a popular search query if home shelves contain only albums/playlists.
    pub fn home_tracks(&self) -> Result<Vec<Track>, String> {
        let body = json!({
            "context": web_remix_context(),
            "browseId": "FEmusic_home"
        });

        let val: Value = self
            .authed_post("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false")
            .send_json(body)
            .map_err(|e| format!("Ana sayfa isteği başarısız: {e}"))?
            .into_json()
            .map_err(|e| format!("Ana sayfa yanıtı okunamadı: {e}"))?;

        self.save_visitor_data(&val);

        let mut out = Vec::new();
        collect_responsive_items(&val, &mut out);
        collect_two_row_items(&val, &mut out);

        if out.len() < 6 {
            if let Ok(extra) = self.search("Top Hits") {
                for t in extra {
                    if !out.iter().any(|x| x.id == t.id) {
                        out.push(t);
                    }
                }
            }
        }
        Ok(out)
    }

    /// Fetch the automatic "Up Next" radio queue (`RDAMVM<videoId>`) for a video.
    pub fn radio_queue(&self, video_id: &str) -> Result<Vec<Track>, String> {
        let body = json!({
            "context": web_remix_context(),
            "videoId": video_id,
            "playlistId": format!("RDAMVM{video_id}"),
            "isAudioOnly": true
        });

        let val: Value = self
            .authed_post("https://music.youtube.com/youtubei/v1/next?prettyPrint=false")
            .send_json(body)
            .map_err(|e| format!("Kuyruk isteği başarısız: {e}"))?
            .into_json()
            .map_err(|e| format!("Kuyruk yanıtı okunamadı: {e}"))?;

        self.save_visitor_data(&val);

        let mut out = Vec::new();
        collect_playlist_panel_items(&val, &mut out);
        Ok(out)
    }

    /// Fetch the signed-in user's Liked Music (`FEmusic_liked_videos`) from YouTube Music.
    pub fn liked_tracks(&self) -> Result<Vec<Track>, String> {
        let body = json!({
            "context": web_remix_context(),
            "browseId": "FEmusic_liked_videos"
        });

        let val: Value = self
            .authed_post("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false")
            .send_json(body)
            .map_err(|e| format!("Beğenilen şarkılar alınamadı: {e}"))?
            .into_json()
            .map_err(|e| format!("Beğenilen şarkılar yanıtı okunamadı: {e}"))?;

        let mut out = Vec::new();
        collect_responsive_items(&val, &mut out);
        Ok(out)
    }

    /// Fetch personalized Mixes (Supermix, Archive Mix, etc.) and Playlists from Home & Library.
    pub fn home_playlists(&self) -> Result<Vec<PlaylistCard>, String> {
        let mut out = vec![PlaylistCard {
            playlist_id: "RDTMAK5uy_kset8DisdE7LSD4TNjEVvrKRTmG7a56sY".to_string(),
            title: "Süpermix'im".to_string(),
            subtitle: "Sana özel kesintisiz kişisel karışım".to_string(),
            thumbnail: None,
            color: "#ff4e45".to_string(),
        }];

        for browse_id in ["FEmusic_home", "FEmusic_liked_playlists"] {
            let body = json!({
                "context": web_remix_context(),
                "browseId": browse_id
            });

            if let Ok(res) = self
                .authed_post("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false")
                .send_json(body)
            {
                if let Ok(val) = res.into_json::<Value>() {
                    self.save_visitor_data(&val);
                    collect_playlist_cards(&val, &mut out);
                }
            }
        }

        // Remove non-playable "Yeni çalma listesi" / system buttons if any slipped in.
        out.retain(|c| {
            !c.playlist_id.is_empty()
                && c.playlist_id != "LM"
                && c.playlist_id != "SE"
                && !c.title.to_lowercase().contains("yeni çalma listesi")
        });

        Ok(out)
    }

    /// Fetch all Moods and Genres from `FEmusic_moods_and_genres`.
    pub fn mood_categories(&self) -> Result<Vec<MoodCategory>, String> {
        let body = json!({
            "context": web_remix_context(),
            "browseId": "FEmusic_moods_and_genres"
        });

        let val: Value = self
            .authed_post("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false")
            .send_json(body)
            .map_err(|e| format!("Kategori isteği başarısız: {e}"))?
            .into_json()
            .map_err(|e| format!("Kategori yanıtı okunamadı: {e}"))?;

        self.save_visitor_data(&val);

        let mut out = Vec::new();
        collect_mood_categories(&val, &mut out);
        Ok(out)
    }

    /// Fetch playlists belonging to a specific mood/genre category.
    pub fn mood_playlists(&self, params: &str) -> Result<Vec<PlaylistCard>, String> {
        let body = json!({
            "context": web_remix_context(),
            "browseId": "FEmusic_moods_and_genres_category",
            "params": params
        });

        let val: Value = self
            .authed_post("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false")
            .send_json(body)
            .map_err(|e| format!("Kategori içeriği isteği başarısız: {e}"))?
            .into_json()
            .map_err(|e| format!("Kategori içeriği yanıtı okunamadı: {e}"))?;

        self.save_visitor_data(&val);

        let mut out = Vec::new();
        collect_playlist_cards(&val, &mut out);
        Ok(out)
    }

    /// Search for an artist channel browseId by name.
    pub fn search_artist(&self, query: &str) -> Result<Option<ArtistCard>, String> {
        let q = query.trim();
        if q.is_empty() {
            return Ok(None);
        }
        let body = json!({
            "context": web_remix_context(),
            "query": q,
            "params": ARTISTS_FILTER_PARAM
        });

        let val: Value = self
            .authed_post("https://music.youtube.com/youtubei/v1/search?prettyPrint=false")
            .send_json(body)
            .map_err(|e| format!("Sanatçı arama isteği başarısız: {e}"))?
            .into_json()
            .map_err(|e| format!("Sanatçı arama yanıtı okunamadı: {e}"))?;

        self.save_visitor_data(&val);

        let mut artists = Vec::new();
        collect_artist_cards(&val, &mut artists);
        Ok(artists.into_iter().next())
    }

    /// Fetch artist discography, top tracks, albums, singles, and similar artists.
    pub fn artist_details(&self, browse_id: &str) -> Result<ArtistData, String> {
        let bid = browse_id.trim();
        if bid.is_empty() {
            return Err("Geçersiz sanatçı kimliği".to_string());
        }

        let body = json!({
            "context": web_remix_context(),
            "browseId": bid
        });

        let val: Value = self
            .authed_post("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false")
            .send_json(body)
            .map_err(|e| format!("Sanatçı sayfası isteği başarısız: {e}"))?
            .into_json()
            .map_err(|e| format!("Sanatçı sayfası yanıtı okunamadı: {e}"))?;

        self.save_visitor_data(&val);

        parse_artist_data(bid, &val)
    }

    /// Fetch time-synced LRC lyrics from LRCLIB (`lrclib.net`), falling back to plain lyrics
    /// from LRCLIB or YouTube Music's official Innertube `MPLYt...` lyrics tab.
    pub fn fetch_lyrics(
        &self,
        video_id: &str,
        title: &str,
        artist: &str,
        duration_ms: u64,
    ) -> Result<Option<LyricsData>, String> {
        let clean_t = clean_track_title(title);
        let clean_a = clean_artist_name(artist);

        let queries = {
            let mut q = Vec::new();
            let first = format!("{clean_a} {clean_t}").trim().to_string();
            if !first.is_empty() {
                q.push(first);
            }
            let raw = format!("{artist} {title}").trim().to_string();
            if !raw.is_empty() && !q.contains(&raw) {
                q.push(raw);
            }
            if !clean_t.is_empty() && !q.contains(&clean_t) {
                q.push(clean_t.clone());
            }
            q
        };

        let mut plain_fallback: Option<String> = None;

        for query in queries {
            let url = format!(
                "https://lrclib.net/api/search?q={}",
                url_encode_component(&query)
            );
            let res = self
                .agent
                .get(&url)
                .set("User-Agent", "BetterYTMusic/0.6.0 (https://github.com/betterytmusic)")
                .call();

            if let Ok(resp) = res {
                if let Ok(arr) = resp.into_json::<Vec<Value>>() {
                    let mut best_synced: Option<(i32, Vec<LyricLine>, Option<String>)> = None;

                    for item in &arr {
                        if item.get("instrumental").and_then(Value::as_bool) == Some(true) {
                            continue;
                        }
                        let item_dur_ms = item
                            .get("duration")
                            .and_then(Value::as_f64)
                            .map(|s| (s * 1000.0) as u64)
                            .unwrap_or(0);

                        let mut score = 0i32;
                        if duration_ms > 0 && item_dur_ms > 0 {
                            let diff = item_dur_ms.abs_diff(duration_ms);
                            if diff <= 4_000 {
                                score += 60;
                            } else if diff <= 12_000 {
                                score += 35;
                            } else if diff > 35_000 {
                                score -= 40;
                            }
                        }

                        if let Some(t_name) = item.get("trackName").and_then(Value::as_str) {
                            if t_name.eq_ignore_ascii_case(&clean_t) || t_name.eq_ignore_ascii_case(title) {
                                score += 30;
                            }
                        }
                        if let Some(a_name) = item.get("artistName").and_then(Value::as_str) {
                            if !clean_a.is_empty()
                                && a_name.to_lowercase().contains(&clean_a.to_lowercase())
                            {
                                score += 25;
                            }
                        }

                        let plain = item
                            .get("plainLyrics")
                            .and_then(Value::as_str)
                            .map(str::trim)
                            .filter(|s| !s.is_empty())
                            .map(String::from);

                        if plain_fallback.is_none() && plain.is_some() {
                            plain_fallback = plain.clone();
                        }

                        if let Some(lrc_raw) = item.get("syncedLyrics").and_then(Value::as_str) {
                            let parsed = parse_lrc(lrc_raw);
                            if !parsed.is_empty() {
                                if best_synced.as_ref().map_or(true, |(best_s, _, _)| score > *best_s) {
                                    best_synced = Some((score, parsed, plain));
                                }
                            }
                        }
                    }

                    if let Some((_, lines, plain)) = best_synced {
                        return Ok(Some(LyricsData {
                            synced: true,
                            lines,
                            plain,
                            source: "LRCLIB".to_string(),
                        }));
                    }
                }
            }
        }

        // Fallback 1: YouTube Music official Innertube lyrics (`MPLYt...` browseId).
        if let Some(yt_plain) = self.fetch_innertube_lyrics(video_id) {
            return Ok(Some(LyricsData {
                synced: false,
                lines: Vec::new(),
                plain: Some(yt_plain),
                source: "YouTube Music".to_string(),
            }));
        }

        // Fallback 2: Plain lyrics from LRCLIB if available.
        if let Some(plain) = plain_fallback {
            return Ok(Some(LyricsData {
                synced: false,
                lines: Vec::new(),
                plain: Some(plain),
                source: "LRCLIB".to_string(),
            }));
        }

        Ok(None)
    }

    /// Check GitHub (`ardacsgn00/betterytmusic`) for a newer release or `package.json` version.
    pub fn check_for_updates(&self) -> Result<UpdateCheckResult, String> {
        let current = env!("CARGO_PKG_VERSION").to_string();
        let repo_url = "https://github.com/ardacsgn00/betterytmusic".to_string();
        let ua = format!("BetterYTMusic/{current}");

        // 1) Check GitHub Releases API (`releases/latest`)
        if let Ok(resp) = self
            .agent
            .get("https://api.github.com/repos/ardacsgn00/betterytmusic/releases/latest")
            .set("User-Agent", &ua)
            .set("Accept", "application/vnd.github+json")
            .call()
        {
            if let Ok(val) = resp.into_json::<Value>() {
                if let Some(tag) = val.get("tag_name").and_then(Value::as_str) {
                    let latest = tag.trim().trim_start_matches('v').to_string();
                    let html_url = val
                        .get("html_url")
                        .and_then(Value::as_str)
                        .unwrap_or(&repo_url)
                        .to_string();
                    let notes = val
                        .get("body")
                        .and_then(Value::as_str)
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(String::from);
                    let update_available = is_newer_version(&current, &latest);
                    return Ok(UpdateCheckResult {
                        current_version: current,
                        latest_version: latest,
                        update_available,
                        release_url: html_url,
                        notes,
                    });
                }
            }
        }

        // 2) Fallback: check `package.json` on `main` branch
        if let Ok(resp) = self
            .agent
            .get("https://raw.githubusercontent.com/ardacsgn00/betterytmusic/main/package.json")
            .set("User-Agent", &ua)
            .call()
        {
            if let Ok(val) = resp.into_json::<Value>() {
                if let Some(ver) = val.get("version").and_then(Value::as_str) {
                    let latest = ver.trim().trim_start_matches('v').to_string();
                    let update_available = is_newer_version(&current, &latest);
                    return Ok(UpdateCheckResult {
                        current_version: current,
                        latest_version: latest,
                        update_available,
                        release_url: repo_url,
                        notes: None,
                    });
                }
            }
        }

        Ok(UpdateCheckResult {
            current_version: current.clone(),
            latest_version: current,
            update_available: false,
            release_url: repo_url,
            notes: None,
        })
    }

    fn fetch_innertube_lyrics(&self, video_id: &str) -> Option<String> {
        let vid = video_id.trim();
        if vid.is_empty() {
            return None;
        }
        let next_body = json!({
            "context": web_remix_context(),
            "videoId": vid
        });
        let next_val: Value = self
            .authed_post("https://music.youtube.com/youtubei/v1/next?prettyPrint=false")
            .send_json(next_body)
            .ok()?
            .into_json()
            .ok()?;

        let browse_id = find_lyrics_browse_id(&next_val)?;
        let browse_body = json!({
            "context": web_remix_context(),
            "browseId": browse_id
        });
        let browse_val: Value = self
            .authed_post("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false")
            .send_json(browse_body)
            .ok()?
            .into_json()
            .ok()?;

        extract_description_shelf_text(&browse_val)
    }

    /// Fetch playable tracks for any Mix (`RDTMAK...`, `RDAMPL...`, `RDAMVM...`) or Playlist (`PL...`, `OLAK...`).
    pub fn playlist_tracks(&self, playlist_id: &str) -> Result<Vec<Track>, String> {
        let pid = playlist_id.trim().trim_start_matches("VL");
        if pid.is_empty() {
            return Ok(Vec::new());
        }

        let mut out = Vec::new();
        let is_radio_mix = pid.starts_with("RD");

        // For standard user/community playlists (`PL...`, `OLAK...`, `MPREb_...`),
        // query `/browse` (`VL<pid>`) first so we get the full tracklist (e.g. 70+ songs).
        if !is_radio_mix {
            let browse_id = if pid.starts_with("MPREb_") || pid.starts_with("VL") {
                pid.to_string()
            } else {
                format!("VL{pid}")
            };
            let browse_body = json!({
                "context": web_remix_context(),
                "browseId": browse_id
            });
            if let Ok(res) = self
                .authed_post("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false")
                .send_json(browse_body)
            {
                if let Ok(val) = res.into_json::<Value>() {
                    self.save_visitor_data(&val);
                    collect_responsive_items(&val, &mut out);
                }
            }
        }

        // For `RD*` mixes (Supermix, Archive Mix, Radio) OR if `/browse` returned nothing,
        // query `/next` which generates/returns the watch queue.
        if is_radio_mix || out.is_empty() {
            let next_body = json!({
                "context": web_remix_context(),
                "playlistId": pid,
                "isAudioOnly": true
            });

            if let Ok(res) = self
                .authed_post("https://music.youtube.com/youtubei/v1/next?prettyPrint=false")
                .send_json(next_body)
            {
                if let Ok(val) = res.into_json::<Value>() {
                    self.save_visitor_data(&val);
                    collect_playlist_panel_items(&val, &mut out);
                }
            }
        }

        Ok(out)
    }

    /// Resolve a playable audio stream URL (`audio/mp4` AAC preferred for Symphonia).
    pub fn resolve_stream(&self, video_id: &str) -> Result<ResolvedStream, String> {
        let visitor = self.ensure_visitor_data();

        // Try VISIONOS first, then ANDROID_VR as fallback.
        if let Some(s) = self.try_player_client(
            video_id,
            visitor.as_deref(),
            "101",
            VISIONOS_VERSION,
            VISIONOS_UA,
            json!({
                "clientName": "VISIONOS",
                "clientVersion": VISIONOS_VERSION,
                "deviceMake": "Apple",
                "deviceModel": "RealityDevice17,1",
                "osName": "visionOS",
                "osVersion": "26.5.23O471",
                "hl": "tr",
                "gl": "TR"
            }),
        ) {
            return Ok(s);
        }

        if let Some(s) = self.try_player_client(
            video_id,
            visitor.as_deref(),
            "28",
            ANDROID_VR_VERSION,
            ANDROID_VR_UA,
            json!({
                "clientName": "ANDROID_VR",
                "clientVersion": ANDROID_VR_VERSION,
                "deviceMake": "Oculus",
                "deviceModel": "Quest 3",
                "androidSdkVersion": 32,
                "osName": "Android",
                "osVersion": "12L",
                "hl": "tr",
                "gl": "TR"
            }),
        ) {
            return Ok(s);
        }

        Err(format!("'{video_id}' için ses akışı çözümlenemedi"))
    }

    fn try_player_client(
        &self,
        video_id: &str,
        visitor: Option<&str>,
        client_id: &str,
        client_ver: &str,
        ua: &'static str,
        client_ctx: Value,
    ) -> Option<ResolvedStream> {
        let body = json!({
            "context": { "client": client_ctx },
            "videoId": video_id,
            "contentCheckOk": true,
            "racyCheckOk": true
        });

        let mut req = self
            .agent
            .post("https://www.youtube.com/youtubei/v1/player?prettyPrint=false")
            .set("User-Agent", ua)
            .set("Origin", "https://www.youtube.com")
            .set("X-YouTube-Client-Name", client_id)
            .set("X-YouTube-Client-Version", client_ver);

        if let Some(v) = visitor {
            req = req.set("X-Goog-Visitor-Id", v);
        }

        let val: Value = req.send_json(body).ok()?.into_json().ok()?;
        let formats = val
            .pointer("/streamingData/adaptiveFormats")
            .and_then(Value::as_array)?;

        // Pick highest-bitrate audio/mp4 (AAC in MP4 container, supported by symphonia-isomp4 + symphonia-aac).
        let best = formats
            .iter()
            .filter(|f| {
                f.get("url").and_then(Value::as_str).is_some()
                    && f.get("mimeType")
                        .and_then(Value::as_str)
                        .map(|m| m.starts_with("audio/mp4"))
                        .unwrap_or(false)
            })
            .max_by_key(|f| f.get("bitrate").and_then(Value::as_u64).unwrap_or(0))?;

        let url = best.get("url")?.as_str()?.to_string();
        let content_length = best
            .get("contentLength")
            .and_then(Value::as_str)
            .and_then(|s| s.parse::<u64>().ok());
        let duration_ms = best
            .get("approxDurationMs")
            .and_then(Value::as_str)
            .and_then(|s| s.parse::<u64>().ok())
            .or_else(|| {
                val.pointer("/videoDetails/lengthSeconds")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse::<u64>().ok())
                    .map(|s| s * 1000)
            });

        Some(ResolvedStream {
            url,
            user_agent: ua,
            duration_ms,
            content_length,
        })
    }
}

fn web_remix_context() -> Value {
    json!({
        "client": {
            "clientName": "WEB_REMIX",
            "clientVersion": WEB_REMIX_VERSION,
            "hl": "tr",
            "gl": "TR"
        }
    })
}

fn find_account_name(v: &Value) -> Option<String> {
    match v {
        Value::Object(map) => {
            if let Some(header) = map.get("activeAccountHeaderRenderer") {
                if let Some(name) = header
                    .pointer("/accountName/runs/0/text")
                    .and_then(Value::as_str)
                {
                    return Some(name.to_string());
                }
            }
            for child in map.values() {
                if let Some(n) = find_account_name(child) {
                    return Some(n);
                }
            }
            None
        }
        Value::Array(arr) => arr.iter().find_map(find_account_name),
        _ => None,
    }
}

fn extract_json_str_field(html: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\":\"");
    let start = html.find(&needle)? + needle.len();
    let rest = &html[start..];
    let end = rest.find('"')?;
    let val = &rest[..end];
    if val.is_empty() {
        None
    } else {
        Some(val.to_string())
    }
}

fn color_for_id(id: &str) -> String {
    let hash = id
        .bytes()
        .fold(0usize, |acc, b| acc.wrapping_mul(31).wrapping_add(b as usize));
    PALETTE[hash % PALETTE.len()].to_string()
}

fn upscale_thumb(url: &str) -> String {
    if let Some((base, _)) = url.rsplit_once("=w") {
        format!("{base}=w544-h544-l90-rj")
    } else {
        url.to_string()
    }
}

fn parse_duration_ms(s: &str) -> Option<u64> {
    let parts: Vec<&str> = s.trim().split(':').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return None;
    }
    let mut secs = 0u64;
    for p in parts {
        let n = p.parse::<u64>().ok()?;
        secs = secs * 60 + n;
    }
    Some(secs * 1000)
}

/// Recursively walk JSON and extract any `musicResponsiveListItemRenderer` that has a `videoId`.
fn collect_responsive_items(v: &Value, out: &mut Vec<Track>) {
    match v {
        Value::Object(map) => {
            if let Some(item) = map.get("musicResponsiveListItemRenderer") {
                if let Some(t) = parse_responsive_item(item) {
                    if !out.iter().any(|x| x.id == t.id) {
                        out.push(t);
                    }
                }
            }
            for child in map.values() {
                collect_responsive_items(child, out);
            }
        }
        Value::Array(arr) => {
            for child in arr {
                collect_responsive_items(child, out);
            }
        }
        _ => {}
    }
}

fn parse_responsive_item(item: &Value) -> Option<Track> {
    let video_id = item
        .pointer("/playlistItemData/videoId")
        .and_then(Value::as_str)
        .or_else(|| {
            item.pointer("/overlay/musicItemThumbnailOverlayRenderer/content/musicPlayButtonRenderer/playNavigationEndpoint/watchEndpoint/videoId")
                .and_then(Value::as_str)
        })?;

    let cols = item.get("flexColumns")?.as_array()?;
    let title = cols
        .first()?
        .pointer("/musicResponsiveListItemFlexColumnRenderer/text/runs/0/text")
        .and_then(Value::as_str)?
        .to_string();

    let mut artist = String::new();
    let mut album = String::new();
    let mut duration_ms = 0u64;

    if let Some(runs) = cols
        .get(1)
        .and_then(|c| c.pointer("/musicResponsiveListItemFlexColumnRenderer/text/runs"))
        .and_then(Value::as_array)
    {
        // Split runs by " • " separator segments.
        let mut groups: Vec<String> = Vec::new();
        let mut current = String::new();
        for r in runs {
            let txt = r.get("text").and_then(Value::as_str).unwrap_or("");
            if txt.trim() == "•" {
                if !current.trim().is_empty() {
                    groups.push(current.trim().to_string());
                }
                current.clear();
            } else {
                current.push_str(txt);
            }
        }
        if !current.trim().is_empty() {
            groups.push(current.trim().to_string());
        }

        for g in groups {
            if let Some(d) = parse_duration_ms(&g) {
                duration_ms = d;
            } else if artist.is_empty()
                && !matches!(
                    g.as_str(),
                    "Şarkı" | "Video" | "Song" | "Tekli" | "Single" | "Bölüm"
                )
            {
                artist = g;
            } else if album.is_empty() && !g.contains("dinlenme") && !g.contains("görüntüleme")
            {
                album = g;
            }
        }
    }

    if album.is_empty() {
        if let Some(a_str) = cols
            .get(2)
            .and_then(|c| c.pointer("/musicResponsiveListItemFlexColumnRenderer/text/runs/0/text"))
            .and_then(Value::as_str)
        {
            if parse_duration_ms(a_str).is_none()
                && !a_str.contains("dinlenme")
                && !a_str.contains("görüntüleme")
            {
                album = a_str.to_string();
            }
        }
    }

    if duration_ms == 0 {
        if let Some(d_str) = item
            .pointer("/fixedColumns/0/musicResponsiveListItemFixedColumnRenderer/text/runs/0/text")
            .or_else(|| {
                item.pointer(
                    "/fixedColumns/0/musicResponsiveListItemFixedColumnRenderer/text/simpleText",
                )
            })
            .and_then(Value::as_str)
        {
            if let Some(d) = parse_duration_ms(d_str) {
                duration_ms = d;
            }
        }
    }

    if artist.is_empty() {
        artist = "YouTube Music".to_string();
    }

    let thumbnail = item
        .pointer("/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails")
        .and_then(Value::as_array)
        .and_then(|a| a.last())
        .and_then(|t| t.get("url"))
        .and_then(Value::as_str)
        .map(upscale_thumb);

    Some(Track {
        id: video_id.to_string(),
        title,
        artist,
        album,
        duration_ms: if duration_ms > 0 { duration_ms } else { 210_000 },
        thumbnail,
        color: color_for_id(video_id),
    })
}

/// Extract playable video items from `musicTwoRowItemRenderer` cards on the Home feed.
fn collect_two_row_items(v: &Value, out: &mut Vec<Track>) {
    match v {
        Value::Object(map) => {
            if let Some(item) = map.get("musicTwoRowItemRenderer") {
                if let Some(t) = parse_two_row_item(item) {
                    if !out.iter().any(|x| x.id == t.id) {
                        out.push(t);
                    }
                }
            }
            for child in map.values() {
                collect_two_row_items(child, out);
            }
        }
        Value::Array(arr) => {
            for child in arr {
                collect_two_row_items(child, out);
            }
        }
        _ => {}
    }
}

fn collect_playlist_cards(v: &Value, out: &mut Vec<PlaylistCard>) {
    match v {
        Value::Object(map) => {
            if let Some(item) = map.get("musicTwoRowItemRenderer") {
                if let Some(card) = parse_playlist_card(item) {
                    if let Some(existing) = out.iter_mut().find(|x| x.playlist_id == card.playlist_id) {
                        if existing.thumbnail.is_none() && card.thumbnail.is_some() {
                            existing.thumbnail = card.thumbnail;
                        }
                    } else {
                        out.push(card);
                    }
                }
            } else if let Some(item) = map.get("musicResponsiveListItemRenderer") {
                if let Some(card) = parse_responsive_playlist_card(item) {
                    if let Some(existing) = out.iter_mut().find(|x| x.playlist_id == card.playlist_id) {
                        if existing.thumbnail.is_none() && card.thumbnail.is_some() {
                            existing.thumbnail = card.thumbnail;
                        }
                    } else {
                        out.push(card);
                    }
                }
            }
            for child in map.values() {
                collect_playlist_cards(child, out);
            }
        }
        Value::Array(arr) => {
            for child in arr {
                collect_playlist_cards(child, out);
            }
        }
        _ => {}
    }
}

fn parse_responsive_playlist_card(item: &Value) -> Option<PlaylistCard> {
    let flex_col_0 = item.pointer("/flexColumns/0/musicResponsiveListItemFlexColumnRenderer")?;
    let title = flex_col_0
        .pointer("/text/runs/0/text")
        .and_then(Value::as_str)?
        .trim();
    if title.is_empty() {
        return None;
    }

    let nav = flex_col_0
        .pointer("/text/runs/0/navigationEndpoint")
        .or_else(|| item.get("navigationEndpoint"))?;

    let raw_id = nav
        .pointer("/browseEndpoint/browseId")
        .and_then(Value::as_str)
        .or_else(|| nav.pointer("/watchPlaylistEndpoint/playlistId").and_then(Value::as_str))?;

    let is_playlist_or_album = raw_id.starts_with("VL")
        || raw_id.starts_with("PL")
        || raw_id.starts_with("RD")
        || raw_id.starts_with("MPREb_")
        || raw_id.starts_with("OLAK");

    if !is_playlist_or_album {
        return None;
    }

    let playlist_id = raw_id.trim_start_matches("VL").to_string();

    let subtitle = item
        .pointer("/flexColumns/1/musicResponsiveListItemFlexColumnRenderer/text/runs")
        .and_then(Value::as_array)
        .map(|runs| {
            runs.iter()
                .filter_map(|r| r.get("text").and_then(Value::as_str))
                .collect::<String>()
        })
        .unwrap_or_else(|| "Çalma Listesi".to_string());

    let thumbnail = item
        .pointer("/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails")
        .and_then(Value::as_array)
        .and_then(|a| a.last())
        .and_then(|t| t.get("url"))
        .and_then(Value::as_str)
        .map(upscale_thumb);

    Some(PlaylistCard {
        color: color_for_id(&playlist_id),
        playlist_id,
        title: title.to_string(),
        subtitle,
        thumbnail,
    })
}

fn parse_playlist_card(item: &Value) -> Option<PlaylistCard> {
    // Only pick items that do NOT have a direct single `watchEndpoint.videoId`,
    // OR whose playlistId is a genuine playlist/mix/album (`RD...`, `PL...`, `OLAK...`, `MPREb_...`).
    let direct_vid = item
        .pointer("/navigationEndpoint/watchEndpoint/videoId")
        .and_then(Value::as_str);
    if direct_vid.is_some() {
        return None;
    }

    let playlist_id = item
        .pointer("/navigationEndpoint/watchPlaylistEndpoint/playlistId")
        .and_then(Value::as_str)
        .or_else(|| {
            item.pointer("/thumbnailOverlay/musicItemThumbnailOverlayRenderer/content/musicPlayButtonRenderer/playNavigationEndpoint/watchPlaylistEndpoint/playlistId")
                .and_then(Value::as_str)
        })
        .or_else(|| {
            item.pointer("/navigationEndpoint/browseEndpoint/browseId")
                .and_then(Value::as_str)
                .filter(|b| b.starts_with("VL") || b.starts_with("RD") || b.starts_with("MPREb_") || b.starts_with("OLAK"))
                .map(|b| b.trim_start_matches("VL"))
        })?;

    let title = item
        .pointer("/title/runs/0/text")
        .and_then(Value::as_str)?
        .to_string();

    let subtitle = item
        .pointer("/subtitle/runs")
        .and_then(Value::as_array)
        .map(|runs| {
            runs.iter()
                .filter_map(|r| r.get("text").and_then(Value::as_str))
                .collect::<String>()
        })
        .unwrap_or_else(|| "Çalma Listesi".to_string());

    let thumbnail = item
        .pointer("/thumbnailRenderer/musicThumbnailRenderer/thumbnail/thumbnails")
        .and_then(Value::as_array)
        .and_then(|a| a.last())
        .and_then(|t| t.get("url"))
        .and_then(Value::as_str)
        .map(upscale_thumb);

    Some(PlaylistCard {
        playlist_id: playlist_id.to_string(),
        title,
        subtitle,
        thumbnail,
        color: color_for_id(playlist_id),
    })
}

fn parse_two_row_item(item: &Value) -> Option<Track> {
    let video_id = item
        .pointer("/navigationEndpoint/watchEndpoint/videoId")
        .and_then(Value::as_str)
        .or_else(|| {
            item.pointer("/thumbnailOverlay/musicItemThumbnailOverlayRenderer/content/musicPlayButtonRenderer/playNavigationEndpoint/watchEndpoint/videoId")
                .and_then(Value::as_str)
        })?;

    let title = item
        .pointer("/title/runs/0/text")
        .and_then(Value::as_str)?
        .to_string();

    let subtitle = item
        .pointer("/subtitle/runs")
        .and_then(Value::as_array)
        .map(|runs| {
            runs.iter()
                .filter_map(|r| r.get("text").and_then(Value::as_str))
                .collect::<String>()
        })
        .unwrap_or_default();

    let mut artist = String::new();
    let mut duration_ms = 0u64;
    for part in subtitle.split('•').map(str::trim) {
        if let Some(d) = parse_duration_ms(part) {
            duration_ms = d;
        } else if artist.is_empty()
            && !part.is_empty()
            && !matches!(part, "Şarkı" | "Video" | "Song")
            && !part.contains("dinlenme")
            && !part.contains("görüntüleme")
        {
            artist = part.to_string();
        }
    }
    if artist.is_empty() {
        artist = "YouTube Music".to_string();
    }

    let thumbnail = item
        .pointer("/thumbnailRenderer/musicThumbnailRenderer/thumbnail/thumbnails")
        .and_then(Value::as_array)
        .and_then(|a| a.last())
        .and_then(|t| t.get("url"))
        .and_then(Value::as_str)
        .map(upscale_thumb);

    Some(Track {
        id: video_id.to_string(),
        title,
        artist,
        album: String::new(),
        duration_ms: if duration_ms > 0 { duration_ms } else { 210_000 },
        thumbnail,
        color: color_for_id(video_id),
    })
}

/// Extract tracks from `playlistPanelVideoRenderer` (returned by the `/next` endpoint).
fn collect_playlist_panel_items(v: &Value, out: &mut Vec<Track>) {
    match v {
        Value::Object(map) => {
            if let Some(item) = map.get("playlistPanelVideoRenderer") {
                if let Some(t) = parse_playlist_panel_item(item) {
                    if !out.iter().any(|x| x.id == t.id) {
                        out.push(t);
                    }
                }
            }
            for child in map.values() {
                collect_playlist_panel_items(child, out);
            }
        }
        Value::Array(arr) => {
            for child in arr {
                collect_playlist_panel_items(child, out);
            }
        }
        _ => {}
    }
}

fn parse_playlist_panel_item(item: &Value) -> Option<Track> {
    let video_id = item.get("videoId")?.as_str()?;
    let title = item
        .pointer("/title/runs/0/text")
        .and_then(Value::as_str)?
        .to_string();

    let mut artist = String::new();
    let mut album = String::new();
    if let Some(runs) = item
        .pointer("/longBylineText/runs")
        .or_else(|| item.pointer("/shortBylineText/runs"))
        .and_then(Value::as_array)
    {
        let joined: String = runs
            .iter()
            .filter_map(|r| r.get("text").and_then(Value::as_str))
            .collect();
        let parts: Vec<&str> = joined.split('•').map(str::trim).collect();
        if let Some(a) = parts.first() {
            artist = (*a).to_string();
        }
        if let Some(al) = parts.get(1) {
            album = (*al).to_string();
        }
    }
    if artist.is_empty() {
        artist = "YouTube Music".to_string();
    }

    let duration_ms = item
        .pointer("/lengthText/runs/0/text")
        .and_then(Value::as_str)
        .and_then(parse_duration_ms)
        .unwrap_or(0);

    let thumbnail = item
        .pointer("/thumbnail/thumbnails")
        .and_then(Value::as_array)
        .and_then(|a| a.last())
        .and_then(|t| t.get("url"))
        .and_then(Value::as_str)
        .map(upscale_thumb);

    Some(Track {
        id: video_id.to_string(),
        title,
        artist,
        album,
        duration_ms,
        thumbnail,
        color: color_for_id(video_id),
    })
}

fn is_newer_version(current: &str, latest: &str) -> bool {
    let parse = |s: &str| -> (u32, u32, u32) {
        let mut parts = s
            .trim()
            .trim_start_matches('v')
            .split('.')
            .map(|p| p.parse::<u32>().unwrap_or(0));
        (
            parts.next().unwrap_or(0),
            parts.next().unwrap_or(0),
            parts.next().unwrap_or(0),
        )
    };
    parse(latest) > parse(current)
}

fn url_encode_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for b in s.as_bytes() {
        match *b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char);
            }
            b' ' => out.push_str("%20"),
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}

fn clean_track_title(title: &str) -> String {
    let mut s = title.to_string();
    // Strip common parenthetical / bracketed noise like (feat. ...), [Official Video], etc.
    while let Some(start) = s.find('(') {
        if let Some(end) = s[start..].find(')') {
            s.replace_range(start..=start + end, "");
        } else {
            break;
        }
    }
    while let Some(start) = s.find('[') {
        if let Some(end) = s[start..].find(']') {
            s.replace_range(start..=start + end, "");
        } else {
            break;
        }
    }
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn clean_artist_name(artist: &str) -> String {
    let s = artist
        .trim_end_matches(" - Topic")
        .split(['&', ',', '•'])
        .next()
        .unwrap_or(artist);
    let s = s.split(" ve ").next().unwrap_or(s);
    let s = s.split(" feat").next().unwrap_or(s);
    let s = s.split(" ft.").next().unwrap_or(s);
    s.trim().to_string()
}

fn parse_lrc(lrc: &str) -> Vec<LyricLine> {
    let mut out = Vec::new();
    for raw_line in lrc.lines() {
        let line = raw_line.trim();
        if !line.starts_with('[') {
            continue;
        }
        let Some(close_idx) = line.find(']') else {
            continue;
        };
        let tag = &line[1..close_idx];
        let text = line[close_idx + 1..].trim();

        let Some((min_str, sec_str)) = tag.split_once(':') else {
            continue;
        };
        let Ok(mins) = min_str.trim().parse::<u64>() else {
            continue;
        };
        let Ok(secs) = sec_str.trim().replace(',', ".").parse::<f64>() else {
            continue;
        };
        let time_ms = mins * 60_000 + (secs * 1000.0).round() as u64;
        if !text.is_empty() {
            out.push(LyricLine {
                time_ms,
                text: text.to_string(),
            });
        }
    }
    out.sort_by_key(|l| l.time_ms);
    out
}

fn find_lyrics_browse_id(v: &Value) -> Option<String> {
    match v {
        Value::Object(map) => {
            if let Some(bid) = map.get("browseId").and_then(Value::as_str) {
                if bid.starts_with("MPLYt") {
                    return Some(bid.to_string());
                }
            }
            for child in map.values() {
                if let Some(found) = find_lyrics_browse_id(child) {
                    return Some(found);
                }
            }
            None
        }
        Value::Array(arr) => {
            for child in arr {
                if let Some(found) = find_lyrics_browse_id(child) {
                    return Some(found);
                }
            }
            None
        }
        _ => None,
    }
}

fn extract_description_shelf_text(v: &Value) -> Option<String> {
    match v {
        Value::Object(map) => {
            if let Some(shelf) = map.get("musicDescriptionShelfRenderer") {
                if let Some(runs) = shelf.pointer("/description/runs").and_then(Value::as_array) {
                    let txt: String = runs
                        .iter()
                        .filter_map(|r| r.get("text").and_then(Value::as_str))
                        .collect();
                    let trimmed = txt.trim();
                    if !trimmed.is_empty() {
                        return Some(trimmed.to_string());
                    }
                }
            }
            for child in map.values() {
                if let Some(found) = extract_description_shelf_text(child) {
                    return Some(found);
                }
            }
            None
        }
        Value::Array(arr) => {
            for child in arr {
                if let Some(found) = extract_description_shelf_text(child) {
                    return Some(found);
                }
            }
            None
        }
        _ => None,
    }
}

fn collect_mood_categories(v: &Value, out: &mut Vec<MoodCategory>) {
    match v {
        Value::Object(map) => {
            if let Some(btn) = map.get("musicNavigationButtonRenderer") {
                if let Some(cat) = parse_mood_category(btn) {
                    if !out.iter().any(|x| x.params == cat.params || x.title == cat.title) {
                        out.push(cat);
                    }
                }
            }
            for child in map.values() {
                collect_mood_categories(child, out);
            }
        }
        Value::Array(arr) => {
            for child in arr {
                collect_mood_categories(child, out);
            }
        }
        _ => {}
    }
}

fn parse_mood_category(btn: &Value) -> Option<MoodCategory> {
    let title = btn.pointer("/buttonText/runs/0/text")
        .and_then(Value::as_str)?
        .trim();
    if title.is_empty() {
        return None;
    }
    let params = btn.pointer("/clickCommand/browseEndpoint/params")
        .and_then(Value::as_str)?
        .trim();
    if params.is_empty() {
        return None;
    }
    let color = color_for_id(title);
    Some(MoodCategory {
        title: title.to_string(),
        params: params.to_string(),
        color,
    })
}

fn parse_artist_card_two_row(item: &Value) -> Option<ArtistCard> {
    let browse_id = item
        .pointer("/navigationEndpoint/browseEndpoint/browseId")
        .and_then(Value::as_str)?;
    if !browse_id.starts_with("UC") {
        return None;
    }
    let name = item
        .pointer("/title/runs/0/text")
        .and_then(Value::as_str)?
        .trim();
    if name.is_empty() {
        return None;
    }
    let subscribers = item
        .pointer("/subtitle/runs")
        .and_then(Value::as_array)
        .map(|runs| {
            runs.iter()
                .filter_map(|r| r.get("text").and_then(Value::as_str))
                .collect::<String>()
        });
    let thumbnail = item
        .pointer("/thumbnailRenderer/musicThumbnailRenderer/thumbnail/thumbnails")
        .and_then(Value::as_array)
        .and_then(|a| a.last())
        .and_then(|t| t.get("url"))
        .and_then(Value::as_str)
        .map(upscale_thumb);

    Some(ArtistCard {
        channel_id: browse_id.to_string(),
        name: name.to_string(),
        subscribers,
        thumbnail,
    })
}

fn parse_artist_card_responsive(item: &Value) -> Option<ArtistCard> {
    let flex_col_0 = item.pointer("/flexColumns/0/musicResponsiveListItemFlexColumnRenderer")?;
    let name = flex_col_0
        .pointer("/text/runs/0/text")
        .and_then(Value::as_str)?
        .trim();
    if name.is_empty() {
        return None;
    }

    let nav = flex_col_0
        .pointer("/text/runs/0/navigationEndpoint")
        .or_else(|| item.get("navigationEndpoint"))?;

    let browse_id = nav
        .pointer("/browseEndpoint/browseId")
        .and_then(Value::as_str)?;
    if !browse_id.starts_with("UC") {
        return None;
    }

    let subscribers = item
        .pointer("/flexColumns/1/musicResponsiveListItemFlexColumnRenderer/text/runs")
        .and_then(Value::as_array)
        .map(|runs| {
            runs.iter()
                .filter_map(|r| r.get("text").and_then(Value::as_str))
                .collect::<String>()
        });

    let thumbnail = item
        .pointer("/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails")
        .and_then(Value::as_array)
        .and_then(|a| a.last())
        .and_then(|t| t.get("url"))
        .and_then(Value::as_str)
        .map(upscale_thumb);

    Some(ArtistCard {
        channel_id: browse_id.to_string(),
        name: name.to_string(),
        subscribers,
        thumbnail,
    })
}

fn collect_artist_cards(v: &Value, out: &mut Vec<ArtistCard>) {
    match v {
        Value::Object(map) => {
            if let Some(item) = map.get("musicTwoRowItemRenderer") {
                if let Some(card) = parse_artist_card_two_row(item) {
                    if !out.iter().any(|x| x.channel_id == card.channel_id) {
                        out.push(card);
                    }
                }
            } else if let Some(item) = map.get("musicResponsiveListItemRenderer") {
                if let Some(card) = parse_artist_card_responsive(item) {
                    if !out.iter().any(|x| x.channel_id == card.channel_id) {
                        out.push(card);
                    }
                }
            }
            for child in map.values() {
                collect_artist_cards(child, out);
            }
        }
        Value::Array(arr) => {
            for child in arr {
                collect_artist_cards(child, out);
            }
        }
        _ => {}
    }
}

fn parse_artist_data(channel_id: &str, val: &Value) -> Result<ArtistData, String> {
    let header = val
        .pointer("/header/musicImmersiveHeaderRenderer")
        .or_else(|| val.pointer("/header/musicVisualHeaderRenderer"));

    let name = header
        .and_then(|h| h.pointer("/title/runs/0/text"))
        .and_then(Value::as_str)
        .unwrap_or("Sanatçı")
        .to_string();

    let description = header
        .and_then(|h| h.pointer("/description/runs/0/text"))
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    let subscribers = header
        .and_then(|h| h.pointer("/subscriptionButton/subscribeButtonRenderer/subscriberCountText/runs/0/text"))
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    let monthly_listeners = header
        .and_then(|h| h.pointer("/monthlyListenerCount/runs/0/text"))
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    let thumbnail = header
        .and_then(|h| h.pointer("/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails"))
        .and_then(Value::as_array)
        .and_then(|a| a.last())
        .and_then(|t| t.get("url"))
        .and_then(Value::as_str)
        .map(upscale_thumb);

    let mut top_tracks = Vec::new();
    let mut albums = Vec::new();
    let mut singles = Vec::new();
    let mut similar_artists = Vec::new();

    let mut shelves = Vec::new();
    collect_raw_shelves(val, &mut shelves);

    for (title, shelf_val) in shelves {
        let t_lower = title.to_lowercase();
        if t_lower.contains("şarkı") || t_lower.contains("songs") || t_lower.contains("tracks") {
            collect_responsive_items(&shelf_val, &mut top_tracks);
        } else if t_lower.contains("albüm") || t_lower.contains("albums") {
            collect_playlist_cards(&shelf_val, &mut albums);
        } else if t_lower.contains("single") || t_lower.contains("ep") {
            collect_playlist_cards(&shelf_val, &mut singles);
        } else if t_lower.contains("benzer") || t_lower.contains("similar") || t_lower.contains("fans might also like") {
            collect_artist_cards(&shelf_val, &mut similar_artists);
        }
    }

    Ok(ArtistData {
        channel_id: channel_id.to_string(),
        name,
        description,
        subscribers,
        monthly_listeners,
        thumbnail,
        top_tracks,
        albums,
        singles,
        similar_artists,
    })
}

fn collect_raw_shelves(v: &Value, out: &mut Vec<(String, Value)>) {
    match v {
        Value::Object(map) => {
            if let Some(shelf) = map.get("musicShelfRenderer") {
                let title = shelf
                    .pointer("/title/runs/0/text")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                out.push((title, shelf.clone()));
            } else if let Some(shelf) = map.get("musicCarouselShelfRenderer") {
                let title = shelf
                    .pointer("/header/musicCarouselShelfBasicHeaderRenderer/title/runs/0/text")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                out.push((title, shelf.clone()));
            }
            for child in map.values() {
                collect_raw_shelves(child, out);
            }
        }
        Value::Array(arr) => {
            for child in arr {
                collect_raw_shelves(child, out);
            }
        }
        _ => {}
    }
}

