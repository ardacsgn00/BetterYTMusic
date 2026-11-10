<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { theme, ACCENT_PRESETS, type UiStyle } from '#lib/theme.svelte.ts';
  import Icon from './Icon.svelte';

  let currentStep = $state(0);
  let activeLyricIdx = $state(1);

  const previewColors = ACCENT_PRESETS.slice(0, 6);

  // Auto-advance lyrics preview on Step 2
  $effect(() => {
    if (currentStep !== 1) return;
    const timer = setInterval(() => {
      activeLyricIdx = (activeLyricIdx + 1) % 3;
    }, 2200);
    return () => clearInterval(timer);
  });

  let demoUiStyle = $state<'seamless' | 'boxed'>(theme.uiStyle);
  let userInteractedWithStyle = $state(false);

  // Auto-toggle demo UI style on Step 3 so the user visibly sees the morph animation
  $effect(() => {
    if (currentStep !== 2 || userInteractedWithStyle) return;
    const timer = setInterval(() => {
      demoUiStyle = demoUiStyle === 'seamless' ? 'boxed' : 'seamless';
    }, 2500);
    return () => clearInterval(timer);
  });

  function selectUiStyle(style: UiStyle) {
    userInteractedWithStyle = true;
    demoUiStyle = style;
    theme.setUiStyle(style);
  }

  const totalSteps = 5;
  const isLast = $derived(currentStep === totalSteps - 1);

  function next() {
    if (currentStep < totalSteps - 1) {
      currentStep++;
    } else {
      player.closeOnboarding();
    }
  }

  function prev() {
    if (currentStep > 0) {
      currentStep--;
    }
  }

  function setStep(idx: number) {
    currentStep = idx;
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      player.closeOnboarding();
    } else if (e.key === 'ArrowRight' && currentStep < totalSteps - 1) {
      currentStep++;
    } else if (e.key === 'ArrowLeft' && currentStep > 0) {
      currentStep--;
    }
  }
</script>

<svelte:window onkeydown={onKeydown} />

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<!-- Modal Backdrop -->
<div
  class="guide-backdrop"
  role="dialog"
  aria-modal="true"
  tabindex="-1"
  onclick={(e) => {
    if (e.target === e.currentTarget) player.closeOnboarding();
  }}
>
  <div class="guide-card">
    <!-- Header -->
    <header class="guide-header">
      <div class="brand">
        <span class="logo-box">
          <Icon name="play" size={18} />
        </span>
        <div class="brand-text">
          <span class="brand-name">{theme.appName.trim() || 'BetterYTMusic'}</span>
          <span class="brand-tag muted">Hızlı Başlangıç Rehberi</span>
        </div>
      </div>

      <div class="header-actions">
        <button
          type="button"
          class="btn ghost sm skip-btn"
          title="Tanıtımı kapat ve uygulamaya geç"
          onclick={player.closeOnboarding}
        >
          Turu Atla
        </button>
        <button
          type="button"
          class="btn icon sm close-btn"
          title="Kapat"
          aria-label="Kapat"
          onclick={player.closeOnboarding}
        >
          <Icon name="close" size={16} />
        </button>
      </div>
    </header>

    <!-- Main Content Stage -->
    <div class="guide-body">
      {#if currentStep === 0}
        <!-- Step 1: Ultra Light & Fast (Tauri 2 + Rust) -->
        <div class="step-pane step-fade">
          <div class="visual-wrap">
            <div class="vinyl-disc">
              <div class="vinyl-groove g1"></div>
              <div class="vinyl-groove g2"></div>
              <div class="vinyl-center" style:background-color={theme.accent}>
                <Icon name="play" size={18} />
              </div>
              <div class="pulse-ring" style:border-color={theme.accent}></div>
            </div>
            <div class="visual-badges">
              <span class="stat-pill"><Icon name="check" size={13} /> 0 ms Gecikme</span>
              <span class="stat-pill accent-pill"><Icon name="music" size={13} /> ~60 MB RAM</span>
              <span class="stat-pill"><Icon name="sparkles" size={13} /> Yerel Rust Motoru</span>
            </div>
          </div>

          <div class="step-text">
            <h2 class="step-title">Işık Hızında, Bağımsız & Hafif</h2>
            <p class="step-desc muted">
              Ağır tarayıcı sekmelerinden ve reklam takipçilerinden tamamen bağımsız. Tauri 2 ve yerel Rodio ses mimarisiyle minimum bellek tüketimi, anında başlayan müzik akışı.
            </p>
          </div>
        </div>

      {:else if currentStep === 1}
        <!-- Step 2: Live Lyrics & Song Radio -->
        <div class="step-pane step-fade">
          <div class="visual-wrap">
            <div class="karaoke-card">
              <div class="karaoke-head">
                <span class="karaoke-dot"></span>
                <span class="karaoke-tag muted">CANLI SENKRONİZE SÖZLER</span>
                <div class="sound-wave" aria-hidden="true">
                  <span class="sw-bar b1" style:background-color={theme.accent}></span>
                  <span class="sw-bar b2" style:background-color={theme.accent}></span>
                  <span class="sw-bar b3" style:background-color={theme.accent}></span>
                  <span class="sw-bar b4" style:background-color={theme.accent}></span>
                </div>
              </div>
              <div class="karaoke-lines">
                <div class="k-line" class:active={activeLyricIdx === 0}>
                  <span class="k-idx">1:12</span>
                  <span>Müziğe göre gerçek zamanlı akan sözler</span>
                </div>
                <div class="k-line" class:active={activeLyricIdx === 1}>
                  <span class="k-idx">1:16</span>
                  <span>Satıra tıklayarak doğrudan o saniyeye atla</span>
                </div>
                <div class="k-line" class:active={activeLyricIdx === 2}>
                  <span class="k-idx">1:21</span>
                  <span>Şarkı radyosu ile kesintisiz benzer parçalar</span>
                </div>
              </div>
            </div>

            <div class="visual-badges">
              <span class="stat-pill"><Icon name="lyrics" size={13} /> Canlı LRC Akışı</span>
              <span class="stat-pill accent-pill"><Icon name="radio" size={13} /> Şarkı Radyosu</span>
            </div>
          </div>

          <div class="step-text">
            <h2 class="step-title">Canlı Şarkı Sözleri & Şarkı Radyosu</h2>
            <p class="step-desc muted">
              Dinle ekranında müziğe eşzamanlı akan şarkı sözlerini takip et. Herhangi bir satıra tıklayarak o ana atlayabilir, alt bardaki radyo ikonuyla şarkıya özel sonsuz çalma listesi başlatabilirsin.
            </p>
          </div>
        </div>

      {:else if currentStep === 2}
        <!-- Step 3: Tactile 3D, Seamless Mode & Customization with Live Morphing Animation -->
        <div class="step-pane step-fade">
          <div class="visual-wrap">
            <div class="theme-showcase">
              <div class="theme-head">
                <span class="theme-hint muted">Canlı Vurgu Rengi:</span>
                <div class="color-swatches">
                  {#each previewColors as c (c.value)}
                    <button
                      type="button"
                      class="color-dot"
                      class:active={theme.accent === c.value}
                      style:background-color={c.value}
                      title={c.name}
                      aria-label={c.name}
                      onclick={() => theme.setAccent(c.value)}
                    >
                      {#if theme.accent === c.value}
                        <Icon name="check" size={13} />
                      {/if}
                    </button>
                  {/each}
                </div>
              </div>

              <!-- Animated Live Mini UI Mockup Card showing the visual transformation -->
              <div
                class="ui-morph-card"
                class:is-boxed={demoUiStyle === 'boxed'}
                class:is-seamless={demoUiStyle === 'seamless'}
              >
                <div class="mock-card-inner">
                  <div class="mock-cover-box" style:background-color={theme.accent}>
                    <Icon name="now" size={20} />
                  </div>
                  <div class="mock-meta">
                    <div class="mock-badge-row">
                      <span class="mock-mode-pill" class:boxed-pill={demoUiStyle === 'boxed'}>
                        {demoUiStyle === 'boxed' ? '3B DOKUNSAL (KUTULU)' : 'AKICI (KUTUSUZ)'}
                      </span>
                      {#if !userInteractedWithStyle}
                        <span class="auto-anim-pill">Canlı Dönüşüm</span>
                      {/if}
                    </div>
                    <span class="mock-song truncate">Süpermix & Çalma Listesi</span>
                    <span class="mock-artist truncate muted">
                      {demoUiStyle === 'boxed' ? 'Dokunsal 3B butonlar ve 2px kutulu çerçeveler' : 'Modern, çerçevesiz ve göz yormayan akıcı düzen'}
                    </span>
                  </div>
                  <button
                    type="button"
                    class="mock-play-btn"
                    class:tactile-btn={demoUiStyle === 'boxed'}
                    style:background-color={theme.accent}
                    title="Önizleme Butonu"
                    aria-label="Önizleme Butonu"
                  >
                    <Icon name="play" size={14} />
                  </button>
                </div>
              </div>

              <!-- Interactive Style Toggle Switch -->
              <div class="style-toggle-row">
                <button
                  type="button"
                  class="btn sm style-btn"
                  class:accent={demoUiStyle === 'seamless'}
                  onclick={() => selectUiStyle('seamless')}
                >
                  Kutusuz (Sade)
                </button>
                <button
                  type="button"
                  class="btn sm style-btn"
                  class:accent={demoUiStyle === 'boxed'}
                  onclick={() => selectUiStyle('boxed')}
                >
                  Kutulu (3B Dokunsal)
                </button>
              </div>
            </div>

            <div class="visual-badges">
              <span class="stat-pill"><Icon name="palette" size={13} /> 10 Renk Paleti</span>
              <span class="stat-pill accent-pill"><Icon name="mini" size={13} /> Mini Oynatıcı</span>
              <span class="stat-pill"><Icon name="settings" size={13} /> Zoom Yönetimi</span>
            </div>
          </div>

          <div class="step-text">
            <h2 class="step-title">Senin Tarzın, Senin Arayüzün</h2>
            <p class="step-desc muted">
              İster ferah kutusuz tasarım, ister dokunsal 3B butonlar. Renk vurgularını, yazı boyutlarını, mini oynatıcı düzenini ve arayüz derinliğini Ayarlar panelinden anında kişiselleştirebilirsin.
            </p>
          </div>
        </div>

      {:else if currentStep === 3}
        <!-- Step 4: Discord Rich Presence -->
        <div class="step-pane step-fade">
          <div class="visual-wrap">
            <div class="onboard-discord-card">
              <div class="ob-d-head">
                <div class="ob-d-user">
                  <div class="ob-d-avatar">
                    <Icon name="now" size={16} />
                    <span class="ob-d-status-dot"></span>
                  </div>
                  <div class="ob-d-meta">
                    <span class="ob-d-name">{player.auth.accountName || 'BetterYTMusic'}</span>
                    <span class="ob-d-tag">BİR ŞEY DİNLİYOR</span>
                  </div>
                </div>
                <span class="ob-d-badge">
                  <Icon name="sparkles" size={12} /> Canlı Durum
                </span>
              </div>

              <div class="ob-d-body">
                <div class="ob-d-art" style:background-color={theme.accent}>
                  <Icon name="music" size={24} />
                  <span class="ob-d-play-pip">
                    <Icon name="play" size={10} />
                  </span>
                </div>
                <div class="ob-d-info">
                  <span class="ob-d-title">{theme.appName.trim() || 'BetterYTMusic'}</span>
                  <span class="ob-d-song truncate">{player.state.track?.title || 'Süpermix & Çalma Listeleri'}</span>
                  <span class="ob-d-artist truncate muted">{player.state.track?.artist || 'YouTube Music Akışı'}</span>
                  <div class="ob-d-progress">
                    <div class="ob-d-bar">
                      <div class="ob-d-bar-fill"></div>
                    </div>
                    <span class="ob-d-time">0:45 / 3:20</span>
                  </div>
                </div>
              </div>
            </div>

            <div class="visual-badges">
              <span class="stat-pill"><Icon name="sparkles" size={13} /> 0 Kurulum Gerekir</span>
              <span class="stat-pill accent-pill"><Icon name="music" size={13} /> Dinliyor Durumu</span>
              <span class="stat-pill"><Icon name="settings" size={13} /> Canlı Süre Çubuğu</span>
            </div>
          </div>

          <div class="step-text">
            <h2 class="step-title">Discord'da Dinleme Durumun Canlı</h2>
            <p class="step-desc muted">
              Müzik zevkini arkadaşlarınla paylaş. BetterYTMusic açıkken çalan şarkı, sanatçı ve canlı geçen süre Discord profilinde hiçbir ek ayar yapmadan doğrudan "Dinliyor" olarak görünür.
            </p>
          </div>
        </div>

      {:else if currentStep === 4}
        <!-- Step 5: Library & Google Sync -->
        <div class="step-pane step-fade">
          <div class="visual-wrap">
            <div class="auth-card">
              <div class="auth-icon-wrap" style:background-color={theme.accent}>
                <Icon name="user" size={32} />
              </div>
              <div class="auth-meta">
                <span class="auth-title">YouTube Music Arşivin</span>
                <span class="auth-sub muted">Kişisel çalma listelerin, beğenilen şarkıların ve süpermix'in</span>
              </div>
            </div>

            <div class="auth-actions">
              <button type="button" class="btn accent main-auth-btn" onclick={player.login}>
                <Icon name="user" size={17} />
                Google ile Oturum Aç
              </button>
              <button type="button" class="btn guest-auth-btn" onclick={player.closeOnboarding}>
                <Icon name="play" size={17} />
                Misafir Olarak Başla (Hesapsız)
              </button>
            </div>
          </div>

          <div class="step-text">
            <h2 class="step-title">Müziğin Seni Bekliyor!</h2>
            <p class="step-desc muted">
              Google hesabınla bağlanarak kitaplığını senkronize edebilir ya da hesap bağlamadan arama, radyo ve çalma listelerini sınırsızca hemen dinlemeye başlayabilirsin.
            </p>
          </div>
        </div>
      {/if}
    </div>

    <!-- Footer Controls -->
    <footer class="guide-footer">
      <div class="footer-left">
        {#if currentStep > 0}
          <button type="button" class="btn ghost sm" onclick={prev}>
            Geri
          </button>
        {/if}
      </div>

      <!-- Step Indicator Dots -->
      <div class="step-dots" role="tablist" aria-label="Adım seçimi">
        {#each Array(totalSteps) as _, idx}
          <button
            type="button"
            class="dot-btn"
            class:active={currentStep === idx}
            style:background-color={currentStep === idx ? theme.accent : undefined}
            title={`Adım ${idx + 1}`}
            aria-label={`Adım ${idx + 1}`}
            onclick={() => setStep(idx)}
          ></button>
        {/each}
      </div>

      <div class="footer-right">
        {#if !isLast}
          <button type="button" class="btn accent sm" onclick={next}>
            Devam Et
            <Icon name="next" size={14} />
          </button>
        {:else}
          <button type="button" class="btn accent sm" onclick={player.closeOnboarding}>
            <Icon name="check" size={15} />
            Müziğe Başla
          </button>
        {/if}
      </div>
    </footer>
  </div>
</div>

<style>
  .guide-backdrop {
    position: fixed;
    inset: 0;
    z-index: 1000;
    background: rgba(0, 0, 0, 0.75);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px;
    animation: fade-in 160ms ease-out;
  }

  .guide-card {
    width: min(640px, 94vw);
    max-height: min(720px, 90vh);
    background: var(--surface) !important;
    border: 2px solid var(--border) !important;
    border-radius: var(--radius);
    box-shadow: 0 28px 80px rgba(0, 0, 0, 0.85), 0 var(--edge-depth) 0 var(--edge);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: pop-in 200ms cubic-bezier(0.16, 1, 0.3, 1);
  }

  /* Header */
  .guide-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
    background: var(--surface-2) !important;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .logo-box {
    width: 32px;
    height: 32px;
    border-radius: var(--radius-sm);
    background: var(--accent);
    color: var(--on-accent);
    display: grid;
    place-items: center;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }
  .brand-text {
    display: flex;
    flex-direction: column;
    line-height: 1.2;
  }
  .brand-name {
    font-size: 14px;
    font-weight: 800;
    letter-spacing: -0.01em;
  }
  .brand-tag {
    font-size: 11px;
    font-weight: 600;
  }
  .header-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .skip-btn {
    font-size: 12.5px;
    font-weight: 700;
  }

  /* Body & Step Animations */
  .guide-body {
    background: var(--surface) !important;
    padding: 28px 24px 20px;
    min-height: 380px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    overflow-y: auto;
  }

  .step-pane {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 22px;
  }

  .step-fade {
    animation: step-appear 220ms cubic-bezier(0.16, 1, 0.3, 1);
  }

  .step-text {
    max-width: 520px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .step-title {
    font-size: 22px;
    font-weight: 800;
    letter-spacing: -0.02em;
    line-height: 1.25;
  }
  .step-desc {
    font-size: 13.5px;
    line-height: 1.55;
  }

  .visual-wrap {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
  }

  .visual-badges {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 8px;
  }
  .stat-pill {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 5px 12px;
    border-radius: 99px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    font-size: 12px;
    font-weight: 700;
  }
  .stat-pill.accent-pill {
    border-color: var(--accent);
    color: var(--accent);
    background: var(--accent-soft);
  }

  /* Step 1: Animated Vinyl Disc */
  .vinyl-disc {
    width: 110px;
    height: 110px;
    border-radius: 50%;
    background: radial-gradient(circle, #2b2b2e 0%, #151518 70%, #0d0d10 100%);
    border: 3px solid #333;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.6);
    position: relative;
    display: grid;
    place-items: center;
    animation: spin 9s linear infinite;
  }
  .vinyl-groove {
    position: absolute;
    border-radius: 50%;
    border: 1px dashed rgba(255, 255, 255, 0.12);
  }
  .vinyl-groove.g1 {
    width: 82px;
    height: 82px;
  }
  .vinyl-groove.g2 {
    width: 58px;
    height: 58px;
  }
  .vinyl-center {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    color: #fff;
    display: grid;
    place-items: center;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
    z-index: 2;
  }
  .pulse-ring {
    position: absolute;
    inset: -8px;
    border-radius: 50%;
    border: 2px solid var(--accent);
    opacity: 0;
    animation: pulse-wave 2.4s ease-out infinite;
  }

  /* Step 2: Karaoke Lyrics Card */
  .karaoke-card {
    width: min(440px, 100%);
    background: var(--surface-2) !important;
    border: 2px solid var(--border) !important;
    border-radius: var(--radius);
    padding: 14px 18px;
    text-align: left;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.35);
  }
  .karaoke-head {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 10px;
  }
  .karaoke-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #06d6a0;
  }
  .karaoke-tag {
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 0.05em;
  }
  .sound-wave {
    margin-left: auto;
    display: flex;
    align-items: flex-end;
    gap: 3px;
    height: 14px;
  }
  .sw-bar {
    width: 3px;
    border-radius: 2px;
    animation: wave-bar 1s ease-in-out infinite alternate;
  }
  .sw-bar.b1 { height: 60%; animation-delay: 0ms; }
  .sw-bar.b2 { height: 100%; animation-delay: 200ms; }
  .sw-bar.b3 { height: 40%; animation-delay: 400ms; }
  .sw-bar.b4 { height: 80%; animation-delay: 150ms; }

  .karaoke-lines {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .k-line {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 10px;
    border-radius: var(--radius-sm);
    font-size: 13px;
    font-weight: 600;
    color: var(--text-muted);
    transition: all 180ms ease;
  }
  .k-idx {
    font-size: 11px;
    font-variant-numeric: tabular-nums;
    opacity: 0.6;
  }
  .k-line.active {
    background: var(--surface-3);
    color: var(--text);
    font-weight: 700;
    transform: translateX(4px);
    box-shadow: inset 3px 0 0 var(--accent);
  }

  /* Step 3: Theme Showcase & Live UI Morph Animation */
  .theme-showcase {
    width: min(460px, 100%);
    padding: 14px 16px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    background: var(--surface-2) !important;
    border: 2px solid var(--border) !important;
    border-radius: var(--radius);
  }
  .theme-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 0 4px;
  }
  .theme-hint {
    font-size: 12px;
    font-weight: 700;
  }
  .color-swatches {
    display: flex;
    gap: 7px;
  }
  .color-dot {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    border: 2px solid transparent;
    color: #fff;
    cursor: pointer;
    display: grid;
    place-items: center;
    transition: transform 120ms ease, box-shadow 120ms ease;
  }
  .color-dot:hover {
    transform: scale(1.15);
  }
  .color-dot.active {
    border-color: #fff;
    transform: scale(1.12);
    box-shadow: 0 0 10px rgba(255, 255, 255, 0.4);
  }

  /* Morphing Live Preview Card */
  .ui-morph-card {
    width: 100%;
    border-radius: var(--radius);
    padding: 12px 14px;
    background: var(--surface);
    transition: all 260ms cubic-bezier(0.16, 1, 0.3, 1);
  }
  .ui-morph-card.is-seamless {
    border: 1px solid rgba(255, 255, 255, 0.08);
    box-shadow: 0 6px 18px rgba(0, 0, 0, 0.3);
    transform: translateY(0);
  }
  .ui-morph-card.is-boxed {
    border: 2px solid var(--border);
    box-shadow: 0 4px 0 var(--edge);
    transform: translateY(-2px);
  }

  .mock-card-inner {
    display: flex;
    align-items: center;
    gap: 12px;
    text-align: left;
  }
  .mock-cover-box {
    width: 44px;
    height: 44px;
    border-radius: var(--radius-sm);
    color: var(--on-accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
    transition: background-color 160ms ease;
  }
  .mock-meta {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .mock-badge-row {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 2px;
  }
  .mock-mode-pill {
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 0.04em;
    padding: 1px 6px;
    border-radius: 4px;
    background: var(--surface-3);
    color: var(--text-muted);
    transition: all 200ms ease;
  }
  .mock-mode-pill.boxed-pill {
    background: var(--accent-soft);
    color: var(--accent);
  }
  .auto-anim-pill {
    font-size: 9.5px;
    font-weight: 700;
    color: var(--accent);
    animation: pulse-soft 1.8s ease-in-out infinite;
  }
  @keyframes pulse-soft {
    0%, 100% { opacity: 0.55; }
    50% { opacity: 1; }
  }
  .mock-song {
    font-size: 13px;
    font-weight: 800;
  }
  .mock-artist {
    font-size: 11px;
    line-height: 1.25;
  }
  .mock-play-btn {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    border: none;
    color: var(--on-accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
    cursor: pointer;
    transition: all 200ms cubic-bezier(0.16, 1, 0.3, 1);
  }
  .mock-play-btn.tactile-btn {
    box-shadow: 0 3px 0 rgba(0, 0, 0, 0.45);
    transform: translateY(-1.5px);
  }

  .style-toggle-row {
    display: flex;
    gap: 8px;
    width: 100%;
  }
  .style-btn {
    flex: 1;
    font-size: 12px;
    font-weight: 700;
    justify-content: center;
  }

  /* Step 4: Discord Card in Onboarding */
  .onboard-discord-card {
    width: min(450px, 100%);
    padding: 14px 16px;
    background: #1e1f22 !important;
    border: 2px solid var(--border) !important;
    border-radius: var(--radius);
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: 12px;
    box-shadow: 0 4px 0 var(--edge);
  }
  .ob-d-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .ob-d-user {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .ob-d-avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: var(--surface-3);
    color: var(--accent);
    display: grid;
    place-items: center;
    position: relative;
  }
  .ob-d-status-dot {
    position: absolute;
    bottom: -1px;
    right: -1px;
    width: 9px;
    height: 9px;
    background: #23a55a;
    border-radius: 50%;
    border: 2px solid #1e1f22;
  }
  .ob-d-meta {
    display: flex;
    flex-direction: column;
  }
  .ob-d-name {
    font-size: 13px;
    font-weight: 800;
    color: #f2f3f5;
  }
  .ob-d-tag {
    font-size: 10px;
    font-weight: 800;
    color: #b5bac1;
    letter-spacing: 0.04em;
  }
  .ob-d-badge {
    font-size: 11px;
    font-weight: 700;
    padding: 3px 8px;
    border-radius: 6px;
    background: rgba(35, 165, 90, 0.15);
    color: #23a55a;
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .ob-d-body {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    background: #2b2d31;
    border-radius: var(--radius-sm);
    border: 1px solid rgba(255, 255, 255, 0.06);
  }
  .ob-d-art {
    width: 52px;
    height: 52px;
    border-radius: 8px;
    display: grid;
    place-items: center;
    color: #ffffff;
    position: relative;
    flex-shrink: 0;
  }
  .ob-d-play-pip {
    position: absolute;
    right: -3px;
    bottom: -3px;
    width: 17px;
    height: 17px;
    border-radius: 50%;
    background: #23a55a;
    border: 2px solid #2b2d31;
    display: grid;
    place-items: center;
    color: #ffffff;
  }
  .ob-d-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .ob-d-title {
    font-size: 13px;
    font-weight: 800;
    color: #f2f3f5;
  }
  .ob-d-song {
    font-size: 12px;
    font-weight: 700;
    color: #dbdee1;
  }
  .ob-d-artist {
    font-size: 11px;
    color: #949ba4;
  }
  .ob-d-progress {
    margin-top: 4px;
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .ob-d-bar {
    flex: 1;
    height: 4px;
    background: #4e5058;
    border-radius: 2px;
    overflow: hidden;
  }
  .ob-d-bar-fill {
    height: 100%;
    background: #ffffff;
    border-radius: 2px;
    animation: ob-flow 3.5s ease-in-out infinite alternate;
  }
  @keyframes ob-flow {
    0% { width: 20%; }
    100% { width: 85%; }
  }
  .ob-d-time {
    font-size: 10px;
    color: #949ba4;
    font-family: monospace;
  }

  /* Step 5: Auth Card */
  .auth-card {
    width: min(440px, 100%);
    padding: 18px;
    display: flex;
    align-items: center;
    gap: 16px;
    background: var(--surface-2) !important;
    border: 2px solid var(--border) !important;
    border-radius: var(--radius);
    text-align: left;
  }
  .auth-icon-wrap {
    width: 52px;
    height: 52px;
    border-radius: var(--radius);
    color: var(--on-accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }
  .auth-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .auth-title {
    font-size: 15px;
    font-weight: 800;
  }
  .auth-sub {
    font-size: 12.5px;
    line-height: 1.35;
  }
  .auth-actions {
    width: min(440px, 100%);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .main-auth-btn {
    height: 44px;
    font-size: 14px;
    font-weight: 800;
  }
  .guest-auth-btn {
    height: 40px;
    font-size: 13.5px;
    font-weight: 700;
  }

  /* Footer Controls */
  .guide-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 20px;
    border-top: 1px solid var(--border);
    background: var(--surface-2) !important;
  }
  .footer-left,
  .footer-right {
    min-width: 90px;
    display: flex;
  }
  .footer-right {
    justify-content: flex-end;
  }

  /* Dots */
  .step-dots {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .dot-btn {
    width: 8px;
    height: 8px;
    padding: 0;
    border: none;
    border-radius: 50%;
    background: var(--border);
    cursor: pointer;
    transition: all 180ms cubic-bezier(0.16, 1, 0.3, 1);
  }
  .dot-btn:hover {
    background: var(--text-muted);
  }
  .dot-btn.active {
    width: 24px;
    border-radius: 99px;
  }

  /* Keyframe Animations */
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
  @keyframes pulse-wave {
    0% { transform: scale(0.95); opacity: 0.8; }
    100% { transform: scale(1.35); opacity: 0; }
  }
  @keyframes wave-bar {
    0% { transform: scaleY(0.3); }
    100% { transform: scaleY(1); }
  }
  @keyframes fade-in {
    from { opacity: 0; }
    to { opacity: 1; }
  }
  @keyframes pop-in {
    from { opacity: 0; transform: scale(0.96) translateY(8px); }
    to { opacity: 1; transform: scale(1) translateY(0); }
  }
  @keyframes step-appear {
    from { opacity: 0; transform: translateY(6px); }
    to { opacity: 1; transform: translateY(0); }
  }

  @media (max-width: 580px) {
    .guide-card {
      width: 100%;
      height: 100%;
      max-height: 100%;
      border-radius: 0;
    }
    .guide-body {
      padding: 18px 14px;
    }
  }
</style>
