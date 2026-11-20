//! Smart track deduplication, canonical title/artist matching, and artist-spaced
//! shuffling/interleaving for radio, discovery, and endless autoplay queues.
//!
//! Solves the common YouTube Music problem where the exact same song appears
//! multiple times under different `videoId`s (e.g. Official Audio, Official Video,
//! Topic channel, Lyrics upload, or a different re-uploader channel).

use std::collections::{HashMap, HashSet};

use crate::mock::Track;

const VERSION_MARKERS: &[(&str, &str)] = &[
    ("remix", "remix"),
    ("acoustic", "acoustic"),
    ("akustik", "acoustic"),
    ("unplugged", "acoustic"),
    ("live", "live"),
    ("canli", "live"),
    ("konser", "live"),
    ("slowed", "slowed"),
    ("reverb", "slowed"),
    ("sped up", "spedup"),
    ("speed up", "spedup"),
    ("nightcore", "spedup"),
    ("instrumental", "instrumental"),
    ("enstrumantal", "instrumental"),
    ("karaoke", "karaoke"),
    ("cover", "cover"),
    ("mashup", "mashup"),
    ("8d", "8d"),
];

const NOISE_PHRASES: &[&str] = &[
    "official music video",
    "official lyric video",
    "official lyrics video",
    "official audio",
    "official video",
    "official visualizer",
    "official mv",
    "music video",
    "lyric video",
    "lyrics video",
    "audio only",
    "full audio",
    "full song",
    "full version",
    "radio edit",
    "single version",
    "album version",
    "remastered",
    "remaster",
    "explicit",
    "clean version",
    "visualizer",
    "lyrics",
    "lyric",
    "sozleri",
    "sozlu",
    "sarki sozleri",
    "klipli",
    "klip",
    "resmi video",
    "resmi klip",
    "hd",
    "4k",
    "hq",
    "mv",
    "topic",
    "vevo",
];

const GENERIC_ARTISTS: &[&str] = &[
    "",
    "youtube music",
    "various artists",
    "unknown",
    "unknown artist",
    "topic",
];

#[derive(Debug, Clone)]
pub struct TrackSignature {
    pub id: String,
    pub title: String,
    pub alt_title: Option<String>,
    pub artist: String,
    pub version: Option<&'static str>,
    pub duration_ms: u64,
}

/// Fold Turkish & common Latin diacritics to lowercase ASCII.
pub fn fold_text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            'İ' | 'I' | 'ı' | 'i' | 'î' | 'ï' | 'í' | 'ì' => out.push('i'),
            'Ş' | 'ş' => out.push('s'),
            'Ğ' | 'ğ' => out.push('g'),
            'Ü' | 'ü' | 'û' | 'ú' | 'ù' => out.push('u'),
            'Ö' | 'ö' | 'ô' | 'ó' | 'ò' | 'õ' => out.push('o'),
            'Ç' | 'ç' => out.push('c'),
            'Â' | 'â' | 'Á' | 'á' | 'À' | 'à' | 'Ä' | 'ä' | 'Ã' | 'ã' => out.push('a'),
            'É' | 'é' | 'È' | 'è' | 'Ê' | 'ê' | 'Ë' | 'ë' => out.push('e'),
            'Ñ' | 'ñ' => out.push('n'),
            'Ý' | 'ý' | 'ÿ' => out.push('y'),
            c if c.is_ascii_uppercase() => out.push(c.to_ascii_lowercase()),
            c => out.push(c),
        }
    }
    out
}

fn keep_alnum_spaces(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last_space = true;
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_space = false;
        } else if !last_space {
            out.push(' ');
            last_space = true;
        }
    }
    out.trim().to_string()
}

fn strip_brackets(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut depth_round = 0usize;
    let mut depth_square = 0usize;
    let mut depth_curly = 0usize;
    let mut depth_cjk = 0usize;

    for ch in input.chars() {
        match ch {
            '(' => depth_round += 1,
            ')' => depth_round = depth_round.saturating_sub(1),
            '[' => depth_square += 1,
            ']' => depth_square = depth_square.saturating_sub(1),
            '{' => depth_curly += 1,
            '}' => depth_curly = depth_curly.saturating_sub(1),
            '【' => depth_cjk += 1,
            '】' => depth_cjk = depth_cjk.saturating_sub(1),
            _ if depth_round == 0 && depth_square == 0 && depth_curly == 0 && depth_cjk == 0 => {
                out.push(ch);
            }
            _ => {}
        }
    }
    out
}

fn detect_version_tag(folded_raw_title: &str) -> Option<&'static str> {
    let cleaned = keep_alnum_spaces(folded_raw_title);
    for (needle, tag) in VERSION_MARKERS {
        if contains_whole_phrase(&cleaned, needle) {
            return Some(*tag);
        }
    }
    None
}

fn contains_whole_phrase(haystack: &str, phrase: &str) -> bool {
    if haystack == phrase {
        return true;
    }
    let bytes = haystack.as_bytes();
    let p_len = phrase.len();
    let mut start = 0;
    while let Some(pos) = haystack[start..].find(phrase) {
        let abs = start + pos;
        let left_ok = abs == 0 || bytes[abs - 1] == b' ';
        let right_idx = abs + p_len;
        let right_ok = right_idx == bytes.len() || bytes[right_idx] == b' ';
        if left_ok && right_ok {
            return true;
        }
        start = abs + 1;
    }
    false
}

fn strip_feat_tail(s: &str) -> &str {
    for marker in [" feat ", " ft ", " featuring ", " prod "] {
        if let Some(idx) = s.find(marker) {
            return s[..idx].trim();
        }
    }
    s
}

fn strip_noise_phrases(s: &str) -> String {
    let mut cur = format!(" {s} ");
    for phrase in NOISE_PHRASES {
        let needle = format!(" {phrase} ");
        while cur.contains(&needle) {
            cur = cur.replace(&needle, " ");
        }
    }
    cur.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Normalize an artist name to its primary canonical form.
pub fn normalize_artist(artist: &str) -> String {
    let folded = fold_text(artist);
    let trimmed = folded
        .trim_end_matches(" - topic")
        .trim_end_matches(" topic")
        .trim_end_matches(" official")
        .trim_end_matches(" vevo");

    let primary = trimmed
        .split(['&', ',', '•', ';', '/'])
        .next()
        .unwrap_or(trimmed);
    let primary = primary.split(" ve ").next().unwrap_or(primary);
    let primary = primary.split(" feat").next().unwrap_or(primary);
    let primary = primary.split(" ft.").next().unwrap_or(primary);
    let primary = primary.split(" ft ").next().unwrap_or(primary);
    let primary = primary.split(" x ").next().unwrap_or(primary);

    keep_alnum_spaces(primary)
}

fn clean_title_fragment(raw: &str) -> String {
    let no_brackets = strip_brackets(raw);
    let folded = fold_text(&no_brackets);
    // Cut off trailing pipe / double-slash channel suffixes (e.g. "Song | Official Audio").
    let before_pipe = folded
        .split('|')
        .next()
        .unwrap_or(&folded)
        .split("//")
        .next()
        .unwrap_or(&folded);
    let alnum = keep_alnum_spaces(before_pipe);
    let no_feat = strip_feat_tail(&alnum);
    strip_noise_phrases(no_feat)
}

/// Build a canonical signature for a track for duplicate detection.
pub fn track_signature(track: &Track) -> TrackSignature {
    let folded_raw = fold_text(&track.title);
    let version = detect_version_tag(&folded_raw);
    let artist = normalize_artist(&track.artist);

    // Check for "Artist - Song" style titles common on unofficial YouTube uploads.
    let mut alt_title = None;
    for sep in [" - ", " – ", " — ", " : "] {
        if let Some((left, right)) = track.title.split_once(sep) {
            let right_clean = clean_title_fragment(right);
            let left_clean = clean_title_fragment(left);
            if right_clean.len() >= 3 && !left_clean.is_empty() {
                alt_title = Some(right_clean);
                break;
            }
        }
    }

    let mut title = clean_title_fragment(&track.title);

    // If the title starts with the artist name (e.g. "duman senden daha guzel"),
    // also provide the stripped remainder as `alt_title`.
    if alt_title.is_none() && artist.len() >= 3 {
        let prefix = format!("{artist} ");
        if let Some(rest) = title.strip_prefix(&prefix) {
            let rest = rest.trim();
            if rest.len() >= 3 {
                alt_title = Some(rest.to_string());
            }
        }
    }

    // If stripping noise emptied the title (e.g. a track literally named "Remastered"), fall back.
    if title.is_empty() {
        title = keep_alnum_spaces(&folded_raw);
    }

    TrackSignature {
        id: track.id.clone(),
        title,
        alt_title,
        artist,
        version,
        duration_ms: track.duration_ms,
    }
}

fn artists_overlap(a: &str, b: &str) -> bool {
    if a.is_empty() || b.is_empty() {
        return false;
    }
    if a == b {
        return true;
    }
    if a.len() >= 4 && b.len() >= 4 && (a.contains(b) || b.contains(a)) {
        return true;
    }
    false
}

fn is_generic_artist(a: &str) -> bool {
    GENERIC_ARTISTS.contains(&a)
}

fn titles_match(a: &TrackSignature, b: &TrackSignature) -> bool {
    let a_candidates: [&str; 2] = [a.title.as_str(), a.alt_title.as_deref().unwrap_or("")];
    let b_candidates: [&str; 2] = [b.title.as_str(), b.alt_title.as_deref().unwrap_or("")];

    for ac in a_candidates {
        if ac.len() < 3 {
            continue;
        }
        for bc in b_candidates {
            if bc.len() < 3 {
                continue;
            }
            if ac == bc {
                return true;
            }
        }
    }

    // Token-based similarity for slight punctuation/wording variations on longer titles.
    let a_words: HashSet<&str> = a
        .alt_title
        .as_deref()
        .unwrap_or(&a.title)
        .split_whitespace()
        .filter(|w| w.len() >= 2)
        .collect();
    let b_words: HashSet<&str> = b
        .alt_title
        .as_deref()
        .unwrap_or(&b.title)
        .split_whitespace()
        .filter(|w| w.len() >= 2)
        .collect();

    if a_words.len() >= 3 && b_words.len() >= 3 {
        let intersection = a_words.intersection(&b_words).count();
        let union = a_words.union(&b_words).count();
        if union > 0 && (intersection * 100 / union) >= 85 {
            return true;
        }
    }

    false
}

/// Returns `true` if two tracks represent the same song (even with different
/// YouTube video IDs or different uploader channels).
pub fn are_tracks_duplicate(a: &Track, b: &Track) -> bool {
    if a.id == b.id {
        return true;
    }
    let sa = track_signature(a);
    let sb = track_signature(b);
    signatures_duplicate(&sa, &sb)
}

pub fn signatures_duplicate(a: &TrackSignature, b: &TrackSignature) -> bool {
    if a.id == b.id {
        return true;
    }
    // Keep genuinely different versions (e.g. original vs remix/acoustic/live).
    if a.version != b.version {
        return false;
    }
    if !titles_match(a, b) {
        return false;
    }

    // Exact/near-exact title match:
    if artists_overlap(&a.artist, &b.artist) {
        return true;
    }

    // Cross-check: one track's artist appears inside the other's raw title
    // (e.g. "Duman - Senden Daha Güzel" uploaded by "Rock Kanalı" vs "Senden Daha Güzel" by "Duman").
    if !is_generic_artist(&a.artist)
        && a.artist.len() >= 3
        && contains_whole_phrase(&b.title, &a.artist)
    {
        return true;
    }
    if !is_generic_artist(&b.artist)
        && b.artist.len() >= 3
        && contains_whole_phrase(&a.title, &b.artist)
    {
        return true;
    }

    // Different YouTube uploader channels often put their own channel name as `artist`,
    // while keeping the same song title and roughly the same duration.
    let dur_compatible = if a.duration_ms > 0 && b.duration_ms > 0 {
        a.duration_ms.abs_diff(b.duration_ms) <= 45_000
    } else {
        true
    };

    if dur_compatible {
        let min_title_len = a
            .alt_title
            .as_deref()
            .unwrap_or(&a.title)
            .len()
            .min(b.alt_title.as_deref().unwrap_or(&b.title).len());
        // Require a non-trivial title (>= 4 chars) to avoid collapsing ultra-short unrelated titles.
        if min_title_len >= 4 {
            return true;
        }
    }

    false
}

/// Score metadata quality so when deduplicating within a fresh batch, we prefer
/// official/clean YouTube Music tracks over random lyric/video re-uploads.
fn metadata_quality_score(track: &Track) -> i32 {
    let mut score = 0;
    if track.thumbnail.is_some() {
        score += 10;
    }
    if !track.album.trim().is_empty() {
        score += 15;
    }
    if track.duration_ms > 0 {
        score += 5;
    }
    let lower = fold_text(&track.title);
    for noisy in ["official video", "lyrics", "lyric", "hd", "4k", "klip"] {
        if lower.contains(noisy) {
            score -= 8;
        }
    }
    if track.artist.ends_with(" - Topic") {
        score += 4;
    }
    score
}

/// Deduplicate `incoming` against `existing` (and within `incoming` itself).
/// Preserves the order of first appearance, upgrading metadata if a cleaner
/// duplicate appears later in the same incoming batch.
pub fn dedup_tracks(incoming: Vec<Track>, existing: &[Track]) -> Vec<Track> {
    let existing_sigs: Vec<TrackSignature> = existing.iter().map(track_signature).collect();
    let mut kept: Vec<Track> = Vec::with_capacity(incoming.len());
    let mut kept_sigs: Vec<TrackSignature> = Vec::with_capacity(incoming.len());

    for track in incoming {
        let sig = track_signature(&track);
        if existing_sigs.iter().any(|e| signatures_duplicate(e, &sig)) {
            continue;
        }
        if let Some(pos) = kept_sigs.iter().position(|k| signatures_duplicate(k, &sig)) {
            if metadata_quality_score(&track) > metadata_quality_score(&kept[pos]) {
                kept[pos] = track;
                kept_sigs[pos] = sig;
            }
            continue;
        }
        kept.push(track);
        kept_sigs.push(sig);
    }

    kept
}

/// Reorder a track list so the same artist doesn't play back-to-back whenever
/// other artists are available, while respecting the `last_artist` before the list.
/// Uses a max-remaining-count greedy scheduler so dominant artists are spread evenly
/// instead of clumping at the end.
pub fn interleave_by_artist(tracks: Vec<Track>, last_artist: Option<&str>) -> Vec<Track> {
    if tracks.len() <= 1 {
        return tracks;
    }

    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut remaining: Vec<(Track, String)> = tracks
        .into_iter()
        .map(|t| {
            let a = normalize_artist(&t.artist);
            if !a.is_empty() {
                *counts.entry(a.clone()).or_insert(0) += 1;
            }
            (t, a)
        })
        .collect();

    let mut result = Vec::with_capacity(remaining.len());
    let mut prev1 = last_artist.map(normalize_artist).unwrap_or_default();
    let mut prev2 = String::new();

    while !remaining.is_empty() {
        let mut best_idx: Option<usize> = None;
        let mut best_score: (usize, u8) = (0, 0);

        for (i, (_, a)) in remaining.iter().enumerate() {
            if !a.is_empty() && a == &prev1 {
                continue;
            }
            let cnt = if a.is_empty() {
                1
            } else {
                *counts.get(a).unwrap_or(&1)
            };
            let not_prev2 = if prev2.is_empty() || a != &prev2 { 1 } else { 0 };
            let score = (cnt, not_prev2);
            if best_idx.is_none() || score > best_score {
                best_idx = Some(i);
                best_score = score;
            }
        }

        let pick_idx = best_idx.unwrap_or(0);
        let (track, artist) = remaining.remove(pick_idx);
        if !artist.is_empty() {
            if let Some(c) = counts.get_mut(&artist) {
                *c = c.saturating_sub(1);
            }
        }
        prev2 = std::mem::replace(&mut prev1, artist);
        result.push(track);
    }

    result
}

/// Shuffle a slice using xorshift64 and then space out consecutive same-artist tracks.
pub fn smart_shuffle(tracks: &mut Vec<Track>, last_artist: Option<&str>) {
    if tracks.len() <= 1 {
        return;
    }
    let mut seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x9E37_79B9_7F4A_7C15);
    if seed == 0 {
        seed = 0x9E37_79B9_7F4A_7C15;
    }

    for i in (1..tracks.len()).rev() {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let j = (seed as usize) % (i + 1);
        tracks.swap(i, j);
    }

    let taken = std::mem::take(tracks);
    *tracks = interleave_by_artist(taken, last_artist);
}

/// Limit consecutive/excessive occurrences of any single artist in a discovery/radio batch
/// (useful for mixed radio so one artist doesn't dominate 80% of the queue).
pub fn cap_artist_frequency(tracks: Vec<Track>, max_per_artist: usize) -> Vec<Track> {
    if max_per_artist == 0 {
        return tracks;
    }
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut out = Vec::with_capacity(tracks.len());
    for t in tracks {
        let a = normalize_artist(&t.artist);
        if a.is_empty() {
            out.push(t);
            continue;
        }
        let c = counts.entry(a).or_insert(0);
        if *c < max_per_artist {
            *c += 1;
            out.push(t);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk(id: &str, title: &str, artist: &str, dur_s: u64) -> Track {
        Track {
            id: id.into(),
            title: title.into(),
            artist: artist.into(),
            album: String::new(),
            duration_ms: dur_s * 1000,
            thumbnail: Some("https://img".into()),
            color: "#ff0000".into(),
        }
    }

    #[test]
    fn deduplicates_different_uploaders_same_song() {
        let a = mk("id1", "Senden Daha Güzel", "Duman - Topic", 232);
        let b = mk(
            "id2",
            "Duman - Senden Daha Güzel (Official Video)",
            "Duman",
            238,
        );
        let c = mk(
            "id3",
            "Senden Daha Guzel [Lyrics]",
            "Türkçe Rock Kanalı",
            230,
        );

        assert!(are_tracks_duplicate(&a, &b));
        assert!(are_tracks_duplicate(&a, &c));
        assert!(are_tracks_duplicate(&b, &c));

        let deduped = dedup_tracks(vec![b, a, c], &[]);
        assert_eq!(deduped.len(), 1);
        // Should prefer the cleaner Topic track over the Official Video upload
        assert_eq!(deduped[0].id, "id1");
    }

    #[test]
    fn keeps_distinct_versions_like_remix_or_acoustic() {
        let orig = mk("id1", "Şımarık", "Tarkan", 235);
        let acoustic = mk("id2", "Şımarık (Acoustic)", "Tarkan", 240);
        let remix = mk("id3", "Şımarık (Club Remix)", "Tarkan", 250);

        assert!(!are_tracks_duplicate(&orig, &acoustic));
        assert!(!are_tracks_duplicate(&orig, &remix));
        assert!(!are_tracks_duplicate(&acoustic, &remix));
    }

    #[test]
    fn interleaves_artists_without_back_to_back() {
        let tracks = vec![
            mk("1", "Song 1", "Duman", 200),
            mk("2", "Song 2", "Duman", 200),
            mk("3", "Song 3", "Mor ve Ötesi", 200),
            mk("4", "Song 4", "Duman", 200),
            mk("5", "Song 5", "Teoman", 200),
        ];
        // With no prior artist, 3 Duman + 2 others can be spaced with ZERO consecutive repeats.
        let spaced = interleave_by_artist(tracks.clone(), None);
        for win in spaced.windows(2) {
            assert_ne!(
                normalize_artist(&win[0].artist),
                normalize_artist(&win[1].artist)
            );
        }

        // When previous track was Duman, first track must not be Duman.
        let spaced_after_duman = interleave_by_artist(tracks, Some("Duman"));
        assert_ne!(normalize_artist(&spaced_after_duman[0].artist), "duman");
    }
}
