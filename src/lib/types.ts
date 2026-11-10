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

export interface MoodCategory {
  title: string;
  params: string;
  color: string;
}

export interface ArtistCard {
  channelId: string;
  name: string;
  subscribers: string | null;
  thumbnail: string | null;
}

export interface ArtistData {
  channelId: string;
  name: string;
  description: string | null;
  subscribers: string | null;
  monthlyListeners: string | null;
  thumbnail: string | null;
  topTracks: Track[];
  albums: PlaylistCard[];
  singles: PlaylistCard[];
  similarArtists: ArtistCard[];
}

export interface AppSettings {
  closeToTray: boolean;
  launchOnStartup: boolean;
  startMinimized: boolean;
  autoplay: boolean;
  discordRpc: boolean;
  discordClientId: string;
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
  outputDevice: string | null;
}
