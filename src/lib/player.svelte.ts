import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type {
  AppSettings,
  ArtistCard,
  ArtistData,
  AuthStatus,
  LyricsData,
  MoodCategory,
  PlayerSnapshot,
  PlaylistCard,
  Track,
  UpdateCheckResult
} from './types';

const EMPTY: PlayerSnapshot = {
  track: null,
  queue: [],
  index: null,
  isPlaying: false,
  positionMs: 0,
  durationMs: 0,
  volume: 0.8,
  repeat: 'off',
  shuffle: false,
  sourceName: null,
  outputDevice: null
};

const OUTPUT_DEVICE_KEY = 'bytm.outputDevice';

/**
 * Frontend mirror of the Rust player.
 * Rust is the single source of truth; it emits `player:state` on every change.
 * Between events the progress is interpolated locally (rAF, only while
 * playing) so we don't need a per-frame IPC stream.
 */
class PlayerStore {
  state = $state<PlayerSnapshot>(EMPTY);
  /** Interpolated playback position in ms. */
  position = $state(0);
  auth = $state<AuthStatus>({ loggedIn: false, accountName: null });
  settings = $state<AppSettings>({
    closeToTray: true,
    launchOnStartup: false,
    startMinimized: false,
    autoplay: true,
    loudnessNormalization: true,
    resumeSession: true,
    discordRpc: true,
    discordClientId: '1569629580694655006',
    discordAppName: 'BetterYTMusic',
    discordShowCover: true,
    discordShowTime: true,
    discordClearOnPause: false,
  });
  fullscreen = $state(false);
  lyrics = $state<LyricsData | null>(null);
  lyricsLoading = $state(false);
  lyricsTrackId = $state<string | null>(null);
  outputDevices = $state<string[]>([]);
  onboardingOpen = $state(false);
  openedArtist = $state<ArtistData | null>(null);
  loadingArtist = $state(false);
  artistError = $state<string | null>(null);
  radioLoading = $state(false);
  discoverLoading = $state(false);
  toast = $state<{ message: string; id: number } | null>(null);

  #toastTimer: ReturnType<typeof setTimeout> | null = null;
  #toastSeq = 0;
  #lyricsCache = new Map<string, LyricsData | null>();
  #syncedAt = 0;
  #raf = 0;
  #unlisten: UnlistenFn | null = null;
  #unlistenAuth: UnlistenFn | null = null;

  async init() {
    if (this.#unlisten) return;
    this.#unlisten = await listen<PlayerSnapshot>('player:state', (e) => this.#apply(e.payload));
    this.#unlistenAuth = await listen<AuthStatus>('auth:changed', (e) => {
      this.auth = e.payload;
      if (e.payload.loggedIn) {
        this.onboardingOpen = false;
      }
    });
    this.#apply(await invoke<PlayerSnapshot>('player_get_state'));
    this.auth = await invoke<AuthStatus>('auth_get_status');
    if (!this.auth.loggedIn) {
      this.onboardingOpen = true;
    }
    try {
      this.settings = await invoke<AppSettings>('settings_get');
    } catch {
      /* ignore if unavailable */
    }
    try {
      await this.refreshOutputDevices();
      const savedDev = localStorage.getItem(OUTPUT_DEVICE_KEY);
      if (savedDev && savedDev.trim()) {
        await this.setOutputDevice(savedDev);
      }
    } catch {
      /* ignore audio device init errors */
    }
  }

  destroy() {
    this.#unlisten?.();
    this.#unlisten = null;
    this.#unlistenAuth?.();
    this.#unlistenAuth = null;
    cancelAnimationFrame(this.#raf);
  }

  #apply(s: PlayerSnapshot) {
    const prevId = this.state.track?.id ?? null;
    this.state = s;
    this.position = s.positionMs;
    this.#syncedAt = performance.now();
    cancelAnimationFrame(this.#raf);
    if (s.isPlaying) this.#raf = requestAnimationFrame(this.#tick);
    if (s.track && s.track.id !== prevId) {
      this.loadLyricsFor(s.track);
    } else if (!s.track) {
      this.lyrics = null;
      this.lyricsTrackId = null;
      this.lyricsLoading = false;
    }
  }

  async loadLyricsFor(track: Track) {
    const id = track.id;
    this.lyricsTrackId = id;
    if (this.#lyricsCache.has(id)) {
      this.lyrics = this.#lyricsCache.get(id) ?? null;
      this.lyricsLoading = false;
      return;
    }
    this.lyrics = null;
    this.lyricsLoading = true;
    try {
      const res = await invoke<LyricsData | null>('yt_lyrics', {
        videoId: track.id,
        title: track.title,
        artist: track.artist,
        durationMs: track.durationMs
      });
      this.#lyricsCache.set(id, res);
      if (this.lyricsTrackId === id) {
        this.lyrics = res;
      }
    } catch {
      if (this.lyricsTrackId === id) {
        this.lyrics = null;
      }
    } finally {
      if (this.lyricsTrackId === id) {
        this.lyricsLoading = false;
      }
    }
  }

  #tick = () => {
    const s = this.state;
    if (!s.isPlaying) return;
    this.position = Math.min(s.positionMs + (performance.now() - this.#syncedAt), s.durationMs);
    this.#raf = requestAnimationFrame(this.#tick);
  };

  toggle = () => invoke('player_toggle');
  play = () => invoke('player_play');
  pause = () => invoke('player_pause');
  next = () => invoke('player_next');
  previous = () => invoke('player_previous');
  cycleRepeat = () => invoke('player_cycle_repeat');
  toggleShuffle = () => invoke('player_toggle_shuffle');
  playIndex = (index: number) => invoke('player_play_index', { index });
  playTrack = (track: Track, queue?: Track[], source?: string) =>
    invoke('player_play_track', {
      track,
      queue: queue ?? null,
      source: source ?? null
    });
  setVolume = (volume: number) => invoke('player_set_volume', { volume });
  refreshOutputDevices = async () => {
    try {
      this.outputDevices = await invoke<string[]>('player_list_output_devices');
    } catch {
      this.outputDevices = [];
    }
    return this.outputDevices;
  };
  setOutputDevice = async (device: string | null) => {
    const clean = device && device.trim() ? device.trim() : null;
    try {
      if (clean) {
        localStorage.setItem(OUTPUT_DEVICE_KEY, clean);
      } else {
        localStorage.removeItem(OUTPUT_DEVICE_KEY);
      }
    } catch {
      /* ignore storage errors */
    }
    await invoke('player_set_output_device', { device: clean });
  };
  showToast = (message: string) => {
    if (this.#toastTimer) {
      clearTimeout(this.#toastTimer);
    }
    this.#toastSeq += 1;
    const id = this.#toastSeq;
    this.toast = { message, id };
    this.#toastTimer = setTimeout(() => {
      if (this.toast?.id === id) {
        this.toast = null;
      }
      this.#toastTimer = null;
    }, 2800);
  };

  search = (query: string) => invoke<Track[]>('yt_search', { query });
  searchPlaylists = (query: string) => invoke<PlaylistCard[]>('yt_search_playlists', { query });
  startRadio = async (track: Track) => {
    await invoke('player_start_radio', { track });
    this.showToast(`"${track.title}" radyosu başlatıldı`);
  };
  startArtistRadio = async (artistName: string, channelId?: string | null) => {
    const clean = artistName.trim();
    if (!clean || this.radioLoading) return;
    this.radioLoading = true;
    this.showToast(`${clean} sanatçı radyosu hazırlanıyor...`);
    try {
      const count = await invoke<number>('player_start_artist_radio', {
        artistName: clean,
        channelId: channelId ?? null
      });
      if (count > 0) {
        this.showToast(`${clean} Radyosu başlatıldı (${count} benzersiz parça)`);
      } else {
        this.showToast(`${clean} için radyo parçası bulunamadı`);
      }
    } catch (e) {
      this.showToast(`Sanatçı radyosu başlatılamadı: ${e}`);
    } finally {
      this.radioLoading = false;
    }
  };
  playNext = async (track: Track) => {
    await invoke('player_play_next', { track });
    this.showToast(`"${track.title}" sıradaki olarak eklendi`);
  };
  removeFromQueue = (index: number) => invoke('player_remove_from_queue', { index });
  clearUpcoming = async () => {
    await invoke('player_clear_upcoming');
    this.showToast('Sıradaki parçalar temizlendi');
  };
  shuffleUpcoming = async () => {
    await invoke('player_shuffle_upcoming');
    this.showToast('Sıradaki parçalar akıllıca karıştırıldı');
  };
  discoverMore = async () => {
    if (this.discoverLoading) return;
    this.discoverLoading = true;
    try {
      const added = await invoke<number>('player_discover_more');
      if (added > 0) {
        this.showToast(`${added} yeni benzer parça sıraya eklendi`);
      } else {
        this.showToast('Yeni benzersiz parça bulunamadı');
      }
    } catch {
      this.showToast('Benzer parçalar alınamadı');
    } finally {
      this.discoverLoading = false;
    }
  };
  startEndlessMix = async (tracks: Track[], source = 'Kesintisiz Keşfet') => {
    if (tracks.length === 0) return;
    if (!this.settings.autoplay) {
      await this.updateSettings({ autoplay: true });
    }
    const copy = [...tracks];
    for (let i = copy.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [copy[i], copy[j]] = [copy[j], copy[i]];
    }
    await this.playTrack(copy[0], copy, source);
    await invoke('player_shuffle_upcoming');
    this.showToast(`${source} başlatıldı · kesintisiz devam edecek`);
  };
  radio = (videoId: string) => invoke<Track[]>('yt_radio', { videoId });
  moodCategories = () => invoke<MoodCategory[]>('yt_mood_categories');
  moodPlaylists = (params: string) => invoke<PlaylistCard[]>('yt_mood_playlists', { params });
  artistDetails = (browseId: string) => invoke<ArtistData>('yt_artist_details', { browseId });
  artistSearch = (query: string) => invoke<ArtistCard | null>('yt_artist_search', { query });
  appendQueue = (tracks: Track[]) => invoke('player_append_queue', { tracks });

  openArtist = async (nameOrBrowseId: string) => {
    const raw = nameOrBrowseId.trim();
    if (!raw) return;
    this.loadingArtist = true;
    this.artistError = null;
    this.openedArtist = null;
    try {
      let browseId = raw;
      if (!browseId.startsWith('UC')) {
        const found = await this.artistSearch(browseId);
        if (found) {
          browseId = found.channelId;
        } else {
          this.artistError = `"${nameOrBrowseId}" için sanatçı profili bulunamadı.`;
          return;
        }
      }
      const data = await this.artistDetails(browseId);
      this.openedArtist = data;
    } catch (e) {
      this.artistError = `Sanatçı bilgisi alınamadı: ${e}`;
    } finally {
      this.loadingArtist = false;
    }
  };

  closeArtist = () => {
    this.openedArtist = null;
    this.artistError = null;
  };

  home = () => invoke<Track[]>('yt_home');
  homePlaylists = () => invoke<PlaylistCard[]>('yt_home_playlists');
  libraryPlaylists = () => invoke<PlaylistCard[]>('yt_library_playlists');
  playlistTracks = (playlistId: string) => invoke<Track[]>('yt_playlist_tracks', { playlistId });
  playPlaylist = async (playlistId: string, title?: string) => {
    const tracks = await this.playlistTracks(playlistId);
    if (tracks.length > 0) {
      await this.playTrack(tracks[0], tracks, title);
    }
    return tracks;
  };
  likedTracks = () => invoke<Track[]>('yt_liked');
  login = () => invoke('auth_login');
  logout = async () => {
    this.auth = await invoke<AuthStatus>('auth_logout');
  };
  openOnboarding = () => {
    this.onboardingOpen = true;
  };
  closeOnboarding = () => {
    this.onboardingOpen = false;
  };
  updateSettings = async (patch: Partial<AppSettings>) => {
    const next: AppSettings = { ...this.settings, ...patch };
    this.settings = next;
    try {
      this.settings = await invoke<AppSettings>('settings_update', { settings: next });
    } catch {
      /* keep local optimistic value */
    }
  };
  checkForUpdates = () => invoke<UpdateCheckResult>('app_check_update');
  openUrl = (url: string) => invoke('app_open_url', { url });

  seek = (positionMs: number) => {
    // Optimistic update so the bar doesn't snap back while waiting for IPC.
    this.position = positionMs;
    return invoke('player_seek', { positionMs: Math.round(positionMs) });
  };

  seekBy = (deltaMs: number) =>
    this.seek(Math.max(0, Math.min(this.position + deltaMs, this.state.durationMs)));

  setFullscreen = async (enable: boolean) => {
    this.fullscreen = enable;
    try {
      await invoke('window_set_fullscreen', { fullscreen: enable });
    } catch {
      /* ignore fullscreen IPC error when previewing outside Tauri */
    }
  };

  toggleFullscreen = () => this.setFullscreen(!this.fullscreen);
}

export const player = new PlayerStore();

