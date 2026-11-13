import type { PlaylistCard, Track } from './types';

const STORAGE_KEY = 'bytm.library.v1';
const PLAYLISTS_STORAGE_KEY = 'bytm.library.playlists.v1';

function readStoredList<T>(key: string, valid: (item: unknown) => item is T): T[] {
  try {
    const raw = localStorage.getItem(key);
    if (!raw) return [];
    const parsed = JSON.parse(raw);
    return Array.isArray(parsed) ? parsed.filter(valid) : [];
  } catch {
    return [];
  }
}

function writeStoredList<T>(key: string, items: T[]) {
  try {
    localStorage.setItem(key, JSON.stringify(items));
  } catch {
    /* ignore */
  }
}

class LibraryStore {
  favorites = $state<Track[]>([]);
  savedPlaylists = $state<PlaylistCard[]>([]);

  init() {
    this.favorites = readStoredList<Track>(
      STORAGE_KEY,
      (x): x is Track => Boolean(x && typeof x === 'object' && typeof (x as Track).id === 'string')
    );
    this.savedPlaylists = readStoredList<PlaylistCard>(
      PLAYLISTS_STORAGE_KEY,
      (x): x is PlaylistCard =>
        Boolean(x && typeof x === 'object' && typeof (x as PlaylistCard).playlistId === 'string')
    );
  }

  isFavorite(id: string | undefined): boolean {
    if (!id) return false;
    return this.favorites.some((t) => t.id === id);
  }

  toggle(track: Track | null | undefined) {
    if (!track) return;
    const idx = this.favorites.findIndex((t) => t.id === track.id);
    if (idx >= 0) {
      this.favorites.splice(idx, 1);
    } else {
      this.favorites.unshift(track);
    }
    writeStoredList(STORAGE_KEY, this.favorites);
  }

  isPlaylistSaved(playlistId: string | undefined): boolean {
    if (!playlistId) return false;
    return this.savedPlaylists.some((p) => p.playlistId === playlistId);
  }

  togglePlaylist(playlist: PlaylistCard | null | undefined) {
    if (!playlist?.playlistId) return;
    const idx = this.savedPlaylists.findIndex((p) => p.playlistId === playlist.playlistId);
    if (idx >= 0) {
      this.savedPlaylists.splice(idx, 1);
    } else {
      this.savedPlaylists.unshift({ ...playlist });
    }
    writeStoredList(PLAYLISTS_STORAGE_KEY, this.savedPlaylists);
  }
}

export const library = new LibraryStore();

