<script lang="ts">
  import {
    theme,
    ACCENT_PRESETS,
    FONT_PRESETS,
    type ThemeMode,
    type FontScale,
    type RadiusScale,
    type EdgeDepth,
    type Density,
    type CardSize,
    type UiStyle,
    type StartupView
  } from '#lib/theme.svelte.ts';
  import { player } from '#lib/player.svelte.ts';
  import { mini } from '#lib/mini.svelte.ts';
  import type { UpdateCheckResult, View, SearchPlacement } from '#lib/types.ts';
  import Icon, { type IconName } from './Icon.svelte';

  const GITHUB_URL = 'https://github.com/ardacsgn00/betterytmusic';

  type SettingsTab = 'appearance' | 'layout' | 'playback' | 'system';
  type PreviewScene = 'home' | 'now' | 'mini';

  const TABS: { id: SettingsTab; label: string; icon: IconName }[] = [
    { id: 'appearance', label: 'Görünüm & Tema', icon: 'palette' },
    { id: 'layout', label: 'Düzen & Arayüz', icon: 'columns' },
    { id: 'playback', label: 'Oynatıcı & Ses', icon: 'now' },
    { id: 'system', label: 'Sistem & Entegrasyon', icon: 'settings' }
  ];

  const NAV_INFO: Record<View, { label: string; icon: IconName }> = {
    now: { label: 'Dinle', icon: 'now' },
    home: { label: 'Ana Sayfa', icon: 'home' },
    search: { label: 'Ara', icon: 'search' },
    library: { label: 'Kitaplık', icon: 'library' },
    theme: { label: 'Ayarlar', icon: 'settings' }
  };

  let activeTab = $state<SettingsTab>('appearance');
  let previewScene = $state<PreviewScene>('home');
  let checkingUpdate = $state(false);
  let updateResult = $state<UpdateCheckResult | null>(null);
  let updateError = $state<string | null>(null);

  async function runUpdateCheck() {
    checkingUpdate = true;
    updateError = null;
    try {
      updateResult = await player.checkForUpdates();
    } catch (e) {
      updateError = String(e);
    } finally {
      checkingUpdate = false;
    }
  }

  function selectTab(tab: SettingsTab) {
    activeTab = tab;
    if (tab === 'appearance') previewScene = 'home';
    else if (tab === 'layout') previewScene = 'home';
    else if (tab === 'playback') previewScene = 'now';
  }

  const modes: {
    id: ThemeMode;
    label: string;
    desc: string;
    bg: string;
    surface: string;
    text: string;
  }[] = [
    { id: 'dark', label: 'Koyu', desc: 'Kömür gri', bg: '#121215', surface: '#26262d', text: '#f2f2f5' },
    { id: 'oled', label: 'Gece Siyahı', desc: 'Tam siyah', bg: '#070709', surface: '#18181f', text: '#f5f5f8' },
    { id: 'slate', label: 'Kuzey Mavisi', desc: 'Koyu arduvaz', bg: '#0f141c', surface: '#202b3c', text: '#eef3fa' },
    { id: 'forest', label: 'Çam Ormanı', desc: 'Derin zümrüt', bg: '#0c1512', surface: '#1b2c25', text: '#edf6f1' },
    { id: 'plum', label: 'Velvet Erik', desc: 'Koyu mor', bg: '#130e19', surface: '#261d33', text: '#f3eff9' },
    { id: 'espresso', label: 'Kahve Kavurma', desc: 'Sıcak kahve', bg: '#171210', surface: '#2d241f', text: '#f6efe9' },
    { id: 'crimson', label: 'Akustik Şarap', desc: 'Derin bordo', bg: '#180d11', surface: '#2f1a22', text: '#f8eff2' },
    { id: 'nord', label: 'Buzul Nord', desc: 'Kutup mavisi', bg: '#242933', surface: '#3b4252', text: '#eceff4' },
    { id: 'light', label: 'Açık', desc: 'Sıcak krem', bg: '#f2f1ed', surface: '#ffffff', text: '#19191d' },
    { id: 'sepia', label: 'Parşömen Sepya', desc: 'Kağıt tonu', bg: '#efe8d8', surface: '#f8f3e8', text: '#2a241c' },
    { id: 'mint', label: 'Buzlu Mint', desc: 'Ferah nane', bg: '#e7f0ec', surface: '#f6fbf9', text: '#14221d' },
    { id: 'system', label: 'Sistem', desc: 'Otomatik', bg: '#121215', surface: '#f2f1ed', text: '#ff4e45' }
  ];

  const fontScales: { id: FontScale; label: string }[] = [
    { id: 'sm', label: 'Küçük (13px)' },
    { id: 'md', label: 'Normal (14px)' },
    { id: 'lg', label: 'Büyük (15.5px)' }
  ];

  const radii: { id: RadiusScale; label: string }[] = [
    { id: 'sharp', label: 'Keskin' },
    { id: 'normal', label: 'Standart' },
    { id: 'round', label: 'Yuvarlak' }
  ];

  const depths: { id: EdgeDepth; label: string }[] = [
    { id: 'none', label: 'Kapalı (0px)' },
    { id: 'subtle', label: 'İnce (2px)' },
    { id: 'normal', label: 'Standart (4px)' },
    { id: 'bold', label: 'Belirgin (6px)' }
  ];

  const densities: { id: Density; label: string }[] = [
    { id: 'comfortable', label: 'Rahat (Geniş)' },
    { id: 'compact', label: 'Sıkı (Kompakt)' }
  ];

  const uiStyles: { id: UiStyle; label: string; desc: string }[] = [
    {
      id: 'seamless',
      label: 'Kutusuz & Akıcı',
      desc: 'Kenarlıksız, modern ve göz yormayan akıcı düzen'
    },
    {
      id: 'boxed',
      label: 'Kutulu (Dokunsal)',
      desc: '2px kenarlık ve 3B alt kenar gölgeli klasik stil'
    }
  ];

  const cardSizes: { id: CardSize; label: string }[] = [
    { id: 'compact', label: 'Kompakt' },
    { id: 'normal', label: 'Standart' },
    { id: 'large', label: 'Geniş' }
  ];

  const startupViews: { id: StartupView; label: string }[] = [
    { id: 'last', label: 'Son Bıraktığım' },
    { id: 'home', label: 'Ana Sayfa' },
    { id: 'now', label: 'Dinle' },
    { id: 'library', label: 'Kitaplık' },
    { id: 'search', label: 'Ara' }
  ];

  const searchPlacements: { id: SearchPlacement; label: string }[] = [
    { id: 'tab', label: 'Kendi Sekmesinde' },
    { id: 'home', label: 'Ana Sayfada' },
    { id: 'both', label: 'İkisi De' }
  ];

  const isPreset = $derived(ACCENT_PRESETS.some((p) => p.value === theme.accent));

  // Computed visual tokens for the live preview so seamless/boxed, edgeDepth, zoom & fontScale match real app behavior.
  const simBoxed = $derived(theme.uiStyle === 'boxed');
  const simEdgePx = $derived.by(() => {
    if (!simBoxed) return '0px';
    if (theme.edgeDepth === 'none') return '0px';
    if (theme.edgeDepth === 'subtle') return '1.5px';
    if (theme.edgeDepth === 'bold') return '4.5px';
    return '3px';
  });
  const simFontFactor = $derived(theme.fontScale === 'sm' ? 0.92 : theme.fontScale === 'lg' ? 1.1 : 1);
  const simZoomScale = $derived(Math.max(0.84, Math.min(1.12, 1 + (theme.zoom - 100) * 0.0032)));
  const simSidebarWidth = $derived(
    theme.sidebarCollapsed ? 38 : Math.round(74 + ((theme.sidebarWidth - 172) / (320 - 172)) * 30)
  );
  const activeFontLabel = $derived((FONT_PRESETS.find((f) => f.id === theme.font) ?? FONT_PRESETS[0]).label);
  const activeModeLabel = $derived((modes.find((m) => m.id === theme.resolved) ?? modes[0]).label);
  const edgeLabel = $derived(
    !simBoxed
      ? 'Akıcı (0px)'
      : theme.edgeDepth === 'none'
        ? '0px'
        : theme.edgeDepth === 'subtle'
          ? '2px'
          : theme.edgeDepth === 'normal'
            ? '4px'
            : '6px'
  );
  const sceneCaption = $derived.by(() => {
    if (previewScene === 'home') {
      return 'Kart boyutu, arama konumu, sol menü, alt başlıklar, köşe ve çerçeve stili burada anında görünür.';
    }
    if (previewScene === 'now') {
      return 'Liste satır yoğunluğu, yazı boyutu, oynatıcı butonu, süre biçimi ve ses dengeleme burada anında görünür.';
    }
    return 'Küçük pencereye küçültüldüğünde kullanılan Yatay Şerit veya Kare Kart düzenini birebir gösterir.';
  });

  const simNavHighlight = $derived.by((): View => {
    if (previewScene === 'now') return 'now';
    if (previewScene === 'home') return 'home';
    return theme.startupView === 'last' ? 'now' : theme.startupView;
  });

  const MOCK_HOME_ALBUMS = [
    { title: 'Gece Yansıması', sub: 'Albüm · 2024', tone: 'var(--accent)' },
    { title: 'Synth Dalgası', sub: 'Çalma Listesi', tone: '#3a86ff' },
    { title: 'Akustik Seçki', sub: 'Mix · 32 parça', tone: '#06d6a0' },
    { title: 'Şehir Işıkları', sub: 'Single · 2025', tone: '#8338ec' }
  ];

  const MOCK_QUEUE_ROWS = [
    { title: 'Karanlıkta Parlayan', artist: 'Duman', dur: '03:45', active: true },
    { title: 'Derin Boşluk', artist: 'Motive', dur: '02:58', active: false },
    { title: 'Yıldızların Altında', artist: 'Sezen Aksu', dur: '04:12', active: false },
    { title: 'Kuzey Rüzgarı', artist: 'Mor ve Ötesi', dur: '03:20', active: false }
  ];
</script>

<section class="panel">
  <header class="head">
    <div>
      <h1>Ayarlar</h1>
      <p class="muted">Görünüm, düzen, ses deneyimi ve sistem tercihlerini kategorilere göre kolayca yönet.</p>
    </div>
    <div class="head-actions">
      <button class="btn accent" onclick={player.openOnboarding}>
        <Icon name="info" size={16} />
        Tanıtım Rehberi
      </button>
      <button class="btn" onclick={() => theme.reset()}>
        <Icon name="refresh" size={16} />
        Görünümü Sıfırla
      </button>
    </div>
  </header>

  <nav class="settings-tabs" aria-label="Ayar kategorileri">
    {#each TABS as tab (tab.id)}
      <button
        type="button"
        class="btn tab-pill"
        class:accent={activeTab === tab.id}
        aria-pressed={activeTab === tab.id}
        onclick={() => selectTab(tab.id)}
      >
        <Icon name={tab.icon} size={16} />
        <span>{tab.label}</span>
      </button>
    {/each}
  </nav>

  <div class="settings-layout">
    <div class="settings-controls-col">
      {#if activeTab === 'appearance'}
        <div class="grid">
          <div class="card block">
            <h2>Tema Paleti</h2>
            <p class="muted">Arka plan ve kart yüzeylerinin renk tonunu belirle.</p>
            <div class="modes" role="radiogroup" aria-label="Tema">
              {#each modes as m (m.id)}
                <button
                  class="btn mode-btn"
                  class:accent={theme.mode === m.id}
                  role="radio"
                  aria-checked={theme.mode === m.id}
                  onclick={() => theme.setMode(m.id)}
                >
                  <span class="m-top">
                    <span class="m-title">{m.label}</span>
                    <span class="m-palette" aria-hidden="true">
                      <span style:background-color={m.bg}></span>
                      <span style:background-color={m.surface}></span>
                      <span style:background-color={m.text}></span>
                    </span>
                  </span>
                  <span class="m-sub">{m.desc}</span>
                </button>
              {/each}
            </div>
          </div>

          <div class="card block">
            <h2>Vurgu Rengi</h2>
            <p class="muted">Düğmeler, ilerleme çubuğu ve seçili öğelerde kullanılır.</p>
            <div class="swatches" role="radiogroup" aria-label="Vurgu rengi">
              {#each ACCENT_PRESETS as p (p.value)}
                <button
                  class="swatch"
                  style:--c={p.value}
                  title={p.name}
                  role="radio"
                  aria-label={p.name}
                  aria-checked={theme.accent === p.value}
                  onclick={() => theme.setAccent(p.value)}
                >
                  {#if theme.accent === p.value}<Icon name="check" size={18} />{/if}
                </button>
              {/each}

              <label class="swatch custom" class:selected={!isPreset} style:--c={theme.accent} title="Özel renk">
                <input
                  type="color"
                  value={theme.accent}
                  aria-label="Özel vurgu rengi"
                  oninput={(e) => theme.setAccent(e.currentTarget.value)}
                />
                <span>+</span>
              </label>
            </div>
            <div class="hex-row">
              <span class="hex-dot" style:background-color={theme.accent}></span>
              <code class="hex muted">{theme.accent}</code>
            </div>
          </div>

          <div class="card block span-2">
            <h2>Arayüz Stili, Köşe & 3B Derinlik</h2>
            <p class="muted">Çerçeve stilini, köşe yuvarlaklığını ve dokunsal alt kenar kalınlığını belirle.</p>

            <div class="split-2">
              <div class="sub-group">
                <span class="sub-label muted">Arayüz Çerçeve Stili</span>
                <p class="sub-desc muted">Kutusuz sade akış ya da 2px çerçeveli klasik kutu görünümü.</p>
                <div class="opt-row">
                  {#each uiStyles as us (us.id)}
                    <button
                      class="btn"
                      class:accent={theme.uiStyle === us.id}
                      title={us.desc}
                      onclick={() => theme.setUiStyle(us.id)}
                    >
                      {us.label}
                    </button>
                  {/each}
                </div>
              </div>

              <div class="sub-group">
                <span class="sub-label muted">Köşe Stili</span>
                <p class="sub-desc muted">Kartlar, kapaklar ve düğmelerin köşe yuvarlaklığı.</p>
                <div class="opt-row">
                  {#each radii as r (r.id)}
                    <button
                      class="btn"
                      class:accent={theme.radius === r.id}
                      onclick={() => theme.setRadius(r.id)}
                    >
                      {r.label}
                    </button>
                  {/each}
                </div>
              </div>
            </div>

            <div class="sub-group">
              <span class="sub-label muted">Dokunsal Alt Kenar Kalınlığı</span>
              {#if theme.uiStyle === 'seamless'}
                <div class="inline-notice">
                  <span class="muted">
                    Şu an <strong>Kutusuz & Akıcı</strong> stil aktif olduğu için alt kenar gölgesi gizlenir. Etkisini görmek için Kutulu stili seçebilirsin.
                  </span>
                  <button
                    type="button"
                    class="btn sm"
                    onclick={() => theme.setUiStyle('boxed')}
                  >
                    Kutulu Stile Geç
                  </button>
                </div>
              {/if}
              <div class="opt-row">
                {#each depths as d (d.id)}
                  <button
                    class="btn"
                    class:accent={theme.edgeDepth === d.id}
                    onclick={() => theme.setEdgeDepth(d.id)}
                  >
                    {d.label}
                  </button>
                {/each}
              </div>
            </div>
          </div>

          <div class="card block span-2">
            <h2>Yazı Tipi (Font)</h2>
            <p class="muted">Uygulama genelinde kullanılacak tipografi ailesini seç.</p>
            <div class="font-grid" role="radiogroup" aria-label="Yazı Tipi">
              {#each FONT_PRESETS as f (f.id)}
                <button
                  class="btn font-card"
                  class:accent={theme.font === f.id}
                  style:font-family={f.stack}
                  role="radio"
                  aria-checked={theme.font === f.id}
                  onclick={() => theme.setFont(f.id)}
                >
                  <span class="f-name">{f.label}</span>
                  <span class="f-sample">{f.sample} · 01:24 / 03:45</span>
                </button>
              {/each}
            </div>
          </div>
        </div>
      {:else if activeTab === 'layout'}
        <div class="grid">
          <div class="card block">
            <h2>Ölçek (Zoom), Yazı & Yoğunluk</h2>
            <p class="muted">Genel yakınlaştırma oranını, yazı boyutunu ve liste satır sıklığını ayarla.</p>

            <div class="sub-group">
              <span class="sub-label muted">Sayfa Ölçeği (%{theme.zoom})</span>
              <div class="opt-row">
                <button
                  class="btn"
                  disabled={theme.zoom <= 75}
                  onclick={() => theme.stepZoom(-5)}
                >
                  - %5
                </button>
                {#each [80, 90, 100, 110, 125] as z (z)}
                  <button
                    class="btn"
                    class:accent={theme.zoom === z}
                    onclick={() => theme.setZoom(z)}
                  >
                    %{z}
                  </button>
                {/each}
                <button
                  class="btn"
                  disabled={theme.zoom >= 135}
                  onclick={() => theme.stepZoom(5)}
                >
                  + %5
                </button>
              </div>
            </div>

            <div class="sub-group">
              <span class="sub-label muted">Yazı / Metin Büyüklüğü</span>
              <div class="opt-row">
                {#each fontScales as s (s.id)}
                  <button
                    class="btn"
                    class:accent={theme.fontScale === s.id}
                    onclick={() => {
                      theme.setFontScale(s.id);
                      previewScene = 'now';
                    }}
                  >
                    {s.label}
                  </button>
                {/each}
              </div>
            </div>

            <div class="sub-group">
              <span class="sub-label muted">Şarkı Listesi Satır Aralığı</span>
              <p class="sub-desc muted">Sıradaki parçalar ve listeler arasındaki dikey boşluk.</p>
              <div class="opt-row">
                {#each densities as d (d.id)}
                  <button
                    class="btn"
                    class:accent={theme.density === d.id}
                    onclick={() => {
                      theme.setDensity(d.id);
                      previewScene = 'now';
                    }}
                  >
                    {d.label}
                  </button>
                {/each}
              </div>
            </div>

            <div class="sub-group">
              <span class="sub-label muted">Dinle Ekranı Panel Genişlikleri</span>
              <p class="sub-desc muted">Kapak, sözler ve sıradaki panellerini sürükleyip bozduysan sıfırlar.</p>
              <div class="opt-row">
                <button class="btn" onclick={() => theme.resetSplits()}>
                  <Icon name="refresh" size={15} />
                  Panel Genişliklerini Sıfırla
                </button>
              </div>
            </div>
          </div>

          <div class="card block">
            <h2>Ana Sayfa & Arama Düzeni</h2>
            <p class="muted">Açılış sekmesini, kart büyüklüğünü, arama konumunu ve alt yazıları ayarla.</p>

            <div class="sub-group">
              <span class="sub-label muted">Açılışta Başlatılacak Sekme</span>
              <div class="opt-row">
                {#each startupViews as sv (sv.id)}
                  <button
                    class="btn"
                    class:accent={theme.startupView === sv.id}
                    onclick={() => theme.setStartupView(sv.id)}
                  >
                    {sv.label}
                  </button>
                {/each}
              </div>
            </div>

            <div class="sub-group">
              <span class="sub-label muted">Ana Sayfa Kart Boyutu</span>
              <div class="opt-row">
                {#each cardSizes as cs (cs.id)}
                  <button
                    class="btn"
                    class:accent={theme.cardSize === cs.id}
                    onclick={() => {
                      theme.setCardSize(cs.id);
                      previewScene = 'home';
                    }}
                  >
                    {cs.label}
                  </button>
                {/each}
              </div>
            </div>

            <div class="sub-group">
              <span class="sub-label muted">Arama Özelliği Konumu</span>
              <div class="opt-row">
                {#each searchPlacements as sp (sp.id)}
                  <button
                    class="btn"
                    class:accent={theme.searchPlacement === sp.id}
                    onclick={() => {
                      theme.setSearchPlacement(sp.id);
                      previewScene = 'home';
                    }}
                  >
                    {sp.label}
                  </button>
                {/each}
              </div>
              <p class="sub-desc muted">
                {#if theme.searchPlacement === 'tab'}
                  Arama yalnızca sol menüdeki "Ara" sekmesinde yer alır.
                {:else if theme.searchPlacement === 'home'}
                  Arama ana sayfada üst kısımda yer alır; sol menüdeki sekme gizlenir.
                {:else}
                  Arama hem ana sayfada hem de sol menüde aktif olur.
                {/if}
              </p>
            </div>

            <div class="sub-group">
              <span class="sub-label muted">Bölüm Açıklama Alt Yazıları</span>
              <div class="opt-row">
                <button
                  class="btn"
                  class:accent={!theme.hideSubtitles}
                  onclick={() => {
                    theme.setHideSubtitles(false);
                    previewScene = 'home';
                  }}
                >
                  Göster
                </button>
                <button
                  class="btn"
                  class:accent={theme.hideSubtitles}
                  onclick={() => {
                    theme.setHideSubtitles(true);
                    previewScene = 'home';
                  }}
                >
                  Gizle (Sade Başlıklar)
                </button>
              </div>
            </div>
          </div>

          <div class="card block span-2">
            <h2>Sol Menü Modu & Sıralaması</h2>
            <p class="muted">
              Kenar çubuğu genişliği ({theme.sidebarCollapsed ? 'İkon Modu · 74px' : `${theme.sidebarWidth}px`}, kenardan da sürüklenebilir) ve sekme sırası.
            </p>

            <div class="opt-row">
              <button
                class="btn"
                class:accent={theme.sidebarCollapsed}
                onclick={() => theme.setSidebarWidth(74)}
              >
                Sadece İkon Modu (74px)
              </button>
              <button
                class="btn"
                class:accent={!theme.sidebarCollapsed}
                onclick={() => theme.setSidebarWidth(228)}
              >
                Tam Menü (228px)
              </button>
              <button class="btn reset-link-btn" onclick={() => theme.resetSidebarNavOrder()}>
                <Icon name="refresh" size={14} />
                Sıralamayı Sıfırla
              </button>
            </div>

            <div class="nav-order-grid">
              {#each theme.sidebarNavOrder as navId, idx (navId)}
                <div class="nav-order-item">
                  <div class="nav-order-left">
                    <span class="order-idx">{idx + 1}</span>
                    <span class="order-icon"><Icon name={NAV_INFO[navId].icon} size={16} /></span>
                    <span class="order-label">{NAV_INFO[navId].label}</span>
                  </div>
                  <div class="nav-order-actions">
                    <button
                      class="btn icon-btn sm"
                      disabled={idx === 0}
                      title="Yukarı Taşı"
                      aria-label="{NAV_INFO[navId].label} Yukarı Taşı"
                      onclick={() => theme.moveSidebarNavItem(navId, 'up')}
                    >
                      <Icon name="up" size={13} />
                    </button>
                    <button
                      class="btn icon-btn sm"
                      disabled={idx === theme.sidebarNavOrder.length - 1}
                      title="Aşağı Taşı"
                      aria-label="{NAV_INFO[navId].label} Aşağı Taşı"
                      onclick={() => theme.moveSidebarNavItem(navId, 'down')}
                    >
                      <Icon name="down" size={13} />
                    </button>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        </div>
      {:else if activeTab === 'playback'}
        <div class="grid">
          <div class="card block">
            <h2>Alt Oynatıcı Çubuğu & Başlık</h2>
            <p class="muted">Oynat düğmesi şeklini, süre görünümünü ve sol üstteki uygulama adını seç.</p>

            <div class="sub-group">
              <span class="sub-label muted">Oynat Düğmesi Stili</span>
              <div class="opt-row">
                <button
                  class="btn"
                  class:accent={theme.playButtonShape === 'circle'}
                  onclick={() => {
                    theme.setPlayButtonShape('circle');
                    previewScene = 'now';
                  }}
                >
                  Yuvarlak Daire
                </button>
                <button
                  class="btn"
                  class:accent={theme.playButtonShape === 'squircle'}
                  onclick={() => {
                    theme.setPlayButtonShape('squircle');
                    previewScene = 'now';
                  }}
                >
                  Yumuşatılmış Kare
                </button>
              </div>
            </div>

            <div class="sub-group">
              <span class="sub-label muted">İlerleme Çubuğu Sağ Süre Gösterimi</span>
              <div class="opt-row">
                <button
                  class="btn"
                  class:accent={!theme.showRemainingTime}
                  onclick={() => {
                    theme.setShowRemainingTime(false);
                    previewScene = 'now';
                  }}
                >
                  Toplam Süre (03:45)
                </button>
                <button
                  class="btn"
                  class:accent={theme.showRemainingTime}
                  onclick={() => {
                    theme.setShowRemainingTime(true);
                    previewScene = 'now';
                  }}
                >
                  Kalan Süre (-02:21)
                </button>
              </div>
            </div>

            <div class="sub-group">
              <span class="sub-label muted">Uygulama Başlığı</span>
              <div class="name-row">
                <input
                  class="text-input"
                  type="text"
                  maxlength="28"
                  placeholder="BetterYTMusic"
                  value={theme.appName}
                  oninput={(e) => theme.setAppName(e.currentTarget.value)}
                />
                <button
                  class="btn"
                  onclick={() => theme.setAppName('BetterYTMusic')}
                  disabled={theme.appName === 'BetterYTMusic'}
                >
                  Sıfırla
                </button>
              </div>
            </div>
          </div>

          <div class="card block">
            <h2>Mini Oynatıcı Düzeni</h2>
            <p class="muted">Her zaman üstte kalabilen küçük pencere tasarımını seç ve sağdaki önizlemede gör.</p>
            <div class="mini-layout-row">
              <button
                class="btn mini-layout-btn"
                class:accent={theme.miniLayout === 'strip'}
                onclick={() => {
                  mini.setLayout('strip');
                  previewScene = 'mini';
                }}
              >
                <span class="ml-preview ml-strip" aria-hidden="true">
                  <span class="ml-cover"></span>
                  <span class="ml-lines"><span></span><span></span></span>
                  <span class="ml-dot"></span>
                </span>
                <span class="ml-label">Yatay Şerit</span>
              </button>
              <button
                class="btn mini-layout-btn"
                class:accent={theme.miniLayout === 'square'}
                onclick={() => {
                  mini.setLayout('square');
                  previewScene = 'mini';
                }}
              >
                <span class="ml-preview ml-square" aria-hidden="true">
                  <span class="ml-panel"><span class="ml-dot"></span></span>
                </span>
                <span class="ml-label">Kare Kart</span>
              </button>
            </div>
            <div class="sub-group">
              <div class="opt-row">
                <button class="btn accent" onclick={mini.enter}>
                  <Icon name="mini" size={16} />
                  Mini Oynatıcıyı Aç
                </button>
                <button class="btn" onclick={() => player.setFullscreen(true)}>
                  <Icon name="maximize" size={16} />
                  Tam Ekranı Aç (F)
                </button>
              </div>
            </div>
          </div>

          <div class="card block span-2">
            <h2>Ses Motoru & Dinleme Deneyimi</h2>
            <p class="muted">Ses yüksekliği dengeleme, oturum kurtarma ve kuyruk devamlılığı seçenekleri.</p>
            <div class="toggle-list two-col">
              <div class="toggle-row">
                <div class="toggle-text">
                  <span class="t-title">Ses Seviyesi Dengeleme (Loudness Normalization)</span>
                  <span class="t-desc muted">YouTube Music ses yüksekliği verisini kullanarak parçalar arasındaki ani ses farklarını dengeler.</span>
                </div>
                <button
                  class="btn toggle-btn"
                  class:accent={player.settings.loudnessNormalization}
                  onclick={() =>
                    player.updateSettings({ loudnessNormalization: !player.settings.loudnessNormalization })}
                >
                  {player.settings.loudnessNormalization ? 'Açık' : 'Kapalı'}
                </button>
              </div>

              <div class="toggle-row">
                <div class="toggle-text">
                  <span class="t-title">Kaldığın Yerden Devam Et</span>
                  <span class="t-desc muted">Kuyruğu ve şarkı konumunu kaydeder; bir sonraki açılışta aynı noktadan hazırlar.</span>
                </div>
                <button
                  class="btn toggle-btn"
                  class:accent={player.settings.resumeSession}
                  onclick={() => player.updateSettings({ resumeSession: !player.settings.resumeSession })}
                >
                  {player.settings.resumeSession ? 'Açık' : 'Kapalı'}
                </button>
              </div>

              <div class="toggle-row span-2">
                <div class="toggle-text">
                  <span class="t-title">Kuyruk Sonu Otomatik Çalma (Autoplay)</span>
                  <span class="t-desc muted">Sıradaki şarkılar bittiğinde YouTube Music benzer parçalarla kesintisiz çalmaya devam eder.</span>
                </div>
                <button
                  class="btn toggle-btn"
                  class:accent={player.settings.autoplay}
                  onclick={() => player.updateSettings({ autoplay: !player.settings.autoplay })}
                >
                  {player.settings.autoplay ? 'Açık' : 'Kapalı'}
                </button>
              </div>
            </div>
          </div>
        </div>
      {:else}
        <div class="grid">
          <div class="card block">
            <h2>Sistem & Başlangıç Ayarları</h2>
            <p class="muted">Pencere kapatma davranışını ve işletim sistemi açılış tercihlerini yönet.</p>
            <div class="toggle-list">
              <div class="toggle-row">
                <div class="toggle-text">
                  <span class="t-title">Kapatınca Sistem Tepsisinde (Tray) Kal</span>
                  <span class="t-desc muted">X düğmesine basıldığında müzik arka planda çalmaya devam eder.</span>
                </div>
                <button
                  class="btn toggle-btn"
                  class:accent={player.settings.closeToTray}
                  onclick={() => player.updateSettings({ closeToTray: !player.settings.closeToTray })}
                >
                  {player.settings.closeToTray ? 'Açık' : 'Kapalı'}
                </button>
              </div>

              <div class="toggle-row">
                <div class="toggle-text">
                  <span class="t-title">Sistem Açılışında Otomatik Başlat</span>
                  <span class="t-desc muted">Bilgisayar açıldığında uygulamayı otomatik olarak başlat.</span>
                </div>
                <button
                  class="btn toggle-btn"
                  class:accent={player.settings.launchOnStartup}
                  onclick={() => player.updateSettings({ launchOnStartup: !player.settings.launchOnStartup })}
                >
                  {player.settings.launchOnStartup ? 'Açık' : 'Kapalı'}
                </button>
              </div>

              <div class="toggle-row">
                <div class="toggle-text">
                  <span class="t-title">Başlangıçta Tepside Küçültülmüş Başlat</span>
                  <span class="t-desc muted">Otomatik başladığında pencereyi açmadan sistem tepsisinde beklet.</span>
                </div>
                <button
                  class="btn toggle-btn"
                  class:accent={player.settings.startMinimized}
                  disabled={!player.settings.launchOnStartup}
                  onclick={() => player.updateSettings({ startMinimized: !player.settings.startMinimized })}
                >
                  {player.settings.startMinimized ? 'Açık' : 'Kapalı'}
                </button>
              </div>
            </div>
          </div>

          <div class="card block discord-block">
            <div class="discord-block-head">
              <div>
                <h2>Discord Zengin Varlık (Rich Presence)</h2>
                <p class="muted">Discord profilinde müzik durumunu göster ve canlı önizle.</p>
              </div>
              <button
                class="btn toggle-btn"
                class:accent={player.settings.discordRpc ?? true}
                onclick={() => player.updateSettings({ discordRpc: !(player.settings.discordRpc ?? true) })}
              >
                {(player.settings.discordRpc ?? true) ? 'Açık' : 'Kapalı'}
              </button>
            </div>

            {#if player.settings.discordRpc ?? true}
              <div class="discord-sim-card">
                <div class="d-sim-user-bar">
                  <div class="d-sim-avatar">
                    {#if player.state.track?.thumbnail && (player.settings.discordShowCover ?? true)}
                      <img src={player.state.track.thumbnail} alt="" class="d-sim-avatar-img" />
                    {:else}
                      <div class="d-sim-avatar-fallback">
                        <Icon name="now" size={16} />
                      </div>
                    {/if}
                    <span class="d-sim-status-dot"></span>
                  </div>
                  <div class="d-sim-user-meta">
                    <div class="d-sim-username">
                      {player.auth.accountName || 'BetterYTMusic'}
                      <span class="d-sim-tag">#0001</span>
                    </div>
                    <span class="d-sim-activity-label">BİR ŞEY DİNLİYOR</span>
                  </div>
                </div>

                <div class="d-sim-activity-box">
                  <div class="d-sim-act-main">
                    {#if player.settings.discordShowCover ?? true}
                      <div class="d-sim-cover-wrap">
                        {#if player.state.track?.thumbnail}
                          <img src={player.state.track.thumbnail} alt="" class="d-sim-cover" />
                        {:else}
                          <div class="d-sim-cover-fallback">
                            <Icon name="now" size={26} />
                          </div>
                        {/if}
                        <div class="d-sim-badge-dot">
                          <Icon name={player.state.isPlaying ? 'play' : 'pause'} size={11} />
                        </div>
                      </div>
                    {/if}

                    <div class="d-sim-meta">
                      <span class="d-sim-app-title">{player.settings.discordAppName || 'BetterYTMusic'}</span>
                      <span class="d-sim-song truncate">
                        {player.state.track?.title || 'Örnek Parça'}
                      </span>
                      <span class="d-sim-artist truncate muted">
                        {#if !player.state.isPlaying && !(player.settings.discordClearOnPause ?? false)}
                          Duraklatıldı · {player.state.track?.artist || 'Sanatçı'}
                        {:else}
                          {player.state.track?.artist || 'Sanatçı'}
                        {/if}
                      </span>

                      {#if player.settings.discordShowTime ?? true}
                        {@const curSec = Math.floor(player.position / 1000)}
                        {@const durSec = Math.max(1, Math.floor((player.state.track?.durationMs || 180000) / 1000))}
                        {@const pct = Math.min(100, Math.max(0, (curSec / durSec) * 100))}
                        <div class="d-sim-progress-wrap">
                          <div class="d-sim-bar-bg">
                            <div class="d-sim-bar-fill" style:width="{pct}%"></div>
                          </div>
                          <div class="d-sim-time-text">
                            <span>{Math.floor(curSec / 60)}:{String(curSec % 60).padStart(2, '0')}</span>
                            <span>{Math.floor(durSec / 60)}:{String(durSec % 60).padStart(2, '0')}</span>
                          </div>
                        </div>
                      {/if}
                    </div>
                  </div>
                </div>
              </div>

              <div class="d-custom-list">
                <div class="toggle-row d-app-name-row">
                  <div class="toggle-text">
                    <span class="t-title">Discord Başlığı (Uygulama Adı)</span>
                    <span class="t-desc muted">Profil kartında görünecek başlık adı.</span>
                  </div>
                  <div class="d-app-name-wrap">
                    <input
                      type="text"
                      class="discord-input"
                      maxlength="32"
                      placeholder="BetterYTMusic"
                      value={player.settings.discordAppName || 'BetterYTMusic'}
                      onchange={(e) => player.updateSettings({ discordAppName: e.currentTarget.value.trim() || 'BetterYTMusic' })}
                    />
                    <button
                      type="button"
                      class="btn sm"
                      title="Varsayılan İsme Dönüştür"
                      onclick={() => player.updateSettings({ discordAppName: 'BetterYTMusic' })}
                      disabled={(player.settings.discordAppName || 'BetterYTMusic') === 'BetterYTMusic'}
                    >
                      Sıfırla
                    </button>
                  </div>
                </div>

                <div class="toggle-row">
                  <div class="toggle-text">
                    <span class="t-title">Albüm Kapağı</span>
                    <span class="t-desc muted">Profil kartında şarkı kapağını göster.</span>
                  </div>
                  <button
                    class="btn toggle-btn"
                    class:accent={player.settings.discordShowCover ?? true}
                    onclick={() => player.updateSettings({ discordShowCover: !(player.settings.discordShowCover ?? true) })}
                  >
                    {(player.settings.discordShowCover ?? true) ? 'Açık' : 'Kapalı'}
                  </button>
                </div>

                <div class="toggle-row">
                  <div class="toggle-text">
                    <span class="t-title">Canlı İlerleme Çubuğu</span>
                    <span class="t-desc muted">Şarkının anlık süresini Discord'da göster.</span>
                  </div>
                  <button
                    class="btn toggle-btn"
                    class:accent={player.settings.discordShowTime ?? true}
                    onclick={() => player.updateSettings({ discordShowTime: !(player.settings.discordShowTime ?? true) })}
                  >
                    {(player.settings.discordShowTime ?? true) ? 'Açık' : 'Kapalı'}
                  </button>
                </div>

                <div class="toggle-row">
                  <div class="toggle-text">
                    <span class="t-title">Duraklatılınca Durumu Temizle</span>
                    <span class="t-desc muted">Müzik durduğunda "Duraklatıldı" yerine durumu kaldır.</span>
                  </div>
                  <button
                    class="btn toggle-btn"
                    class:accent={player.settings.discordClearOnPause ?? false}
                    onclick={() => player.updateSettings({ discordClearOnPause: !(player.settings.discordClearOnPause ?? false) })}
                  >
                    {(player.settings.discordClearOnPause ?? false) ? 'Açık' : 'Kapalı'}
                  </button>
                </div>
              </div>
            {:else}
              <div class="d-disabled-msg">
                <p class="muted">Discord Zengin Varlık özelliği kapalı. Açıldığında dinlediğin parçalar Discord profilinde görünür.</p>
              </div>
            {/if}
          </div>

          <div class="card block span-2">
            <h2>Güncelleme & Hakkında</h2>
            <p class="muted">Uygulama sürümü, geliştirici bilgileri ve GitHub güncelleme kontrolü.</p>

            <div class="about-grid">
              <div class="about-item">
                <span class="sub-label muted">Geliştirici</span>
                <span class="about-val">Arda Coşgun</span>
                <span class="about-sub muted">@ardacsgn00</span>
              </div>

              <div class="about-item">
                <span class="sub-label muted">Kurulu Sürüm</span>
                <span class="about-val">v{__APP_VERSION__}</span>
              </div>

              <div class="about-item">
                <span class="sub-label muted">Kaynak Kod & Proje Sayfası</span>
                <span class="about-val truncate">ardacsgn00/betterytmusic</span>
                <button class="btn github-btn" onclick={() => player.openUrl(GITHUB_URL)}>
                  <Icon name="github" size={16} />
                  GitHub'da Aç
                </button>
              </div>

              <div class="about-item">
                <span class="sub-label muted">Başlangıç & Tanıtım</span>
                <span class="about-val">Özellik Rehberi</span>
                <button class="btn tour-btn" onclick={player.openOnboarding}>
                  <Icon name="info" size={16} />
                  Rehberi Başlat
                </button>
              </div>
            </div>

            <div class="update-bar">
              <div class="update-status">
                {#if checkingUpdate}
                  <span class="u-title">GitHub üzerinden güncellemeler denetleniyor...</span>
                  <span class="u-desc muted">Lütfen bekle.</span>
                {:else if updateError}
                  <span class="u-title">Güncelleme denetimi başarısız oldu</span>
                  <span class="u-desc muted">{updateError}</span>
                {:else if updateResult}
                  {#if updateResult.updateAvailable}
                    <span class="u-title accent-txt">
                      Yeni sürüm mevcut: v{updateResult.latestVersion} (Kurulu: v{updateResult.currentVersion})
                    </span>
                    <span class="u-desc muted">
                      {updateResult.notes ?? 'Yeni sürümü indirmek için GitHub sürüm sayfasını açabilirsin.'}
                    </span>
                  {:else}
                    <span class="u-title ok-txt">
                      Güncel sürümü kullanıyorsun (v{updateResult.currentVersion})
                    </span>
                    <span class="u-desc muted">GitHub üzerindeki son sürüm: v{updateResult.latestVersion}</span>
                  {/if}
                {:else}
                  <span class="u-title">Güncelleme Denetimi</span>
                  <span class="u-desc muted">GitHub deposundaki en son sürümü kontrol etmek için düğmeye tıkla.</span>
                {/if}
              </div>

              <div class="update-actions">
                {#if updateResult?.updateAvailable}
                  <button
                    class="btn accent"
                    onclick={() => player.openUrl(updateResult?.releaseUrl ?? GITHUB_URL)}
                  >
                    <Icon name="github" size={16} />
                    Yeni Sürümü İndir
                  </button>
                {/if}
                <button class="btn" class:accent={!updateResult?.updateAvailable} disabled={checkingUpdate} onclick={runUpdateCheck}>
                  <Icon name="refresh" size={16} />
                  {checkingUpdate ? 'Denetleniyor...' : 'Güncellemeleri Denetle'}
                </button>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Right / Top Live Simulation Inspector -->
    <aside class="simulation-col">
      <div class="card block preview-panel">
        <div class="sim-top-bar">
          <div class="sim-header">
            <div class="sim-title-wrap">
              <span class="sim-live-dot" style:background-color={theme.accent}></span>
              <span class="sim-title">Canlı Arayüz Simülasyonu</span>
            </div>
            <span class="sim-sub muted">{sceneCaption}</span>
          </div>

          <div class="sim-scene-tabs" role="tablist" aria-label="Simülasyon görünümü">
            <button
              type="button"
              role="tab"
              aria-selected={previewScene === 'home'}
              class="sim-scene-btn"
              class:active={previewScene === 'home'}
              onclick={() => (previewScene = 'home')}
            >
              Ana Sayfa
            </button>
            <button
              type="button"
              role="tab"
              aria-selected={previewScene === 'now'}
              class="sim-scene-btn"
              class:active={previewScene === 'now'}
              onclick={() => (previewScene = 'now')}
            >
              Dinle & Sıra
            </button>
            <button
              type="button"
              role="tab"
              aria-selected={previewScene === 'mini'}
              class="sim-scene-btn"
              class:active={previewScene === 'mini'}
              onclick={() => (previewScene = 'mini')}
            >
              Mini Oynatıcı
            </button>
          </div>
        </div>

        <!-- Bounded Fixed-Proportion Mini Application Window -->
        <div
          class="mini-window"
          class:is-boxed={simBoxed}
          class:is-seamless={!simBoxed}
          style:--sim-edge={simEdgePx}
          style:--sim-font-factor={simFontFactor}
          style:--sim-zoom={simZoomScale}
        >
          {#if previewScene === 'mini'}
            <!-- Simulated Desktop with Mini Player Window -->
            <div class="mw-desktop">
              <div class="mw-desktop-bar">
                <span class="mw-desktop-badge">Masaüstü Üstü · Her Zaman Üstte</span>
                <span class="mw-zoom-badge">
                  {theme.miniLayout === 'square' ? 'Kare Kart' : 'Yatay Şerit'}
                </span>
              </div>

              {#if theme.miniLayout === 'strip'}
                <div class="mw-mini-strip">
                  <div class="mw-mini-thumb" style:background-color={theme.accent}>
                    <Icon name="now" size={16} />
                  </div>
                  <div class="mw-mini-meta">
                    <span class="mw-mini-title truncate">Karanlıkta Parlayan</span>
                    <span class="mw-mini-artist truncate muted">Duman</span>
                    <div class="mw-mini-prog"><div style:width="48%"></div></div>
                  </div>
                  <div class="mw-mini-ctrls">
                    <span class="mw-ctrl-btn"><Icon name="prev" size={12} /></span>
                    <span class="mw-pb-play" class:squircle={theme.playButtonShape === 'squircle'}>
                      <Icon name="pause" size={10} />
                    </span>
                    <span class="mw-ctrl-btn"><Icon name="next" size={12} /></span>
                  </div>
                </div>
              {:else}
                <div class="mw-mini-square">
                  <div class="mw-mini-sq-art" style:background-color={theme.accent}>
                    <Icon name="disc" size={34} />
                  </div>
                  <div class="mw-mini-sq-footer">
                    <div class="mw-mini-meta">
                      <span class="mw-mini-title truncate">Karanlıkta Parlayan</span>
                      <span class="mw-mini-artist truncate muted">Duman</span>
                    </div>
                    <div class="mw-mini-ctrls">
                      <span class="mw-ctrl-btn"><Icon name="prev" size={11} /></span>
                      <span class="mw-pb-play" class:squircle={theme.playButtonShape === 'squircle'}>
                        <Icon name="play" size={10} />
                      </span>
                      <span class="mw-ctrl-btn"><Icon name="next" size={11} /></span>
                    </div>
                  </div>
                </div>
              {/if}
            </div>
          {:else}
            <div class="mw-titlebar">
              <div class="mw-controls">
                <span class="mw-dot close"></span>
                <span class="mw-dot min"></span>
                <span class="mw-dot max"></span>
              </div>
              <span class="mw-title truncate">{theme.appName.trim() || 'BetterYTMusic'}</span>
              <div class="mw-tb-right">
                {#if player.settings.loudnessNormalization}
                  <span class="mw-norm-badge" title="Ses Dengeleme Açık">NORM</span>
                {/if}
                <span class="mw-zoom-badge">%{theme.zoom}</span>
              </div>
            </div>

            <div class="mw-body">
              <div
                class="mw-sidebar"
                class:collapsed={theme.sidebarCollapsed}
                style:width="{simSidebarWidth}px"
              >
                <div class="mw-brand">
                  <span class="mw-logo"><Icon name="play" size={10} /></span>
                  {#if !theme.sidebarCollapsed}
                    <span class="mw-brand-text truncate">{theme.appName.trim() || 'BetterYTMusic'}</span>
                  {/if}
                </div>
                <div class="mw-nav-items">
                  {#each theme.sidebarNavOrder as navId (navId)}
                    {#if !(navId === 'search' && theme.searchPlacement === 'home')}
                      {@const isStart = theme.startupView === navId || (theme.startupView === 'last' && navId === 'now')}
                      <div class="mw-nav-item" class:active={simNavHighlight === navId}>
                        <Icon name={NAV_INFO[navId].icon} size={11} />
                        {#if !theme.sidebarCollapsed}
                          <span class="truncate">{NAV_INFO[navId].label}</span>
                          {#if isStart}
                            <span class="mw-start-dot" title="Başlangıç Sekmesi"></span>
                          {/if}
                        {/if}
                      </div>
                    {/if}
                  {/each}
                </div>
              </div>

              <div class="mw-content">
                <div class="mw-zoom-stage">
                  {#if previewScene === 'home'}
                    <div class="mw-content-head">
                      <span class="mw-head-title">Süpermix & Keşfet</span>
                      {#if !theme.hideSubtitles}
                        <span class="mw-head-sub muted">Kişisel miksler ve öne çıkan albümler</span>
                      {/if}
                    </div>

                    {#if theme.searchPlacement === 'home' || theme.searchPlacement === 'both'}
                      <div class="mw-search-mock">
                        <Icon name="search" size={9} />
                        <span>Şarkı, sanatçı veya albüm ara...</span>
                      </div>
                    {/if}

                    <!-- Realistic Horizontal Mix Cards (matching HomeView .mix-card) -->
                    <div class="mw-mix-row" data-card-size={theme.cardSize}>
                      <div class="mw-mix-card active">
                        <div class="mw-mix-thumb" style:background-color={theme.accent}>
                          <Icon name="now" size={12} />
                        </div>
                        <div class="mw-mix-txt">
                          <span class="mw-ct truncate">Süpermix'im</span>
                          <span class="mw-ca truncate muted">Kişisel Miks</span>
                        </div>
                      </div>
                      <div class="mw-mix-card">
                        <div class="mw-mix-thumb" style:background-color="#8338ec">
                          <Icon name="radio" size={12} />
                        </div>
                        <div class="mw-mix-txt">
                          <span class="mw-ct truncate">Arşiv Mix'im</span>
                          <span class="mw-ca truncate muted">Eski Parçalar</span>
                        </div>
                      </div>
                    </div>

                    <!-- Realistic Square Album Grid (showing cardSize column density accurately) -->
                    <div class="mw-albums-grid" data-card-size={theme.cardSize}>
                      {#each MOCK_HOME_ALBUMS.slice(0, theme.cardSize === 'compact' ? 4 : theme.cardSize === 'large' ? 2 : 3) as album (album.title)}
                        <div class="mw-album-card">
                          <div class="mw-album-art" style:background-color={album.tone}>
                            <Icon name="disc" size={14} />
                          </div>
                          <span class="mw-ct truncate">{album.title}</span>
                          <span class="mw-ca truncate muted">{album.sub}</span>
                        </div>
                      {/each}
                    </div>
                  {:else}
                    <!-- Now Playing + Queue Density Simulation -->
                    <div class="mw-now-split">
                      <div class="mw-now-mini">
                        <div class="mw-now-cover" style:background-color={theme.accent}>
                          <Icon name="now" size={20} />
                        </div>
                        <span class="mw-ct truncate">Karanlıkta Parlayan</span>
                        {#if !theme.hideSubtitles}
                          <span class="mw-ca truncate muted">Duman · Belki Alışman Lazım</span>
                        {/if}
                        <div class="mw-lyric-preview">
                          <span class="mw-lyr-line active">Sözler senkron akıyor...</span>
                          <span class="mw-lyr-line muted">Satır aralığı & font canlı</span>
                        </div>
                      </div>

                      <div class="mw-queue-box" class:compact={theme.density === 'compact'}>
                        <div class="mw-q-head">
                          <span>Sıradaki</span>
                          <span class="muted">{theme.density === 'compact' ? 'Sıkı' : 'Rahat'}</span>
                        </div>
                        <div class="mw-q-list">
                          {#each MOCK_QUEUE_ROWS as row (row.title)}
                            <div class="mw-q-row" class:active={row.active}>
                              <div class="mw-q-meta">
                                <span class="mw-qt truncate">{row.title}</span>
                                <span class="mw-qa truncate muted">{row.artist}</span>
                              </div>
                              <span class="mw-qd muted">{row.dur}</span>
                            </div>
                          {/each}
                        </div>
                      </div>
                    </div>
                  {/if}
                </div>
              </div>
            </div>

            <div class="mw-playerbar">
              <div class="mw-pb-track">
                <div class="mw-pb-cover"></div>
                <div class="mw-pb-meta">
                  <span class="mw-pbt truncate">Karanlıkta Parlayan</span>
                  <span class="mw-pba truncate muted">Duman</span>
                </div>
              </div>

              <div class="mw-pb-center">
                <div class="mw-pb-btns">
                  <span class="mw-ctrl-btn"><Icon name="prev" size={10} /></span>
                  <span class="mw-pb-play" class:squircle={theme.playButtonShape === 'squircle'}>
                    <Icon name="play" size={9} />
                  </span>
                  <span class="mw-ctrl-btn"><Icon name="next" size={10} /></span>
                </div>
                <div class="mw-pb-progress">
                  <span class="mw-time">01:24</span>
                  <div class="mw-slider"><div class="mw-slider-fill" style:width="42%"></div></div>
                  <span class="mw-time" class:rem-mode={theme.showRemainingTime}>
                    {theme.showRemainingTime ? '-02:21' : '03:45'}
                  </span>
                </div>
              </div>

              <div class="mw-pb-vol">
                <span class="mw-speaker-icon"><Icon name="speaker" size={11} /></span>
                <Icon name="volume" size={11} />
              </div>
            </div>
          {/if}
        </div>

        <!-- Compact 4-Tile Status Strip (replaces 13 noisy summary boxes) -->
        <div class="sim-summary-strip">
          <div class="sim-stat">
            <span class="sim-stat-k muted">Tema & Yazı</span>
            <span class="sim-stat-v truncate">
              <span class="sim-swatch-dot" style:background-color={theme.accent}></span>
              {activeModeLabel} · {activeFontLabel}
            </span>
          </div>
          <div class="sim-stat">
            <span class="sim-stat-k muted">Stil & Kenar</span>
            <span class="sim-stat-v truncate">
              {simBoxed ? 'Kutulu' : 'Akıcı'} · {edgeLabel}
            </span>
          </div>
          <div class="sim-stat">
            <span class="sim-stat-k muted">Ölçek & Yoğunluk</span>
            <span class="sim-stat-v truncate">
              %{theme.zoom} · {theme.density === 'compact' ? 'Sıkı' : 'Rahat'} · {theme.cardSize === 'compact' ? 'Kompakt' : theme.cardSize === 'large' ? 'Geniş' : 'Standart'}
            </span>
          </div>
          <div class="sim-stat">
            <span class="sim-stat-k muted">Çalma & Oturum</span>
            <span class="sim-stat-v truncate">
              {player.settings.loudnessNormalization ? 'Norm Açık' : 'Norm Kapalı'} · {player.settings.resumeSession ? 'Devam Açık' : 'Devam Kapalı'}
            </span>
          </div>
        </div>
      </div>
    </aside>
  </div>
</section>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 18px;
    width: 100%;
    max-width: 1520px;
    padding-bottom: 32px;
  }

  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    flex-wrap: wrap;
  }

  .head-actions {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
  }

  h1 {
    font-size: 26px;
    font-weight: 800;
  }

  .settings-tabs {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
    padding-bottom: 4px;
  }

  .tab-pill {
    min-height: 38px;
    padding: 0 14px;
    font-size: 13.5px;
    font-weight: 700;
    gap: 8px;
    border-radius: var(--radius);
  }

  .settings-layout {
    display: grid;
    grid-template-columns: minmax(0, 1.35fr) minmax(380px, 490px);
    gap: 22px;
    align-items: start;
  }

  .settings-controls-col {
    min-width: 0;
    container-type: inline-size;
  }

  .simulation-col {
    position: sticky;
    top: 12px;
    min-width: 0;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(min(100%, 330px), 1fr));
    gap: 16px;
  }

  .span-2 {
    grid-column: 1 / -1;
  }

  .block {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 20px;
  }

  h2 {
    font-size: 16.5px;
    font-weight: 800;
  }

  .split-2 {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 14px;
  }

  .modes {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(128px, 1fr));
    gap: 8px;
  }

  .mode-btn {
    flex-direction: column;
    align-items: stretch;
    padding: 9px 11px;
    gap: 4px;
    height: auto;
    text-align: left;
  }

  .m-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 6px;
    width: 100%;
  }

  .m-title {
    font-weight: 700;
    font-size: 13px;
  }

  .m-palette {
    display: inline-flex;
    align-items: center;
    gap: 2px;
    padding: 2px;
    border-radius: 999px;
    background: var(--bg);
    border: 1px solid var(--border);
    flex-shrink: 0;
  }

  .m-palette > span {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .m-sub {
    font-size: 11px;
    opacity: 0.78;
  }

  .swatches {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }

  .swatch {
    position: relative;
    width: 40px;
    height: 40px;
    border-radius: var(--radius);
    border: 2px solid color-mix(in srgb, var(--c) 68%, #000);
    background: var(--c);
    box-shadow: 0 var(--edge-depth) 0 color-mix(in srgb, var(--c) 60%, #000);
    color: #fff;
    display: grid;
    place-items: center;
    cursor: pointer;
    transition: transform 70ms ease, box-shadow 70ms ease;
  }

  .swatch:active,
  .swatch[aria-checked='true'],
  .swatch.selected {
    transform: translateY(3px);
    box-shadow: 0 1px 0 color-mix(in srgb, var(--c) 60%, #000);
  }

  .swatch[aria-checked='true'],
  .swatch.selected {
    outline: 3px solid var(--text);
    outline-offset: 2px;
  }

  .custom {
    background: var(--surface-2);
    border-color: var(--border);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    color: var(--text);
    font-size: 20px;
    font-weight: 700;
  }

  .custom.selected {
    background: var(--c);
    color: #fff;
  }

  .custom input {
    position: absolute;
    inset: 0;
    opacity: 0;
    cursor: pointer;
    width: 100%;
    height: 100%;
  }

  .hex-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .hex-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    border: 1px solid var(--border);
  }

  .hex {
    font-size: 12px;
  }

  .inline-notice {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    flex-wrap: wrap;
    padding: 10px 12px;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    border: 1.5px solid var(--border);
    font-size: 12px;
    line-height: 1.4;
  }

  .btn.sm {
    min-height: 30px;
    padding: 0 10px;
    font-size: 12px;
    font-weight: 700;
  }

  .font-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(195px, 1fr));
    gap: 10px;
  }

  .font-card {
    flex-direction: column;
    align-items: flex-start;
    padding: 10px 12px;
    gap: 3px;
    text-align: left;
  }

  .f-name {
    font-weight: 800;
    font-size: 14.5px;
  }

  .f-sample {
    font-size: 11.5px;
    opacity: 0.8;
  }

  .sub-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .sub-label {
    font-size: 11.5px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .sub-desc {
    font-size: 12px;
    line-height: 1.35;
    margin-top: -2px;
  }

  .reset-link-btn {
    min-height: 36px;
    padding: 0 12px;
    font-size: 12.5px;
    font-weight: 700;
    gap: 6px;
  }

  .nav-order-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(210px, 1fr));
    gap: 8px;
  }

  .nav-order-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding: 8px 10px;
    border-radius: var(--radius);
    background: var(--surface-2);
    border: 2px solid var(--border);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .nav-order-left {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .order-idx {
    font-size: 11px;
    font-weight: 800;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: var(--surface);
    border: 1px solid var(--border);
    display: grid;
    place-items: center;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .order-icon {
    display: flex;
    align-items: center;
    color: var(--accent);
    flex-shrink: 0;
  }

  .order-label {
    font-size: 13px;
    font-weight: 700;
    color: var(--text);
  }

  .nav-order-actions {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .icon-btn.sm {
    width: 30px;
    height: 30px;
    min-height: 30px;
    padding: 0;
    display: grid;
    place-items: center;
    border-radius: var(--radius-sm);
  }

  .opt-row {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .name-row {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .text-input {
    flex: 1;
    min-width: 0;
    height: 40px;
    padding: 0 12px;
    border-radius: var(--radius);
    border: 2px solid var(--border);
    background: var(--surface-2);
    color: var(--text);
    font: inherit;
    font-weight: 700;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .text-input:focus {
    outline: none;
    border-color: var(--accent);
  }

  .toggle-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .toggle-list.two-col {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 10px;
  }

  .toggle-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 10px 12px;
    border-radius: var(--radius);
    background: var(--surface-2);
    border: 2px solid var(--border);
  }

  .toggle-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .t-title {
    font-weight: 700;
    font-size: 13.5px;
  }

  .t-desc {
    font-size: 12px;
  }

  .toggle-btn {
    min-width: 74px;
    flex-shrink: 0;
    font-weight: 700;
  }

  /* Discord Block & Card */
  .discord-block {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .discord-block-head {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 12px;
  }

  .discord-sim-card {
    background: #1e1f22;
    border: 2px solid var(--border);
    border-radius: var(--radius);
    padding: 12px 14px;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .d-sim-user-bar {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .d-sim-avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    position: relative;
    flex-shrink: 0;
  }

  .d-sim-avatar-img {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    object-fit: cover;
  }

  .d-sim-avatar-fallback {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    background: var(--surface-3);
    color: var(--accent);
    display: grid;
    place-items: center;
  }

  .d-sim-status-dot {
    position: absolute;
    bottom: -1px;
    right: -1px;
    width: 9px;
    height: 9px;
    background: #23a55a;
    border-radius: 50%;
    border: 2px solid #1e1f22;
  }

  .d-sim-user-meta {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .d-sim-username {
    font-size: 12.5px;
    font-weight: 800;
    color: #f2f3f5;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .d-sim-tag {
    font-size: 11px;
    font-weight: 600;
    color: #949ba4;
  }

  .d-sim-activity-label {
    font-size: 10px;
    font-weight: 800;
    color: #b5bac1;
    letter-spacing: 0.04em;
  }

  .d-sim-activity-box {
    margin-top: 8px;
    background: #2b2d31;
    border-radius: var(--radius-sm);
    padding: 10px;
    border: 1px solid var(--border);
  }

  .d-sim-act-main {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .d-sim-cover-wrap {
    width: 52px;
    height: 52px;
    border-radius: 8px;
    position: relative;
    flex-shrink: 0;
  }

  .d-sim-cover {
    width: 100%;
    height: 100%;
    border-radius: 8px;
    object-fit: cover;
  }

  .d-sim-cover-fallback {
    width: 100%;
    height: 100%;
    border-radius: 8px;
    background: #1e1f22;
    color: var(--accent);
    display: grid;
    place-items: center;
  }

  .d-sim-badge-dot {
    position: absolute;
    right: -4px;
    bottom: -4px;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #23a55a;
    border: 2px solid #2b2d31;
    display: grid;
    place-items: center;
    color: #ffffff;
  }

  .d-sim-meta {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .d-sim-app-title {
    font-size: 13px;
    font-weight: 800;
    color: #f2f3f5;
  }

  .d-sim-song {
    font-size: 12px;
    font-weight: 700;
    color: #dbdee1;
  }

  .d-sim-artist {
    font-size: 11px;
    color: #949ba4;
  }

  .d-sim-progress-wrap {
    margin-top: 5px;
    width: 100%;
  }

  .d-sim-bar-bg {
    height: 4px;
    border-radius: 2px;
    background: #4e5058;
    overflow: hidden;
  }

  .d-sim-bar-fill {
    height: 100%;
    background: #ffffff;
    border-radius: 2px;
  }

  .d-sim-time-text {
    display: flex;
    justify-content: space-between;
    font-size: 10px;
    color: #949ba4;
    margin-top: 2px;
    font-family: monospace;
  }

  .d-custom-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .d-app-name-row {
    flex-wrap: wrap;
    gap: 10px;
  }

  .d-app-name-wrap {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
  }

  .discord-input {
    width: 155px;
    padding: 6px 10px;
    border-radius: var(--radius-sm);
    border: 2px solid var(--border);
    background: var(--surface);
    color: var(--text);
    font: inherit;
    font-size: 12.5px;
    font-weight: 700;
  }

  .discord-input:focus {
    outline: none;
    border-color: var(--accent);
  }

  .d-disabled-msg {
    padding: 16px;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    text-align: center;
    border: 2px dashed var(--border);
  }

  /* About & Update */
  .about-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(190px, 1fr));
    gap: 10px;
  }

  .about-item {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
    padding: 12px;
    border-radius: var(--radius);
    background: var(--surface-2);
    border: 2px solid var(--border);
    min-width: 0;
  }

  .about-val {
    font-size: 15px;
    font-weight: 800;
    max-width: 100%;
  }

  .about-sub {
    font-size: 12px;
  }

  .github-btn,
  .tour-btn {
    margin-top: 2px;
    min-height: 32px;
    padding: 0 11px;
    font-weight: 700;
    font-size: 12px;
  }

  .update-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 12px;
    padding: 12px 14px;
    border-radius: var(--radius);
    background: var(--surface-2);
    border: 2px solid var(--border);
  }

  .update-status {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1;
  }

  .u-title {
    font-weight: 800;
    font-size: 13.5px;
  }

  .u-desc {
    font-size: 12px;
  }

  .accent-txt {
    color: var(--accent);
  }

  .ok-txt {
    color: #06d6a0;
  }

  .update-actions {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }

  /* =========================================================================
     Right / Sticky Live Preview Inspector
     Uses `.card.block.preview-panel` so seamless mode keeps a crisp surface
     and never collides with ArtistView `.sim-card`.
     ========================================================================= */
  .preview-panel {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 16px;
  }

  .sim-top-bar {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .sim-header {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .sim-title-wrap {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .sim-live-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .sim-title {
    font-size: 14.5px;
    font-weight: 800;
  }

  .sim-sub {
    font-size: 11.5px;
    line-height: 1.35;
  }

  .sim-scene-tabs {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 4px;
    padding: 3px;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    border: 1.5px solid var(--border);
  }

  .sim-scene-btn {
    height: 30px;
    border: none;
    border-radius: calc(var(--radius-sm) - 3px);
    background: transparent;
    color: var(--text-muted);
    font-family: inherit;
    font-size: 11.5px;
    font-weight: 700;
    cursor: pointer;
    transition: background-color 120ms ease, color 120ms ease;
    white-space: nowrap;
    padding: 0 6px;
  }

  .sim-scene-btn:hover {
    background: var(--surface-3);
    color: var(--text);
  }

  .sim-scene-btn.active {
    background: var(--accent);
    color: var(--on-accent);
    font-weight: 800;
  }

  /* Bounded Fixed-Proportion Mini Window */
  .mini-window {
    width: 100%;
    max-width: 500px;
    margin: 0 auto;
    border-radius: var(--radius);
    border: 2px solid var(--border);
    background: var(--bg);
    box-shadow: 0 var(--sim-edge) 0 var(--edge);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    font-size: calc(12px * var(--sim-font-factor));
  }

  .mini-window.is-seamless {
    border-width: 1px;
    box-shadow: none;
  }

  .mw-titlebar {
    height: 28px;
    background: var(--surface-2);
    border-bottom: 1.5px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 10px;
    gap: 8px;
  }

  .mw-controls {
    display: flex;
    align-items: center;
    gap: 5px;
    flex-shrink: 0;
  }

  .mw-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .mw-dot.close {
    background: #ff5f56;
  }

  .mw-dot.min {
    background: #ffbd2e;
  }

  .mw-dot.max {
    background: #27c93f;
  }

  .mw-title {
    font-size: 11px;
    font-weight: 700;
    color: var(--text);
    opacity: 0.9;
    min-width: 0;
  }

  .mw-tb-right {
    display: flex;
    align-items: center;
    gap: 5px;
    flex-shrink: 0;
  }

  .mw-norm-badge {
    font-size: 9px;
    font-weight: 800;
    padding: 1px 5px;
    border-radius: 4px;
    background: var(--accent-soft);
    color: var(--accent);
    letter-spacing: 0.04em;
  }

  .mw-zoom-badge {
    font-size: 10px;
    font-weight: 800;
    padding: 1px 6px;
    border-radius: 999px;
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--accent);
  }

  .mw-body {
    display: flex;
    min-height: 248px;
    background: var(--bg);
  }

  .mw-sidebar {
    background: var(--surface);
    border-right: 1.5px solid var(--border);
    padding: 8px 6px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex-shrink: 0;
  }

  .mw-sidebar.collapsed {
    padding: 8px 4px;
    align-items: center;
  }

  .mw-brand {
    display: flex;
    align-items: center;
    gap: 5px;
    font-weight: 800;
    color: var(--accent);
    min-width: 0;
  }

  .mw-logo {
    width: 16px;
    height: 16px;
    border-radius: 4px;
    background: var(--accent);
    color: var(--on-accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }

  .mw-brand-text {
    font-size: 9.5px;
    font-weight: 800;
  }

  .mw-nav-items {
    display: flex;
    flex-direction: column;
    gap: 3px;
    width: 100%;
  }

  .mw-nav-item {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 4px 5px;
    border-radius: calc(var(--radius-sm) * 0.65);
    font-size: 9.5px;
    font-weight: 600;
    color: var(--text-muted);
    min-width: 0;
  }

  .mw-nav-item.active {
    background: var(--accent-soft);
    color: var(--text);
    font-weight: 800;
  }

  .mw-start-dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background: var(--accent);
    margin-left: auto;
    flex-shrink: 0;
  }

  .mw-content {
    flex: 1;
    min-width: 0;
    padding: calc(10px * var(--sim-zoom)) calc(11px * var(--sim-zoom));
    background: var(--bg);
    overflow: hidden;
  }

  .mw-zoom-stage {
    display: flex;
    flex-direction: column;
    gap: calc(8px * var(--sim-zoom));
    font-size: calc(12px * var(--sim-font-factor) * var(--sim-zoom));
    width: 100%;
  }

  .mw-content-head {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .mw-head-title {
    font-size: 1em;
    font-weight: 800;
    line-height: 1.2;
  }

  .mw-head-sub {
    font-size: 0.78em;
    line-height: 1.2;
  }

  .mw-search-mock {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 4px 8px;
    border-radius: var(--radius-sm);
    background: var(--surface);
    border: 1px solid var(--border);
    font-size: 0.78em;
    color: var(--text-muted);
  }

  .mini-window.is-boxed .mw-search-mock {
    border-width: 1.5px;
    box-shadow: 0 var(--sim-edge) 0 var(--edge);
  }

  /* Home Mix Row (matches HomeView horizontal square-art mix card) */
  .mw-mix-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 7px;
  }

  .mw-mix-card {
    display: grid;
    grid-template-columns: 28px minmax(0, 1fr);
    align-items: center;
    gap: 7px;
    padding: 5px;
    border-radius: calc(var(--radius) * 0.75);
    background: transparent;
    border: 1px solid transparent;
    min-width: 0;
  }

  .mw-mix-card.active {
    background: var(--accent-soft);
  }

  .mini-window.is-boxed .mw-mix-card {
    background: var(--surface);
    border: 1.5px solid var(--border);
    box-shadow: 0 var(--sim-edge) 0 var(--edge);
  }

  .mini-window.is-boxed .mw-mix-card.active {
    border-color: var(--accent);
  }

  .mw-mix-row[data-card-size='compact'] .mw-mix-card {
    grid-template-columns: 22px minmax(0, 1fr);
    padding: 4px;
    gap: 5px;
  }

  .mw-mix-row[data-card-size='large'] .mw-mix-card {
    grid-template-columns: 34px minmax(0, 1fr);
    padding: 6px;
    gap: 8px;
  }

  .mw-mix-thumb {
    width: 100%;
    aspect-ratio: 1;
    border-radius: calc(var(--radius-sm) * 0.65);
    display: grid;
    place-items: center;
    color: #fff;
  }

  .mw-mix-txt {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  /* Realistic Square Album Grid */
  .mw-albums-grid {
    display: grid;
    gap: 8px;
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  .mw-albums-grid[data-card-size='compact'] {
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 6px;
  }

  .mw-albums-grid[data-card-size='large'] {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 10px;
  }

  .mw-album-card {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 5px;
    border-radius: calc(var(--radius) * 0.75);
    background: transparent;
    border: 1px solid transparent;
    min-width: 0;
  }

  .mini-window.is-boxed .mw-album-card {
    background: var(--surface);
    border: 1.5px solid var(--border);
    box-shadow: 0 var(--sim-edge) 0 var(--edge);
  }

  .mw-album-art {
    width: 100%;
    aspect-ratio: 1;
    border-radius: calc(var(--radius) * 0.55);
    display: grid;
    place-items: center;
    color: #ffffff;
  }

  .mw-ct {
    font-size: 0.8em;
    font-weight: 700;
    line-height: 1.2;
  }

  .mw-ca {
    font-size: 0.68em;
    line-height: 1.2;
  }

  /* Now Playing + Queue Scene */
  .mw-now-split {
    display: grid;
    grid-template-columns: 0.9fr 1.1fr;
    gap: 8px;
    align-items: start;
  }

  .mw-now-mini {
    display: flex;
    flex-direction: column;
    gap: 5px;
    padding: 7px;
    border-radius: calc(var(--radius) * 0.75);
    background: transparent;
    border: 1px solid transparent;
    min-width: 0;
  }

  .mini-window.is-boxed .mw-now-mini {
    background: var(--surface);
    border: 1.5px solid var(--border);
    box-shadow: 0 var(--sim-edge) 0 var(--edge);
  }

  .mw-now-cover {
    width: 100%;
    max-width: 84px;
    aspect-ratio: 1;
    border-radius: calc(var(--radius) * 0.6);
    display: grid;
    place-items: center;
    color: #ffffff;
    margin: 0 auto 2px;
  }

  .mw-lyric-preview {
    margin-top: 2px;
    padding-top: 5px;
    border-top: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .mw-lyr-line {
    font-size: 0.72em;
    font-weight: 700;
    line-height: 1.25;
  }

  .mw-lyr-line.active {
    color: var(--accent);
  }

  .mw-queue-box {
    display: flex;
    flex-direction: column;
    gap: 5px;
    padding: 7px;
    border-radius: calc(var(--radius) * 0.75);
    background: transparent;
    border: 1px solid transparent;
    min-width: 0;
  }

  .mini-window.is-boxed .mw-queue-box {
    background: var(--surface);
    border: 1.5px solid var(--border);
    box-shadow: 0 var(--sim-edge) 0 var(--edge);
  }

  .mw-q-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 0.78em;
    font-weight: 800;
    padding-bottom: 4px;
    border-bottom: 1px solid var(--border);
  }

  .mw-q-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .mw-queue-box.compact .mw-q-list {
    gap: 2px;
  }

  .mw-q-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 6px;
    padding: 6px 7px;
    border-radius: calc(var(--radius-sm) * 0.65);
    background: transparent;
    border: 1px solid transparent;
    min-width: 0;
  }

  .mw-queue-box.compact .mw-q-row {
    padding: 3px 6px;
  }

  .mw-q-row.active {
    background: var(--accent-soft);
  }

  .mini-window.is-boxed .mw-q-row {
    background: var(--surface-2);
    border-color: var(--border);
  }

  .mini-window.is-boxed .mw-q-row.active {
    border-color: var(--accent);
  }

  .mw-q-meta {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .mw-qt {
    font-size: 0.78em;
    font-weight: 700;
    line-height: 1.2;
  }

  .mw-qa {
    font-size: 0.68em;
    line-height: 1.2;
  }

  .mw-qd {
    font-size: 0.7em;
    font-variant-numeric: tabular-nums;
    flex-shrink: 0;
  }

  /* Mini Desktop Preview Scene */
  .mw-desktop {
    padding: 16px 14px 20px;
    min-height: 220px;
    background: var(--bg);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .mw-desktop-bar {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .mw-desktop-badge {
    font-size: 10px;
    font-weight: 700;
    color: var(--text-muted);
  }

  .mw-mini-strip {
    width: 100%;
    max-width: 320px;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    border-radius: var(--radius);
    background: var(--surface);
    border: 2px solid var(--border);
    box-shadow: 0 var(--sim-edge) 0 var(--edge);
    margin: auto 0;
  }

  .mw-mini-thumb {
    width: 38px;
    height: 38px;
    border-radius: calc(var(--radius-sm) * 0.7);
    display: grid;
    place-items: center;
    color: var(--on-accent);
    flex-shrink: 0;
  }

  .mw-mini-meta {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .mw-mini-title {
    font-size: 11px;
    font-weight: 800;
  }

  .mw-mini-artist {
    font-size: 9.5px;
  }

  .mw-mini-prog {
    height: 3px;
    border-radius: 999px;
    background: var(--surface-3);
    overflow: hidden;
    margin-top: 2px;
  }

  .mw-mini-prog > div {
    height: 100%;
    background: var(--accent);
  }

  .mw-mini-ctrls {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .mw-mini-square {
    width: 165px;
    border-radius: var(--radius);
    background: var(--surface);
    border: 2px solid var(--border);
    box-shadow: 0 var(--sim-edge) 0 var(--edge);
    overflow: hidden;
    margin: auto 0;
  }

  .mw-mini-sq-art {
    width: 100%;
    aspect-ratio: 1;
    display: grid;
    place-items: center;
    color: var(--on-accent);
  }

  .mw-mini-sq-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 8px 10px;
    border-top: 1px solid var(--border);
  }

  /* Bottom Player Bar */
  .mw-playerbar {
    height: 44px;
    background: var(--surface);
    border-top: 1.5px solid var(--border);
    padding: 0 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .mw-pb-track {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 105px;
    min-width: 0;
  }

  .mw-pb-cover {
    width: 22px;
    height: 22px;
    border-radius: 4px;
    background: var(--accent);
    flex-shrink: 0;
  }

  .mw-pb-meta {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .mw-pbt {
    font-size: 9px;
    font-weight: 700;
  }

  .mw-pba {
    font-size: 7.5px;
  }

  .mw-pb-center {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    flex: 1;
    min-width: 0;
  }

  .mw-pb-btns {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .mw-ctrl-btn {
    color: var(--text-muted);
    display: flex;
    align-items: center;
  }

  .mw-pb-play {
    width: 21px;
    height: 21px;
    background: var(--accent);
    color: var(--on-accent);
    display: grid;
    place-items: center;
    border-radius: 50%;
    box-shadow: 0 var(--sim-edge) 0 var(--accent-edge);
  }

  .mw-pb-play.squircle {
    border-radius: 6px;
  }

  .mw-pb-progress {
    display: flex;
    align-items: center;
    gap: 5px;
    width: 100%;
    max-width: 155px;
  }

  .mw-time {
    font-size: 8px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }

  .mw-time.rem-mode {
    color: var(--accent);
    font-weight: 700;
  }

  .mw-slider {
    flex: 1;
    height: 3px;
    background: var(--surface-3);
    border-radius: 999px;
    overflow: hidden;
  }

  .mw-slider-fill {
    height: 100%;
    background: var(--accent);
  }

  .mw-pb-vol {
    display: flex;
    align-items: center;
    gap: 4px;
    color: var(--text-muted);
  }

  .mw-speaker-icon {
    color: var(--accent);
    display: flex;
    align-items: center;
  }

  /* Compact 4-Tile Status Strip */
  .sim-summary-strip {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 8px;
    padding-top: 6px;
    border-top: 1px solid var(--border);
  }

  .sim-stat {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 7px 9px;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    border: 1px solid var(--border);
    min-width: 0;
  }

  .sim-stat-k {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .sim-stat-v {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 11.5px;
    font-weight: 800;
    color: var(--text);
  }

  .sim-swatch-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  /* Mini Layout Controls */
  .mini-layout-row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 10px;
  }

  .mini-layout-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 14px;
    min-height: 46px;
  }

  .ml-preview {
    width: 32px;
    height: 22px;
    border-radius: 4px;
    background: var(--surface-3);
    border: 1px solid var(--border);
    overflow: hidden;
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .ml-strip {
    padding: 2px 4px;
    gap: 3px;
  }

  .ml-cover {
    width: 14px;
    height: 14px;
    border-radius: 2px;
    background: var(--accent);
    flex-shrink: 0;
  }

  .ml-lines {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .ml-lines span {
    height: 2px;
    border-radius: 1px;
    background: var(--text-muted);
    width: 100%;
  }

  .ml-lines span:last-child {
    width: 60%;
  }

  .ml-dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background: var(--accent);
    flex-shrink: 0;
  }

  .ml-square {
    width: 26px;
    height: 26px;
    border-radius: 4px;
    display: flex;
    align-items: flex-end;
    background: var(--surface-3);
  }

  .ml-panel {
    width: 100%;
    height: 10px;
    background: color-mix(in srgb, var(--surface) 80%, #000);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  @media (max-width: 1040px) {
    .settings-layout {
      grid-template-columns: 1fr;
    }

    /* Keep preview easily reachable right above settings on narrower windows */
    .simulation-col {
      position: static;
      order: -1;
    }
  }

  @media (max-width: 720px) {
    .block {
      padding: 16px;
    }

    .mw-pb-track {
      width: 72px;
    }

    .mw-pb-vol {
      display: none;
    }

    .mw-now-split {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 480px) {
    .sim-summary-strip {
      grid-template-columns: 1fr;
    }

    .settings-tabs {
      gap: 6px;
    }

    .tab-pill {
      padding: 0 10px;
      font-size: 12.5px;
    }
  }
</style>
