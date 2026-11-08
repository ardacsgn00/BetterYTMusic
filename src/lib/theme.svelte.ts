import type { View } from '#lib/types.ts';

export type ThemeMode = 'dark' | 'oled' | 'slate' | 'light' | 'system';
export type FontId =
  | 'system'
  | 'geometric'
  | 'bahnschrift'
  | 'rounded'
  | 'humanist'
  | 'gothic'
  | 'cyber'
  | 'serif'
  | 'vintage'
  | 'mono';
export type FontScale = 'sm' | 'md' | 'lg';
export type RadiusScale = 'sharp' | 'normal' | 'round';
export type EdgeDepth = 'none' | 'subtle' | 'normal' | 'bold';
export type Density = 'compact' | 'comfortable';
export type PlayButtonShape = 'circle' | 'squircle';
export type CardSize = 'compact' | 'normal' | 'large';
export type StartupView = 'last' | 'home' | 'now' | 'library' | 'search';

export const DEFAULT_NAV_ORDER: View[] = ['now', 'home', 'search', 'library', 'theme'];

export const ACCENT_PRESETS = [
  { name: 'YT Kırmızı', value: '#ff4e45' },
  { name: 'Kobalt Mavi', value: '#3a86ff' },
  { name: 'Zümrüt Mint', value: '#06d6a0' },
  { name: 'Kehribar Sarı', value: '#ffbe0b' },
  { name: 'Mor', value: '#8338ec' },
  { name: 'Mercan Pembe', value: '#ef476f' },
  { name: 'Turuncu', value: '#f77f00' },
  { name: 'Turkuaz', value: '#00b4d8' },
  { name: 'İndigo', value: '#6366f1' },
  { name: 'Limon Yeşili', value: '#84cc16' }
] as const;

export const FONT_PRESETS: { id: FontId; label: string; sample: string; stack: string }[] = [
  {
    id: 'system',
    label: 'Sistem Sans',
    sample: 'Keskin ve dengeli arayüz',
    stack: "'Segoe UI Variable', 'Segoe UI', system-ui, -apple-system, sans-serif"
  },
  {
    id: 'geometric',
    label: 'Modern Geometrik',
    sample: 'Temiz ve çağdaş çizgiler',
    stack: "'Aptos', 'Inter', 'Calibri', 'Segoe UI', sans-serif"
  },
  {
    id: 'bahnschrift',
    label: 'Bahnschrift DIN',
    sample: 'Alman endüstriyel poster stili',
    stack: "'Bahnschrift', 'DIN Next LT Pro', 'Arial Narrow', sans-serif"
  },
  {
    id: 'rounded',
    label: 'Yumuşak & Akıcı',
    sample: 'Geniş ve samimi harfler',
    stack: "'Trebuchet MS', 'Verdana', 'Segoe UI', sans-serif"
  },
  {
    id: 'humanist',
    label: 'Sıcak Hümanist',
    sample: 'Zarif ve doğal tipografi',
    stack: "'Candara', 'Corbel', 'Optima', 'Segoe UI', sans-serif"
  },
  {
    id: 'gothic',
    label: 'Century Gothic',
    sample: 'Dairesel modern albüm tarzı',
    stack: "'Century Gothic', 'Tw Cen MT', 'Segoe UI', sans-serif"
  },
  {
    id: 'cyber',
    label: 'Fütüristik Synth',
    sample: 'Elektronik & synth estetiği',
    stack: "'Eurostile', 'Century Gothic', 'Segoe UI', sans-serif"
  },
  {
    id: 'serif',
    label: 'Editöryal Serif',
    sample: 'Klasik plak ve dergi stili',
    stack: "'Georgia', 'Cambria', 'Times New Roman', serif"
  },
  {
    id: 'vintage',
    label: 'Vintage Caz Plak',
    sample: "Sıcak 70'ler caz albüm kapağı",
    stack: "'Palatino Linotype', 'Garamond', 'Book Antiqua', serif"
  },
  {
    id: 'mono',
    label: 'Stüdyo Mono',
    sample: 'Teknik ses konsolu görünümü',
    stack: "'Cascadia Code', 'Consolas', 'Courier New', monospace"
  }
];

const RADIUS_MAP: Record<RadiusScale, { sm: string; md: string; lg: string }> = {
  sharp: { sm: '6px', md: '8px', lg: '12px' },
  normal: { sm: '10px', md: '14px', lg: '20px' },
  round: { sm: '14px', md: '18px', lg: '26px' }
};

const EDGE_MAP: Record<EdgeDepth, string> = {
  none: '0px',
  subtle: '2px',
  normal: '4px',
  bold: '6px'
};

const SCALE_MAP: Record<FontScale, string> = {
  sm: '13px',
  md: '14px',
  lg: '15.5px'
};

const STORAGE_KEY = 'bytm.theme';

/** Pick black or white text for best contrast on the given hex color. */
function contrastText(hex: string): string {
  const m = /^#?([0-9a-f]{6})$/i.exec(hex);
  if (!m) return '#ffffff';
  const n = parseInt(m[1], 16);
  const [r, g, b] = [(n >> 16) & 255, (n >> 8) & 255, n & 255].map((c) => {
    const v = c / 255;
    return v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4;
  });
  const lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
  return lum > 0.45 ? '#111114' : '#ffffff';
}

class ThemeStore {
  mode = $state<ThemeMode>('dark');
  accent = $state<string>(ACCENT_PRESETS[0].value);
  font = $state<FontId>('system');
  fontScale = $state<FontScale>('md');
  radius = $state<RadiusScale>('normal');
  edgeDepth = $state<EdgeDepth>('normal');
  density = $state<Density>('comfortable');
  appName = $state<string>('BetterYTMusic');
  /** Page zoom percentage (75% – 135%, like Chrome zoom). */
  zoom = $state<number>(100);
  /** 2-col Dinle view: left panel width % (30 – 70). */
  split2Left = $state<number>(52);
  /** 3-col Dinle view: left panel width % (22 – 46) and right panel width % (22 – 46). */
  split3Left = $state<number>(32);
  split3Right = $state<number>(32);
  /** Sidebar width in px (74 = collapsed icon-only mode, or 172 – 320 expanded). Default: 74 (icon mode). */
  sidebarWidth = $state<number>(74);
  /** Custom order of navigation items in the sidebar. */
  sidebarNavOrder = $state<View[]>([...DEFAULT_NAV_ORDER]);
  /** Play button shape in the bottom PlayerBar ('circle' | 'squircle'). */
  playButtonShape = $state<PlayButtonShape>('circle');
  /** Whether the right-side duration label in PlayerBar shows remaining time (-1:15) instead of total time (3:31). */
  showRemainingTime = $state<boolean>(false);
  /** Which tab opens when the app launches ('last' | 'home' | 'now' | 'library' | 'search'). */
  startupView = $state<StartupView>('last');
  /** Last active view (excluding settings) for 'last' startup mode. */
  lastView = $state<'now' | 'home' | 'search' | 'library'>('now');
  /** Home page card size / grid column density ('compact' | 'normal' | 'large'). */
  cardSize = $state<CardSize>('normal');
  /** Hide helper subtitles under page and section headers for a cleaner look. */
  hideSubtitles = $state<boolean>(false);

  #media: MediaQueryList | null = null;

  init() {
    try {
      const saved = JSON.parse(localStorage.getItem(STORAGE_KEY) ?? '{}');
      if (saved.mode) this.mode = saved.mode;
      if (saved.accent) this.accent = saved.accent;
      if (saved.font) this.font = saved.font;
      if (saved.fontScale) this.fontScale = saved.fontScale;
      if (saved.radius) this.radius = saved.radius;
      if (saved.edgeDepth) this.edgeDepth = saved.edgeDepth;
      if (saved.density) this.density = saved.density;
      if (typeof saved.appName === 'string' && saved.appName.trim()) {
        this.appName = saved.appName;
      }
      if (typeof saved.zoom === 'number') {
        this.zoom = Math.max(75, Math.min(135, Math.round(saved.zoom)));
      }
      if (typeof saved.split2Left === 'number') {
        this.split2Left = Math.max(30, Math.min(70, saved.split2Left));
      }
      if (typeof saved.split3Left === 'number') {
        this.split3Left = Math.max(22, Math.min(46, saved.split3Left));
      }
      if (typeof saved.split3Right === 'number') {
        this.split3Right = Math.max(22, Math.min(46, saved.split3Right));
      }
      if (typeof saved.sidebarWidth === 'number' && saved.sidebarWidth !== 228) {
        this.sidebarWidth = saved.sidebarWidth <= 136 ? 74 : Math.max(172, Math.min(320, Math.round(saved.sidebarWidth)));
      } else {
        this.sidebarWidth = 74;
      }
      if (Array.isArray(saved.sidebarNavOrder)) {
        const valid = (saved.sidebarNavOrder as View[]).filter((v) => DEFAULT_NAV_ORDER.includes(v));
        const missing = DEFAULT_NAV_ORDER.filter((v) => !valid.includes(v));
        this.sidebarNavOrder = [...valid, ...missing];
      }
      if (saved.playButtonShape === 'circle' || saved.playButtonShape === 'squircle') {
        this.playButtonShape = saved.playButtonShape;
      }
      if (typeof saved.showRemainingTime === 'boolean') {
        this.showRemainingTime = saved.showRemainingTime;
      }
      if (
        saved.startupView === 'last' ||
        saved.startupView === 'home' ||
        saved.startupView === 'now' ||
        saved.startupView === 'library' ||
        saved.startupView === 'search'
      ) {
        this.startupView = saved.startupView;
      }
      if (
        saved.lastView === 'now' ||
        saved.lastView === 'home' ||
        saved.lastView === 'search' ||
        saved.lastView === 'library'
      ) {
        this.lastView = saved.lastView;
      }
      if (saved.cardSize === 'compact' || saved.cardSize === 'normal' || saved.cardSize === 'large') {
        this.cardSize = saved.cardSize;
      }
      if (typeof saved.hideSubtitles === 'boolean') {
        this.hideSubtitles = saved.hideSubtitles;
      }
    } catch {
      /* ignore corrupt storage */
    }
    this.#media = matchMedia('(prefers-color-scheme: light)');
    this.#media.addEventListener('change', () => this.mode === 'system' && this.#apply());
    this.#apply();
  }

  get sidebarCollapsed(): boolean {
    return this.sidebarWidth <= 96;
  }

  get initialView(): 'now' | 'home' | 'search' | 'library' {
    return this.startupView === 'last' ? this.lastView : this.startupView;
  }

  get resolved(): 'dark' | 'oled' | 'slate' | 'light' {
    if (this.mode !== 'system') return this.mode;
    return this.#media?.matches ? 'light' : 'dark';
  }

  setMode(mode: ThemeMode) {
    this.mode = mode;
    this.#apply();
  }

  setAccent(hex: string) {
    this.accent = hex;
    this.#apply();
  }

  setFont(font: FontId) {
    this.font = font;
    this.#apply();
  }

  setFontScale(scale: FontScale) {
    this.fontScale = scale;
    this.#apply();
  }

  setRadius(radius: RadiusScale) {
    this.radius = radius;
    this.#apply();
  }

  setEdgeDepth(depth: EdgeDepth) {
    this.edgeDepth = depth;
    this.#apply();
  }

  setDensity(density: Density) {
    this.density = density;
    this.#apply();
  }

  setAppName(name: string) {
    this.appName = name;
    this.#apply();
  }

  setZoom(zoom: number) {
    this.zoom = Math.max(75, Math.min(135, Math.round(zoom)));
    this.#apply();
  }

  stepZoom(delta: number) {
    this.setZoom(this.zoom + delta);
  }

  setSplit2Left(pct: number) {
    this.split2Left = Math.max(30, Math.min(70, Math.round(pct * 10) / 10));
    this.#apply();
  }

  setSplit3Left(pct: number) {
    const maxLeft = Math.min(46, 76 - this.split3Right);
    this.split3Left = Math.max(22, Math.min(maxLeft, Math.round(pct * 10) / 10));
    this.#apply();
  }

  setSplit3Right(pct: number) {
    const maxRight = Math.min(46, 76 - this.split3Left);
    this.split3Right = Math.max(22, Math.min(maxRight, Math.round(pct * 10) / 10));
    this.#apply();
  }

  setSidebarWidth(px: number) {
    if (px <= 136) {
      this.sidebarWidth = 74;
    } else {
      this.sidebarWidth = Math.max(172, Math.min(320, Math.round(px)));
    }
    this.#apply();
  }

  toggleSidebarCollapsed() {
    this.sidebarWidth = this.sidebarCollapsed ? 228 : 74;
    this.#apply();
  }

  setPlayButtonShape(shape: PlayButtonShape) {
    this.playButtonShape = shape;
    this.#apply();
  }

  setShowRemainingTime(val: boolean) {
    this.showRemainingTime = val;
    this.#apply();
  }

  toggleRemainingTime() {
    this.showRemainingTime = !this.showRemainingTime;
    this.#apply();
  }

  setStartupView(v: StartupView) {
    this.startupView = v;
    this.#apply();
  }

  setLastView(v: 'now' | 'home' | 'search' | 'library') {
    if (this.lastView === v) return;
    this.lastView = v;
    this.#apply();
  }

  setCardSize(size: CardSize) {
    this.cardSize = size;
    this.#apply();
  }

  setHideSubtitles(hide: boolean) {
    this.hideSubtitles = hide;
    this.#apply();
  }

  moveSidebarNavItem(id: View, direction: 'up' | 'down') {
    const idx = this.sidebarNavOrder.indexOf(id);
    if (idx < 0) return;
    const targetIdx = direction === 'up' ? idx - 1 : idx + 1;
    if (targetIdx < 0 || targetIdx >= this.sidebarNavOrder.length) return;
    const copy = [...this.sidebarNavOrder];
    const [removed] = copy.splice(idx, 1);
    copy.splice(targetIdx, 0, removed);
    this.sidebarNavOrder = copy;
    this.#apply();
  }

  resetSidebarNavOrder() {
    this.sidebarNavOrder = [...DEFAULT_NAV_ORDER];
    this.#apply();
  }

  resetSplits() {
    this.split2Left = 52;
    this.split3Left = 32;
    this.split3Right = 32;
    this.sidebarWidth = 74;
    this.#apply();
  }

  reset() {
    this.mode = 'dark';
    this.accent = ACCENT_PRESETS[0].value;
    this.font = 'system';
    this.fontScale = 'md';
    this.radius = 'normal';
    this.edgeDepth = 'normal';
    this.density = 'comfortable';
    this.appName = 'BetterYTMusic';
    this.zoom = 100;
    this.split2Left = 52;
    this.split3Left = 32;
    this.split3Right = 32;
    this.sidebarWidth = 74;
    this.sidebarNavOrder = [...DEFAULT_NAV_ORDER];
    this.playButtonShape = 'circle';
    this.showRemainingTime = false;
    this.startupView = 'last';
    this.cardSize = 'normal';
    this.hideSubtitles = false;
    this.#apply();
  }

  #apply() {
    const root = document.documentElement;
    root.dataset.theme = this.resolved;
    root.dataset.density = this.density;
    root.dataset.edge = this.edgeDepth;
    root.dataset.cardSize = this.cardSize;
    root.dataset.subtitles = this.hideSubtitles ? 'hidden' : 'visible';
    root.style.setProperty('--accent', this.accent);
    root.style.setProperty('--on-accent', contrastText(this.accent));

    const fontObj = FONT_PRESETS.find((f) => f.id === this.font) ?? FONT_PRESETS[0];
    root.style.setProperty('--font', fontObj.stack);
    root.style.setProperty('--font-size', SCALE_MAP[this.fontScale] ?? '14px');

    const r = RADIUS_MAP[this.radius] ?? RADIUS_MAP.normal;
    root.style.setProperty('--radius-sm', r.sm);
    root.style.setProperty('--radius', r.md);
    root.style.setProperty('--radius-lg', r.lg);

    root.style.setProperty('--edge-depth', EDGE_MAP[this.edgeDepth] ?? '4px');
    root.style.setProperty('--page-zoom', `${this.zoom}%`);
    root.style.setProperty('--sidebar-w', `${this.sidebarWidth}px`);

    localStorage.setItem(
      STORAGE_KEY,
      JSON.stringify({
        mode: this.mode,
        accent: this.accent,
        font: this.font,
        fontScale: this.fontScale,
        radius: this.radius,
        edgeDepth: this.edgeDepth,
        density: this.density,
        appName: this.appName,
        zoom: this.zoom,
        split2Left: this.split2Left,
        split3Left: this.split3Left,
        split3Right: this.split3Right,
        sidebarWidth: this.sidebarWidth,
        sidebarNavOrder: this.sidebarNavOrder,
        playButtonShape: this.playButtonShape,
        showRemainingTime: this.showRemainingTime,
        startupView: this.startupView,
        lastView: this.lastView,
        cardSize: this.cardSize,
        hideSubtitles: this.hideSubtitles
      })
    );
  }
}

export const theme = new ThemeStore();
