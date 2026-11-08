export type ThemeMode = 'dark' | 'oled' | 'slate' | 'light' | 'system';
export type FontId = 'system' | 'geometric' | 'rounded' | 'serif' | 'mono';
export type FontScale = 'sm' | 'md' | 'lg';
export type RadiusScale = 'sharp' | 'normal' | 'round';
export type EdgeDepth = 'none' | 'subtle' | 'normal' | 'bold';
export type Density = 'compact' | 'comfortable';

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
    id: 'rounded',
    label: 'Yumuşak & Akıcı',
    sample: 'Geniş ve samimi harfler',
    stack: "'Trebuchet MS', 'Verdana', 'Segoe UI', sans-serif"
  },
  {
    id: 'serif',
    label: 'Editöryal Serif',
    sample: 'Klasik plak ve dergi stili',
    stack: "'Georgia', 'Cambria', 'Times New Roman', serif"
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
    } catch {
      /* ignore corrupt storage */
    }
    this.#media = matchMedia('(prefers-color-scheme: light)');
    this.#media.addEventListener('change', () => this.mode === 'system' && this.#apply());
    this.#apply();
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

  reset() {
    this.mode = 'dark';
    this.accent = ACCENT_PRESETS[0].value;
    this.font = 'system';
    this.fontScale = 'md';
    this.radius = 'normal';
    this.edgeDepth = 'normal';
    this.density = 'comfortable';
    this.appName = 'BetterYTMusic';
    this.#apply();
  }

  #apply() {
    const root = document.documentElement;
    root.dataset.theme = this.resolved;
    root.dataset.density = this.density;
    root.dataset.edge = this.edgeDepth;
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
        appName: this.appName
      })
    );
  }
}

export const theme = new ThemeStore();
