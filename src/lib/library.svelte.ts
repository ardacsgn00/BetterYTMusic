import type { Track } from './types';

const STORAGE_KEY = 'bytm.library.v1';

class LibraryStore {
  favorites = $state<Track[]>([]);

  init() {
    try {
      const raw = localStorage.getItem(STORAGE_KEY);
      if (raw) {
        const parsed = JSON.parse(raw);
        if (Array.isArray(parsed)) this.favorites = parsed;
      }
    } catch {
      /* ignore corrupt storage */
    }
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
    this.#save();
  }

  #save() {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(this.favorites));
    } catch {
      /* ignore */
    }
  }
}

export const library = new LibraryStore();

