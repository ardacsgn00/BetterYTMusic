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
    type StartupView
  } from '#lib/theme.svelte.ts';
  import { player } from '#lib/player.svelte.ts';
  import type { UpdateCheckResult } from '#lib/types.ts';
  import Icon from './Icon.svelte';

  const GITHUB_URL = 'https://github.com/ardacsgn00/betterytmusic';

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

  const modes: { id: ThemeMode; label: string; desc: string }[] = [
    { id: 'dark', label: 'Koyu', desc: 'Kömür gri' },
    { id: 'oled', label: 'Gece Siyahı', desc: 'Tam siyah' },
    { id: 'slate', label: 'Kuzey Mavisi', desc: 'Koyu arduvaz' },
    { id: 'light', label: 'Açık', desc: 'Sıcak krem' },
    { id: 'system', label: 'Sistem', desc: 'Otomatik' }
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

  const isPreset = $derived(ACCENT_PRESETS.some((p) => p.value === theme.accent));
</script>

<section class="panel">
  <header class="head">
    <div>
      <h1>Ayarlar</h1>
      <p class="muted">Görünüm, sistem davranışları, güncelleme denetimi ve uygulama bilgileri.</p>
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

  <div class="settings-layout">
    <div class="settings-controls-col">
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
            <span class="m-title">{m.label}</span>
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
      <code class="hex muted">{theme.accent}</code>
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
            <span class="f-sample">{f.sample} — 01:24 / 03:45</span>
          </button>
        {/each}
      </div>
    </div>

    <div class="card block">
      <h2>Sayfa Ölçeği (Zoom), Yazı & Liste Boyutları</h2>
      <p class="muted">Sayfa zoom oranını (%75–%135), metin büyüklüğünü ve panel genişliklerini yönet.</p>
      <div class="sub-group">
        <span class="sub-label muted">Sayfa Görünürlük Oranı (%{theme.zoom})</span>
        <div class="opt-row">
          <button
            class="btn"
            disabled={theme.zoom <= 75}
            onclick={() => theme.stepZoom(-5)}
          >
            − %5
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
        <p class="sub-desc muted">Tüm sayfalardaki metinlerin genel boyutu.</p>
        <div class="opt-row">
          {#each fontScales as s (s.id)}
            <button
              class="btn"
              class:accent={theme.fontScale === s.id}
              onclick={() => theme.setFontScale(s.id)}
            >
              {s.label}
            </button>
          {/each}
        </div>
      </div>

      <div class="sub-group">
        <span class="sub-label muted">Şarkı Listesi Satır Aralığı (Yoğunluk)</span>
        <p class="sub-desc muted">Çalma listeleri ve sıradaki şarkılar arasındaki dikey boşluk.</p>
        <div class="opt-row">
          {#each densities as d (d.id)}
            <button
              class="btn"
              class:accent={theme.density === d.id}
              onclick={() => theme.setDensity(d.id)}
            >
              {d.label}
            </button>
          {/each}
        </div>
      </div>

      <div class="sub-group">
        <span class="sub-label muted">Dinle Ekranı Panel Genişlikleri</span>
        <p class="sub-desc muted">Kapak, şarkı sözleri ve sıra listesini fareyle çekip boyutlandırdıysan varsayılan oranlara sıfırlar.</p>
        <div class="opt-row">
          <button class="btn" onclick={() => theme.resetSplits()}>
            <Icon name="refresh" size={15} />
            Panel Genişliklerini Varsayılana Sıfırla
          </button>
        </div>
      </div>
    </div>

    <div class="card block">
      <h2>Kart Köşeleri & 3B Derinlik</h2>
      <p class="muted">Düğme ve kartların köşe yuvarlaklığını ve alt kenar kalınlığını seç.</p>
      <div class="sub-group">
        <span class="sub-label muted">Köşe Stili</span>
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
      <div class="sub-group">
        <span class="sub-label muted">Dokunsal Alt Kenar</span>
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

    <div class="card block">
      <h2>Ana Sayfa & Sol Menü Düzeni</h2>
      <p class="muted">Açılış sekmesini, kart boyutlarını, sol menü modunu ve başlık alt yazılarını özelleştir.</p>
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
        <span class="sub-label muted">Ana Sayfa Kart Boyutu & Yoğunluğu</span>
        <div class="opt-row">
          {#each cardSizes as cs (cs.id)}
            <button
              class="btn"
              class:accent={theme.cardSize === cs.id}
              onclick={() => theme.setCardSize(cs.id)}
            >
              {cs.label}
            </button>
          {/each}
        </div>
      </div>

      <div class="sub-group">
        <span class="sub-label muted">Sol Menü ({theme.sidebarCollapsed ? 'İkon Modu' : `${theme.sidebarWidth}px`} · Sağ kenardan sürüklenebilir)</span>
        <div class="opt-row">
          <button
            class="btn"
            class:accent={!theme.sidebarCollapsed}
            onclick={() => theme.setSidebarWidth(228)}
          >
            Tam Menü (228px)
          </button>
          <button
            class="btn"
            class:accent={theme.sidebarCollapsed}
            onclick={() => theme.setSidebarWidth(74)}
          >
            Sadece İkon Modu
          </button>
        </div>
      </div>

      <div class="sub-group">
        <span class="sub-label muted">Bölüm Açıklama Alt Yazıları</span>
        <div class="opt-row">
          <button
            class="btn"
            class:accent={!theme.hideSubtitles}
            onclick={() => theme.setHideSubtitles(false)}
          >
            Göster
          </button>
          <button
            class="btn"
            class:accent={theme.hideSubtitles}
            onclick={() => theme.setHideSubtitles(true)}
          >
            Gizle (Sade Başlıklar)
          </button>
        </div>
      </div>
    </div>

    <div class="card block">
      <h2>Alt Oynatıcı Bar & Uygulama Başlığı</h2>
      <p class="muted">Oynat düğmesi şeklini, süre görünümünü ve sol üstteki uygulama adını seç.</p>
      <div class="sub-group">
        <span class="sub-label muted">Oynat Düğmesi Stili</span>
        <div class="opt-row">
          <button
            class="btn"
            class:accent={theme.playButtonShape === 'circle'}
            onclick={() => theme.setPlayButtonShape('circle')}
          >
            Yuvarlak Daire
          </button>
          <button
            class="btn"
            class:accent={theme.playButtonShape === 'squircle'}
            onclick={() => theme.setPlayButtonShape('squircle')}
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
            onclick={() => theme.setShowRemainingTime(false)}
          >
            Toplam Süre (3:31)
          </button>
          <button
            class="btn"
            class:accent={theme.showRemainingTime}
            onclick={() => theme.setShowRemainingTime(true)}
          >
            Kalan Süre (-1:15)
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
      <h2>Sistem & Başlangıç Ayarları</h2>
      <p class="muted">Pencere kapatma davranışını ve Windows başlangıç tercihlerini yönet.</p>
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
            <span class="t-title">Windows Başlangıcında Çalıştır</span>
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
            <span class="u-desc muted">
              GitHub deposundaki en son sürümü kontrol etmek için düğmeye tıkla.
            </span>
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
    </div>

    <!-- Right Column: Sticky Live Simulation & Effect Inspector -->
    <aside class="simulation-col">
      <div class="card sim-card">
        <div class="sim-header">
          <div class="sim-title-wrap">
            <span class="sim-dot live-dot"></span>
            <span class="sim-title">Canlı Arayüz Simülasyonu</span>
          </div>
          <span class="sim-sub muted">Anlık Değişim & Etki Önizlemesi</span>
        </div>

        <!-- The Mini BetterYTMusic Window Mockup -->
        <div class="mini-window">
          <div class="mw-titlebar">
            <div class="mw-controls">
              <span class="mw-dot close"></span>
              <span class="mw-dot min"></span>
              <span class="mw-dot max"></span>
            </div>
            <span class="mw-title truncate">{theme.appName.trim() || 'BetterYTMusic'}</span>
            <span class="mw-zoom-badge">%{theme.zoom}</span>
          </div>

          <div class="mw-body">
            <!-- Mini Sidebar -->
            <div class="mw-sidebar" class:collapsed={theme.sidebarCollapsed}>
              <div class="mw-brand">
                <span class="mw-logo"><Icon name="play" size={11} /></span>
                {#if !theme.sidebarCollapsed}
                  <span class="mw-brand-text truncate">{theme.appName.trim() || 'BetterYTMusic'}</span>
                {/if}
              </div>
              <div class="mw-nav-items">
                <div class="mw-nav-item active">
                  <Icon name="now" size={12} />
                  {#if !theme.sidebarCollapsed}<span>Dinle</span>{/if}
                </div>
                <div class="mw-nav-item">
                  <Icon name="home" size={12} />
                  {#if !theme.sidebarCollapsed}<span>Ana Sayfa</span>{/if}
                </div>
                <div class="mw-nav-item">
                  <Icon name="search" size={12} />
                  {#if !theme.sidebarCollapsed}<span>Ara</span>{/if}
                </div>
              </div>
            </div>

            <!-- Mini Content Area -->
            <div class="mw-content">
              <div class="mw-content-head">
                <span class="mw-head-title">Süpermix & Listeler</span>
                {#if !theme.hideSubtitles}
                  <span class="mw-head-sub muted">Kişisel miksler ve öneriler</span>
                {/if}
              </div>

              <!-- Mini Cards Grid reflecting cardSize, radius, edgeDepth -->
              <div class="mw-cards-grid" data-size={theme.cardSize}>
                <div class="mw-card active">
                  <div class="mw-card-art" style:background-color={theme.accent}>
                    <Icon name="now" size={15} />
                    <span class="mw-card-play"><Icon name="play" size={9} /></span>
                  </div>
                  <div class="mw-card-info">
                    <span class="mw-ct truncate">Süpermix'im</span>
                    <span class="mw-ca truncate muted">Kişisel Miks</span>
                  </div>
                </div>

                <div class="mw-card">
                  <div class="mw-card-art" style:background-color="#8338ec">
                    <Icon name="now" size={15} />
                  </div>
                  <div class="mw-card-info">
                    <span class="mw-ct truncate">Arşiv Mix'im</span>
                    <span class="mw-ca truncate muted">Eski Parçalar</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Mini PlayerBar reflecting playButtonShape and showRemainingTime -->
          <div class="mw-playerbar">
            <div class="mw-pb-track">
              <div class="mw-pb-cover"></div>
              <div class="mw-pb-meta">
                <span class="mw-pbt truncate">Örnek Şarkı</span>
                <span class="mw-pba truncate muted">Sanatçı</span>
              </div>
            </div>

            <div class="mw-pb-center">
              <div class="mw-pb-btns">
                <span class="mw-ctrl-btn"><Icon name="prev" size={11} /></span>
                <span
                  class="mw-pb-play"
                  class:squircle={theme.playButtonShape === 'squircle'}
                  title={theme.playButtonShape === 'circle' ? 'Yuvarlak Oynatıcı' : 'Yumuşatılmış Kare Oynatıcı'}
                >
                  <Icon name="play" size={10} />
                </span>
                <span class="mw-ctrl-btn"><Icon name="next" size={11} /></span>
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
              <span class="mw-speaker-icon"><Icon name="speaker" size={12} /></span>
              <Icon name="volume" size={12} />
            </div>
          </div>
        </div>

        <!-- Live Effect Inspector Badges -->
        <div class="sim-inspector">
          <span class="insp-title muted">Seçili Ayar Değerleri Özeti</span>
          <div class="insp-badges">
            <div class="insp-badge">
              <span class="insp-dot" style:background-color={theme.accent}></span>
              <span class="insp-k">Vurgu:</span>
              <span class="insp-v">{theme.accent}</span>
            </div>
            <div class="insp-badge">
              <span class="insp-k">Tema Modu:</span>
              <span class="insp-v">{theme.resolved.toUpperCase()}</span>
            </div>
            <div class="insp-badge">
              <span class="insp-k">3B Dokunsal:</span>
              <span class="insp-v">{theme.edgeDepth === 'none' ? '0px (Kapalı)' : theme.edgeDepth === 'subtle' ? '2px (İnce)' : theme.edgeDepth === 'normal' ? '4px (Standart)' : '6px (Belirgin)'}</span>
            </div>
            <div class="insp-badge">
              <span class="insp-k">Oynatıcı Butonu:</span>
              <span class="insp-v">{theme.playButtonShape === 'circle' ? 'Yuvarlak Daire' : 'Yumuşatılmış Kare'}</span>
            </div>
            <div class="insp-badge">
              <span class="insp-k">Süre Çubuğu:</span>
              <span class="insp-v">{theme.showRemainingTime ? 'Kalan Süre (-02:21)' : 'Toplam Süre (03:45)'}</span>
            </div>
            <div class="insp-badge">
              <span class="insp-k">Yazı Boyutu:</span>
              <span class="insp-v">{theme.fontScale === 'sm' ? 'Küçük (13px)' : theme.fontScale === 'lg' ? 'Büyük (15.5px)' : 'Normal (14px)'}</span>
            </div>
            <div class="insp-badge">
              <span class="insp-k">Liste Aralığı:</span>
              <span class="insp-v">{theme.density === 'compact' ? 'Sıkı (Kompakt)' : 'Rahat (Geniş)'}</span>
            </div>
            <div class="insp-badge">
              <span class="insp-k">Sol Menü:</span>
              <span class="insp-v">{theme.sidebarCollapsed ? 'İkon Modu (74px)' : `${theme.sidebarWidth}px`}</span>
            </div>
            <div class="insp-badge">
              <span class="insp-k">Kart Boyutu:</span>
              <span class="insp-v">{theme.cardSize === 'compact' ? 'Kompakt' : theme.cardSize === 'large' ? 'Geniş' : 'Standart'}</span>
            </div>
            <div class="insp-badge">
              <span class="insp-k">Başlık Yazıları:</span>
              <span class="insp-v">{theme.hideSubtitles ? 'Gizli (Sade)' : 'Görünür'}</span>
            </div>
          </div>
          <div class="sim-tip-box">
            <span class="sim-tip-icon">💡</span>
            <p class="sim-tip muted">
              Sol taraftan yaptığın her seçim (renk, dokunsal kenar, buton stili, zoom, süre modu) anında bu simülasyonda ve uygulamada canlı olarak değişir.
            </p>
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
    gap: 20px;
    width: 100%;
    max-width: 1540px;
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

  .settings-layout {
    display: grid;
    grid-template-columns: minmax(0, 1.45fr) minmax(380px, 460px);
    gap: 24px;
    align-items: start;
  }
  .settings-controls-col {
    min-width: 0;
  }
  .simulation-col {
    position: sticky;
    top: 16px;
    min-width: 0;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 20px;
  }
  .span-2 {
    grid-column: span 2;
  }
  .block {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 22px;
  }
  h2 {
    font-size: 17px;
    font-weight: 800;
  }
  .modes {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(115px, 1fr));
    gap: 10px;
    margin-top: 4px;
  }
  .mode-btn {
    flex-direction: column;
    align-items: flex-start;
    padding: 10px 12px;
    gap: 2px;
    height: auto;
  }
  .m-title {
    font-weight: 700;
    font-size: 14px;
  }
  .m-sub {
    font-size: 11px;
    opacity: 0.78;
  }
  .swatches {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    margin-top: 4px;
  }
  .swatch {
    position: relative;
    width: 42px;
    height: 42px;
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
    font-size: 22px;
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
  .hex {
    font-size: 12px;
  }
  .font-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(210px, 1fr));
    gap: 12px;
    margin-top: 4px;
  }
  .font-card {
    flex-direction: column;
    align-items: flex-start;
    padding: 12px 14px;
    gap: 4px;
    text-align: left;
  }
  .f-name {
    font-weight: 800;
    font-size: 15px;
  }
  .f-sample {
    font-size: 12px;
    opacity: 0.8;
  }
  .sub-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: 4px;
  }
  .sub-label {
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }
  .sub-desc {
    font-size: 12px;
    line-height: 1.35;
    margin-top: -2px;
    margin-bottom: 2px;
  }
  .opt-row {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
  .name-row {
    display: flex;
    gap: 10px;
    align-items: center;
    margin-top: 4px;
  }
  .text-input {
    flex: 1;
    min-width: 0;
    height: 42px;
    padding: 0 14px;
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
    margin-top: 4px;
  }
  .toggle-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 14px;
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
    min-width: 78px;
    flex-shrink: 0;
    font-weight: 700;
  }
  .about-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 12px;
    margin-top: 4px;
  }
  .about-item {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
    padding: 12px 14px;
    border-radius: var(--radius);
    background: var(--surface-2);
    border: 2px solid var(--border);
    min-width: 0;
    overflow: hidden;
  }
  .about-val {
    font-size: 15.5px;
    font-weight: 800;
    max-width: 100%;
  }
  .about-sub {
    font-size: 12px;
  }
  .github-btn {
    margin-top: 2px;
    min-height: 32px;
    padding: 0 12px;
    font-weight: 700;
    font-size: 12.5px;
    max-width: 100%;
  }
  .tour-btn {
    margin-top: 2px;
    min-height: 32px;
    padding: 0 12px;
    font-weight: 700;
    font-size: 12.5px;
    max-width: 100%;
  }
  .update-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 14px;
    padding: 14px 16px;
    border-radius: var(--radius);
    background: var(--surface-2);
    border: 2px solid var(--border);
    margin-top: 4px;
  }
  .update-status {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
    flex: 1;
  }
  .u-title {
    font-weight: 800;
    font-size: 14px;
  }
  .u-desc {
    font-size: 12.5px;
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
    gap: 10px;
    flex-wrap: wrap;
  }

  /* Right Sticky Simulation Card & Mockup */
  .sim-card {
    display: flex;
    flex-direction: column;
    gap: 14px;
    padding: 18px;
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
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
  .sim-dot.live-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--accent);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--accent) 30%, transparent);
    flex-shrink: 0;
  }
  .sim-title {
    font-size: 15px;
    font-weight: 800;
  }
  .sim-sub {
    font-size: 11.5px;
  }

  /* Mini Mockup Window */
  .mini-window {
    border-radius: calc(var(--radius) * 0.85);
    border: 2px solid var(--border);
    background: var(--bg);
    box-shadow: 0 4px 0 var(--edge);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  .mw-titlebar {
    height: 28px;
    background: var(--surface-2);
    border-bottom: 1.5px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 10px;
  }
  .mw-controls {
    display: flex;
    align-items: center;
    gap: 5px;
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
    opacity: 0.85;
    max-width: 170px;
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
    height: 185px;
    background: var(--bg);
  }
  .mw-sidebar {
    width: 88px;
    background: var(--surface);
    border-right: 1.5px solid var(--border);
    padding: 10px 6px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    transition: width 0.15s ease;
  }
  .mw-sidebar.collapsed {
    width: 38px;
    padding: 10px 4px;
    align-items: center;
  }
  .mw-brand {
    display: flex;
    align-items: center;
    gap: 6px;
    font-weight: 800;
    font-size: 10.5px;
    color: var(--accent);
  }
  .mw-logo {
    width: 17px;
    height: 17px;
    border-radius: 4px;
    background: var(--accent);
    color: #fff;
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }
  .mw-brand-text {
    font-size: 10px;
    font-weight: 800;
  }
  .mw-nav-items {
    display: flex;
    flex-direction: column;
    gap: 4px;
    width: 100%;
  }
  .mw-nav-item {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 4px 6px;
    border-radius: 6px;
    font-size: 10px;
    font-weight: 600;
    color: var(--text-muted);
  }
  .mw-nav-item.active {
    background: color-mix(in srgb, var(--accent) 18%, var(--surface-2));
    color: var(--text);
    font-weight: 700;
    border-left: 2px solid var(--accent);
  }

  .mw-content {
    flex: 1;
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-width: 0;
    background: var(--bg);
  }
  .mw-content-head {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }
  .mw-head-title {
    font-size: 12px;
    font-weight: 800;
  }
  .mw-head-sub {
    font-size: 9px;
  }

  .mw-cards-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }
  .mw-cards-grid[data-size='compact'] .mw-card-art {
    height: 38px;
  }
  .mw-cards-grid[data-size='large'] .mw-card-art {
    height: 56px;
  }

  .mw-card {
    background: var(--surface);
    border: 1.5px solid var(--border);
    border-radius: calc(var(--radius) * 0.75);
    padding: 6px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }
  .mw-card.active {
    border-color: var(--accent);
  }
  .mw-card-art {
    height: 46px;
    border-radius: calc(var(--radius) * 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    color: #fff;
    transition: height 0.12s ease;
  }
  .mw-card-play {
    position: absolute;
    bottom: 3px;
    right: 3px;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #fff;
    color: #000;
    display: grid;
    place-items: center;
  }
  .mw-card-info {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 0;
  }
  .mw-ct {
    font-size: 10px;
    font-weight: 700;
  }
  .mw-ca {
    font-size: 8.5px;
  }

  /* Mini Player Bar */
  .mw-playerbar {
    height: 44px;
    background: var(--surface);
    border-top: 2px solid var(--border);
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
    width: 80px;
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
    width: 20px;
    height: 20px;
    background: var(--accent);
    color: #fff;
    display: grid;
    place-items: center;
    border-radius: 50%;
    box-shadow: 0 var(--edge-depth) 0 color-mix(in srgb, var(--accent) 60%, #000);
    transition: border-radius 0.12s ease;
  }
  .mw-pb-play.squircle {
    border-radius: 5px;
  }
  .mw-pb-progress {
    display: flex;
    align-items: center;
    gap: 5px;
    width: 100%;
    max-width: 150px;
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
    background: var(--surface-2);
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

  /* Inspector Badges */
  .sim-inspector {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-top: 10px;
    border-top: 1.5px solid var(--border);
  }
  .insp-title {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }
  .insp-badges {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 6px;
  }
  .insp-badge {
    display: flex;
    align-items: center;
    gap: 5px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 5px 7px;
    font-size: 10.5px;
    min-width: 0;
  }
  .insp-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .insp-k {
    color: var(--text-muted);
    font-weight: 600;
    white-space: nowrap;
  }
  .insp-v {
    color: var(--text);
    font-weight: 700;
    margin-left: auto;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sim-tip-box {
    display: flex;
    align-items: flex-start;
    gap: 7px;
    background: color-mix(in srgb, var(--accent) 9%, var(--surface-2));
    border: 1.5px dashed var(--accent);
    border-radius: 8px;
    padding: 8px 10px;
    margin-top: 2px;
  }
  .sim-tip-icon {
    font-size: 13px;
    line-height: 1.2;
  }
  .sim-tip {
    font-size: 11px;
    line-height: 1.4;
    margin: 0;
  }

  @media (max-width: 1200px) {
    .settings-layout {
      grid-template-columns: 1fr;
    }
    .simulation-col {
      position: static;
    }
  }

  @media (max-width: 860px) {
    .grid {
      grid-template-columns: 1fr;
    }
    .span-2 {
      grid-column: span 1;
    }
    .insp-badges {
      grid-template-columns: 1fr;
    }
  }
</style>
