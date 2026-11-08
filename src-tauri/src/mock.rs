use serde::{Deserialize, Serialize};

/// A playable track. Field names mirror what we'll later map from Innertube
/// responses (`videoId`, title, artists, album, duration, thumbnails).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration_ms: u64,
    /// Remote cover URL. `None` → UI renders a solid placeholder using `color`.
    pub thumbnail: Option<String>,
    /// Placeholder / accent color for the cover (hex).
    pub color: String,
}

/// Mock catalogue used until the Innertube client is wired up.
#[allow(dead_code)]
pub fn tracks() -> Vec<Track> {
    let t = |id: &str, title: &str, artist: &str, album: &str, secs: u64, color: &str| Track {
        id: id.into(),
        title: title.into(),
        artist: artist.into(),
        album: album.into(),
        duration_ms: secs * 1000,
        thumbnail: None,
        color: color.into(),
    };

    vec![
        t("mock-01", "Gece Yarısı Treni", "Kuzey Rüzgârı", "Rotalar", 214, "#e4572e"),
        t("mock-02", "Neon Boğaz", "Lale Devri Synth", "Retro İstanbul", 187, "#3a86ff"),
        t("mock-03", "Kısa Test Parçası", "QA Orkestrası", "Gapless Testleri", 15, "#8338ec"),
        t("mock-04", "Sabah Kahvesi", "Mavi Bisiklet", "Pazar", 243, "#ffbe0b"),
        t("mock-05", "Sessiz Liman", "Deniz Feneri", "Kıyı", 198, "#06d6a0"),
        t("mock-06", "Asfalt Ritmi", "Beton Kalpler", "Şehir", 176, "#ef476f"),
        t("mock-07", "Yağmurdan Sonra", "Kuzey Rüzgârı", "Rotalar", 265, "#118ab2"),
        t("mock-08", "Son Durak", "Gece Vardiyası", "Yol Hikâyeleri", 229, "#f4a261"),
    ]
}

