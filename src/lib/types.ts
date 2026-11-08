export interface Track {
  id: string;
  title: string;
  artist: string;
  album: string;
  durationMs: number;
  thumbnail: string | null;
  color: string;
}

export interface PlaylistCard {
  playlistId: string;
  title: string;
  subtitle: string;
  thumbnail: string | null;
  color: string;
}

export interface LyricLine {
  timeMs: number;
  text: string;
}

export interface LyricsData {
  synced: boolean;
  lines: LyricLine[];
  plain: string | null;
  source: string;
}

export interface AppSettings {
  closeToTray: boolean;
  launchOnStartup: boolean;
  startMinimized: boolean;
}

export interface UpdateCheckResult {
  currentVersion: string;
  latestVersion: string;
  updateAvailable: boolean;
  releaseUrl: string;
  notes: string | null;
}

export type RepeatMode = 'off' | 'all' | 'one';

export type View = 'now' | 'home' | 'search' | 'library' | 'theme';

export interface AuthStatus {
  loggedIn: boolean;
  accountName: string | null;
}

/** Mirrors `PlayerSnapshot` in src-tauri/src/player/mod.rs */
export interface PlayerSnapshot {
  track: Track | null;
  queue: Track[];
  index: number | null;
  isPlaying: boolean;
  positionMs: number;
  durationMs: number;
  volume: number;
  repeat: RepeatMode;
  shuffle: boolean;
  sourceName: string | null;
}
