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
    { id: 'sm', label: 'Kompakt' },
    { id: 'md', label: 'Standart' },
    { id: 'lg', label: 'Geniş' }
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
    { id: 'comfortable', label: 'Dengeli' },
    { id: 'compact', label: 'Sıkı Liste' }
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
      <h2>Sayfa Ölçeği (Zoom) & Liste Yoğunluğu</h2>
      <p class="muted">Sayfa görünürlük oranını (%75–%135), metin boyutunu ve panel genişliklerini yönet.</p>
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
        <span class="sub-label muted">Metin Ölçeği & Liste Görünümü</span>
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
</section>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 20px;
    width: 100%;
    max-width: 1320px;
    padding-bottom: 24px;
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

  @media (max-width: 860px) {
    .grid {
      grid-template-columns: 1fr;
    }
    .span-2 {
      grid-column: span 1;
    }
  }
</style>
