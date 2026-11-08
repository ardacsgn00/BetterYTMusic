<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { theme, ACCENT_PRESETS } from '#lib/theme.svelte.ts';
  import Icon, { type IconName } from './Icon.svelte';

  let currentStep = $state(0);

  // Micro-animations & interactive state
  let activeLyricIdx = $state(1); // 0, 1, 2
  let previewAccentIdx = $state(0);
  let userOverrodeAccent = $state(false);
  let previewSidebarCollapsed = $state(true);
  let activeDeviceIdx = $state(0); // 0 = Speaker, 1 = Bluetooth
  let mockPlaying = $state(true);

  const previewAccents = [
    { name: 'YT Kırmızı', value: '#ff4e45' },
    { name: 'Kobalt Mavi', value: '#3a86ff' },
    { name: 'Zümrüt Mint', value: '#06d6a0' },
    { name: 'Kehribar Sarı', value: '#ffbe0b' },
    { name: 'Mor', value: '#8338ec' },
    { name: 'Mercan Pembe', value: '#ef476f' }
  ];

  const currentPreviewAccent = $derived(
    userOverrodeAccent ? theme.accent : previewAccents[previewAccentIdx].value
  );

  $effect(() => {
    const timer = setInterval(() => {
      // Step 2: Synchronized lyrics line auto-advance
      if (currentStep === 1) {
        activeLyricIdx = (activeLyricIdx + 1) % 3;
      }
      // Step 4: Theme accent & sidebar auto-rotation
      else if (currentStep === 3 && !userOverrodeAccent) {
        previewAccentIdx = (previewAccentIdx + 1) % previewAccents.length;
        if (previewAccentIdx % 2 === 0) {
          previewSidebarCollapsed = !previewSidebarCollapsed;
        }
      }
      // Step 5: Sound output device auto-switch
      else if (currentStep === 4) {
        activeDeviceIdx = (activeDeviceIdx + 1) % 2;
      }
    }, 2400);

    return () => clearInterval(timer);
  });

  interface StepInfo {
    badge: string;
    icon: IconName;
    title: string;
    subtitle: string;
    points: { icon: IconName; title: string; desc: string }[];
  }

  const steps: StepInfo[] = [
    {
      badge: '01 / Başlangıç & Performans',
      icon: 'now',
      title: "BetterYTMusic'e Hoş Geldin!",
      subtitle: 'Sıfır tarayıcı şişkinliği ile ultra hızlı, hafif ve bağımsız masaüstü müzik istemcisi.',
      points: [
        {
          icon: 'play',
          title: 'Tauri 2 & Rust Gücü',
          desc: 'Yerel Symphonia ve Rodio ses mimarisiyle minimum bellek ve CPU tüketimi; anında başlayan ses akışı.'
        },
        {
          icon: 'check',
          title: 'Sade & Odaklı Deneyim',
          desc: 'Ağır reklam takipçilerinden ve onlarca tarayıcı sekmesinden bağımsız, sadece müziğe odaklı tasarım.'
        }
      ]
    },
    {
      badge: '02 / Dinle Ekranı & Sözler',
      icon: 'lyrics',
      title: 'Canlı Şarkı Sözleri & Esnek Paneller',
      subtitle: 'Müziğe göre gerçek zamanlı kayan sözler ve farenizle boyutlandırabileceğiniz sütunlar.',
      points: [
        {
          icon: 'lyrics',
          title: 'Eşzamanlı Kayan Sözler (LRC)',
          desc: 'Şarkı çalarken sözler satır satır akar; herhangi bir dizeye tıklayarak doğrudan o saniyeye atlayabilirsin.'
        },
        {
          icon: 'columns',
          title: 'Fareyle Sürüklenebilir Bölmeler',
          desc: 'Kapak, sözler ve sıradaki liste arasındaki dikey çizgilerden tutarak genişlikleri serbestçe daraltıp genişlet.'
        }
      ]
    },
    {
      badge: '03 / Ana Sayfa & Keşif',
      icon: 'home',
      title: 'Süpermix & Çalma Listeleri',
      subtitle: 'YouTube Music mikslerin, arşivlerin ve çalma listesi parça detayları parmaklarının ucunda.',
      points: [
        {
          icon: 'play',
          title: 'Kapak Üzerinden Tek Tıkla Çal',
          desc: 'Kapağın ortasındaki oynat simgesine tıklayarak çalma listesini doğrudan başlatabilirsin.'
        },
        {
          icon: 'now',
          title: 'Liste İçeriğini İncele',
          desc: 'Kartın gövdesine tıklandığında listedeki tüm parçalar listelenir; istediğin şarkıyı seçip başlatabilirsin.'
        }
      ]
    },
    {
      badge: '04 / Tasarım & Özelleştirme',
      icon: 'palette',
      title: 'Tamamen Kişiselleştirilebilir Arayüz',
      subtitle: '10 renk paleti, OLED/Koyu/Slate temaları, sayfa zoom ayarı ve daraltılabilir menü.',
      points: [
        {
          icon: 'palette',
          title: 'Dokunsal 3B Butonlar & Vurgu Renkleri',
          desc: 'Taktil alt kenar derinliği, köşe yuvarlaklığı ve canlı renklerle arayüzü tam zevkine göre uyarla.'
        },
        {
          icon: 'search',
          title: 'Tek Tıkla Zoom & Daraltılabilir Sol Menü',
          desc: 'Ana sayfadaki zoom menüsünden %75–%135 görünüm seç; sol menüyü daraltıp tek tıkla ikon moduna geçir.'
        }
      ]
    },
    {
      badge: '05 / Kesintisiz Ses & Sistem',
      icon: 'speaker',
      title: 'Ses Çıkış Cihazı & Arka Plan',
      subtitle: 'Kulaklık ve hoparlör arasında anında geçiş yap; arka planda kesintisiz müzik dinle.',
      points: [
        {
          icon: 'speaker',
          title: 'Hızlı Ses Çıkış Seçimi',
          desc: 'Alt bardaki hoparlör simgesine tıklayarak kulaklık veya hoparlörünü şarkı duraksamadan anında değiştir.'
        },
        {
          icon: 'settings',
          title: 'Sistem Tepsisi & Görev Çubuğu',
          desc: 'Pencereyi kapatsan bile müzik çalmaya devam eder; klavye medya tuşlarıyla dilediğin an yönet.'
        }
      ]
    },
    {
      badge: '06 / Son Adım: Hesap Girişi',
      icon: 'user',
      title: 'Kitaplığını & Beğendiklerini Eşitle',
      subtitle: 'YouTube Music hesabınla oturum açarak tüm kişisel arşivine tek tıkla ulaş.',
      points: [
        {
          icon: 'user',
          title: 'Kişisel Çalma Listelerin & Beğenilenler',
          desc: 'Google hesabınla bağlanarak kendi listelerine, beğenilen şarkılarına ve özel mikslerine eriş.'
        },
        {
          icon: 'check',
          title: 'Misafir Olarak da Kullanabilirsin',
          desc: 'Hesap bağlamadan da tüm arama, dinleme ve radyo özelliklerini sınırsızca kullanmaya devam edebilirsin.'
        }
      ]
    }
  ];

  const totalSteps = steps.length;
  const isLast = $derived(currentStep === totalSteps - 1);
  const step = $derived(steps[currentStep]);

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

<div class="fullscreen-guide" role="dialog" aria-modal="true" tabindex="-1">
  <!-- Top Navigation Bar -->
  <header class="guide-header">
    <div class="brand">
      <span class="logo"><Icon name="play" size={20} /></span>
      <div class="brand-text">
        <span class="brand-name">{theme.appName.trim() || 'BetterYTMusic'}</span>
        <span class="brand-tag muted">Başlangıç & Özellik Rehberi</span>
      </div>
    </div>

    <div class="header-actions">
      <!-- Explicit Skip Button Requested by User -->
      <button
        type="button"
        class="btn skip-btn"
        title="Tanıtım turunu kapat ve doğrudan uygulamaya geç"
        onclick={player.closeOnboarding}
      >
        <Icon name="close" size={15} />
        Turu Atla
      </button>

      <button
        type="button"
        class="btn icon close-icon-btn"
        title="Kapat"
        aria-label="Kapat"
        onclick={player.closeOnboarding}
      >
        <Icon name="close" size={18} />
      </button>
    </div>
  </header>

  <!-- Main Showcase Stage (Balanced 2-Column Wide Screen Layout) -->
  <main class="guide-stage">
    <div class="stage-container">
      <!-- Left Column: Title, Subtitle & Value Cards -->
      <section class="stage-info">
        <div class="badge-row">
          <span class="badge accent-badge">{step.badge}</span>
          <span class="step-num muted">Adım {currentStep + 1} / {totalSteps}</span>
        </div>

        <h1 class="stage-title">{step.title}</h1>
        <p class="stage-subtitle muted">{step.subtitle}</p>

        <div class="cards-list">
          {#each step.points as point}
            <div class="info-card">
              <div class="card-icon">
                <Icon name={point.icon} size={22} />
              </div>
              <div class="card-content">
                <h3 class="card-title">{point.title}</h3>
                <p class="card-desc muted">{point.desc}</p>
              </div>
            </div>
          {/each}
        </div>

        {#if isLast}
          <div class="auth-action-block">
            <button type="button" class="btn accent full main-auth-btn" onclick={player.login}>
              <Icon name="user" size={18} />
              Google ile Bağlan
            </button>
            <button type="button" class="btn full guest-auth-btn" onclick={player.closeOnboarding}>
              Misafir Olarak Başla (Hesapsız Devam Et)
            </button>
          </div>
        {/if}
      </section>

      <!-- Right Column: Interactive Visual Simulation / Spotlight Mockup -->
      <aside class="stage-visual">
        <div class="card visual-panel">
          {#if currentStep === 0}
            <!-- Step 1: Performance & Architecture Showcase -->
            <div class="mockup-step1">
              <div class="hero-logo-wrap">
                <div class="hero-logo-large">
                  <Icon name="play" size={52} />
                </div>
                <div class="pulse-ring"></div>
              </div>

              <!-- Animated Equalizer Spectrum -->
              <div class="live-spectrum-card">
                <div class="spectrum-head">
                  <span class="live-ping"></span>
                  <span class="spec-title">CANLI SES DALGASI & SPEKTRUM</span>
                  <span class="spec-codec">AAC 256kbps · 48kHz</span>
                </div>
                <div class="eq-bars-row">
                  {#each [35, 65, 95, 75, 45, 85, 100, 50, 70, 92, 65, 80, 40, 72, 88, 55] as baseH, i}
                    <div class="eq-bar" style="--base-h: {baseH}%; animation-delay: {i * 85}ms"></div>
                  {/each}
                </div>
              </div>

              <div class="stat-pills">
                <div class="stat-pill">
                  <span class="sp-val accent-txt">0 ms</span>
                  <span class="sp-lbl muted">Yerel Ses Gecikmesi</span>
                </div>
                <div class="stat-pill">
                  <span class="sp-val green-txt">~60 MB</span>
                  <span class="sp-lbl muted">Düşük RAM Tüketimi</span>
                </div>
                <div class="stat-pill">
                  <span class="sp-val">Tauri 2 + Rust</span>
                  <span class="sp-lbl muted">Rodio + Symphonia</span>
                </div>
              </div>
              <div class="mini-note muted">
                Windows 10/11 için optimize edilmiş doğrudan donanım ses sürücüsü.
              </div>
            </div>
          {:else if currentStep === 1}
            <!-- Step 2: Now Playing & Live Resizing Split Showcase -->
            <div class="mockup-step2">
              <div class="mock-panel-wrap auto-resizing">
                <div class="mock-col mock-cover-col">
                  <div class="mock-cover-box">
                    <Icon name="now" size={32} />
                    <div class="vinyl-groove"></div>
                  </div>
                  <span class="mock-song-t truncate">Anadolu Flex</span>
                  <span class="mock-song-a truncate muted">Murda ve Ezhel</span>
                </div>

                <div class="mock-divider" title="Fareyle sürüklenebilir sınır">
                  <span class="divider-grip"></span>
                  <div class="drag-ghost-tag">
                    <Icon name="move-horizontal" size={14} />
                  </div>
                </div>

                <div class="mock-col mock-lyrics-col">
                  <div class="lrc-head">
                    <span class="mock-badge">Canlı Senkronize Sözler</span>
                    <span class="lrc-source">LRC · Otomatik Kayar</span>
                  </div>
                  <div class="lrc-scroll-box">
                    <p class="lrc-line" class:active={activeLyricIdx === 0}>
                      {#if activeLyricIdx === 0}<span class="lrc-play-arrow"><Icon name="play" size={9} /></span>{/if}
                      Harbi Anadolu Flex, aldırmadı bi' nefes
                    </p>
                    <p class="lrc-line" class:active={activeLyricIdx === 1}>
                      {#if activeLyricIdx === 1}<span class="lrc-play-arrow"><Icon name="play" size={9} /></span>{/if}
                      Müziğe göre eşzamanlı söz akışı
                    </p>
                    <p class="lrc-line" class:active={activeLyricIdx === 2}>
                      {#if activeLyricIdx === 2}<span class="lrc-play-arrow"><Icon name="play" size={9} /></span>{/if}
                      Satıra tıklayarak doğrudan o saniyeye atla
                    </p>
                  </div>
                </div>
              </div>
              <div class="mock-hint-bar">
                <span class="hint-pulse"><Icon name="lightbulb" size={15} /></span>
                <span class="mock-hint muted">
                  Sütun sınırları otomatik olarak genişleyip daralır; gerçek uygulamada dikey çizgiyi farenizle serbestçe çekebilirsiniz.
                </span>
              </div>
            </div>
          {:else if currentStep === 2}
            <!-- Step 3: Mixes & Playlist Discovery Preview -->
            <div class="mockup-step3">
              <div class="mock-mix-card active" class:playing={mockPlaying}>
                <div class="mock-mix-art">
                  <Icon name="now" size={24} />
                  <div class="mini-equalizer">
                    <span class="me-bar b1"></span>
                    <span class="me-bar b2"></span>
                    <span class="me-bar b3"></span>
                  </div>
                  <button
                    type="button"
                    class="mock-play-btn"
                    title="Oynat / Duraklat"
                    onclick={() => (mockPlaying = !mockPlaying)}
                  >
                    <Icon name={mockPlaying ? 'pause' : 'play'} size={14} />
                  </button>
                </div>
                <div class="mock-mix-meta">
                  <div class="meta-tag-row">
                    <span class="mock-tag">KİŞİSEL MİKS</span>
                    <span class="playing-live-pill">ÇALINIYOR</span>
                  </div>
                  <span class="mock-mix-t">Süpermix'im</span>
                  <span class="mock-mix-sub muted">Kapağın ortasındaki düğmeye basınca doğrudan başlar</span>
                </div>
              </div>

              <!-- Queue snippet showing inside tracks -->
              <div class="mock-queue-preview">
                <div class="queue-row active">
                  <span class="q-idx">1</span>
                  <span class="q-title truncate">ALO</span>
                  <span class="q-artist truncate muted">Arda Coşgun</span>
                  <span class="q-dur muted">03:30</span>
                </div>
                <div class="queue-row">
                  <span class="q-idx">2</span>
                  <span class="q-title truncate">Kafana Göre</span>
                  <span class="q-artist truncate muted">Duman</span>
                  <span class="q-dur muted">04:12</span>
                </div>
              </div>
            </div>
          {:else if currentStep === 3}
            <!-- Step 4: Appearance, Theme & Real-time Auto-rotating Preview -->
            <div class="mockup-step4" style:--preview-accent={currentPreviewAccent}>
              <div class="theme-live-banner">
                <span class="sim-dot live-dot" style:background-color={currentPreviewAccent}></span>
                <span class="t-banner-txt">
                  {userOverrodeAccent ? 'Seçtiğin Renk:' : 'Otomatik Renk Geçişi:'}
                  <strong style:color={currentPreviewAccent}>
                    {previewAccents.find((p) => p.value === currentPreviewAccent)?.name ?? currentPreviewAccent}
                  </strong>
                </span>
              </div>

              <div class="mini-swatches">
                {#each previewAccents as p}
                  <button
                    type="button"
                    class="mini-swatch"
                    class:active={currentPreviewAccent === p.value}
                    style:background-color={p.value}
                    title={p.name}
                    onclick={() => {
                      userOverrodeAccent = true;
                      theme.setAccent(p.value);
                    }}
                  ></button>
                {/each}
              </div>

              <!-- Mini Mockup Box with Animated Transitions -->
              <div class="mini-theme-showcase" style:border-color={currentPreviewAccent}>
                <div class="mini-showcase-sidebar" class:collapsed={previewSidebarCollapsed}>
                  <div class="m-sb-logo" style:background-color={currentPreviewAccent}>
                    <Icon name="play" size={10} />
                  </div>
                  {#if !previewSidebarCollapsed}
                    <span class="m-sb-text truncate">BetterYTMusic</span>
                  {/if}
                </div>
                <div class="mini-showcase-body">
                  <div class="mini-demo-card" style:box-shadow="0 3px 0 {currentPreviewAccent}">
                    <span class="mini-demo-btn" style:background-color={currentPreviewAccent}>
                      Dokunsal Buton
                    </span>
                    <span class="mini-demo-zoom">Zoom: %{theme.zoom}</span>
                  </div>
                </div>
              </div>

              <div class="mini-toggle-row">
                <span class="muted">Sol Menü:</span>
                <span class="badge accent-badge" style:background-color={currentPreviewAccent}>
                  {previewSidebarCollapsed ? 'İkon Modu (74px)' : 'Tam Menü (228px)'}
                </span>
                <button
                  type="button"
                  class="btn btn-sm"
                  onclick={() => {
                    previewSidebarCollapsed = !previewSidebarCollapsed;
                    theme.toggleSidebarCollapsed();
                  }}
                >
                  Değiştir
                </button>
              </div>
            </div>
          {:else if currentStep === 4}
            <!-- Step 5: Sound Output & Background System Tray Preview -->
            <div class="mockup-step5">
              <div class="device-box">
                <div class="dev-box-head">
                  <Icon name="speaker" size={20} />
                  <span class="dev-box-t">Ses Çıkış Aygıtı (Anında Geçiş)</span>
                </div>
                <div class="dev-box-list">
                  <div class="dev-row" class:active={activeDeviceIdx === 0}>
                    <div class="dev-info-col">
                      <span class="dev-name">Sistem Varsayılanı (Hoparlör)</span>
                      <span class="dev-sub muted">Realtek High Definition Audio</span>
                    </div>
                    {#if activeDeviceIdx === 0}
                      <span class="dev-active-badge">Aktif Çalıyor</span>
                      <Icon name="check" size={16} />
                    {/if}
                  </div>
                  <div class="dev-row" class:active={activeDeviceIdx === 1}>
                    <div class="dev-info-col">
                      <span class="dev-name">Bluetooth Kulaklık</span>
                      <span class="dev-sub muted">Sony WH-1000XM4 (LDAC)</span>
                    </div>
                    {#if activeDeviceIdx === 1}
                      <span class="dev-active-badge">Aktif Çalıyor</span>
                      <Icon name="check" size={16} />
                    {/if}
                  </div>
                </div>
              </div>

              <!-- Animated Windows 11 Taskbar Tray Mockup -->
              <div class="windows-tray-mockup">
                <div class="taskbar-left">
                  <div class="win-start-btn"><Icon name="windows" size={13} /></div>
                  <div class="taskbar-app-icon active">
                    <Icon name="play" size={14} />
                  </div>
                </div>
                <div class="taskbar-tray">
                  <div class="tray-app-item">
                    <span class="tray-icon-red"><Icon name="play" size={10} /></span>
                    <div class="tray-floating-tooltip">
                      <span class="tooltip-note"><Icon name="now" size={12} /></span>
                      <span>Arka planda çalıyor (Pencereyi kapatsan da kesintisiz)</span>
                    </div>
                  </div>
                  <span class="tray-clock">14:55</span>
                </div>
              </div>
            </div>
          {:else if currentStep === 5}
            <!-- Step 6: Auth Showcase -->
            <div class="mockup-step6">
              <div class="auth-hero-circle">
                <Icon name="user" size={44} />
                <div class="auth-glow-ring"></div>
              </div>
              <h3 class="auth-h3">Kişisel Kitaplığın & Süpermix'in</h3>
              <p class="auth-p muted">
                Google hesabınla bağlanarak tüm çalma listelerine, beğendiğin şarkılara ve özel sanatçı mikslerine tek tıkla ulaş.
              </p>
              <div class="auth-perk-grid">
                <div class="perk-card float-1">
                  <span class="perk-icon"><Icon name="now" size={18} /></span>
                  <div class="perk-meta">
                    <strong>Tüm Listelerin</strong>
                    <span class="muted">70+ parçalık tam listeler</span>
                  </div>
                </div>
                <div class="perk-card float-2">
                  <span class="perk-icon"><Icon name="star" size={18} /></span>
                  <div class="perk-meta">
                    <strong>Beğenilen Şarkılar</strong>
                    <span class="muted">Otomatik senkronize</span>
                  </div>
                </div>
                <div class="perk-card float-3">
                  <span class="perk-icon"><Icon name="radio" size={18} /></span>
                  <div class="perk-meta">
                    <strong>Sonsuz Radyo</strong>
                    <span class="muted">Tarzına uygun öneriler</span>
                  </div>
                </div>
              </div>
            </div>
          {/if}
        </div>
      </aside>
    </div>
  </main>

  <!-- Bottom Navigation Bar with Dots & Buttons -->
  <footer class="guide-footer">
    <div class="footer-container">
      <!-- Dots Indicator -->
      <div class="dots-nav" role="tablist" aria-label="Adım seçici">
        {#each steps as s, idx}
          <button
            type="button"
            class="dot-pill"
            class:active={currentStep === idx}
            title={s.title}
            aria-label={`Adım ${idx + 1}: ${s.title}`}
            onclick={() => (currentStep = idx)}
          >
            <span class="dot-idx">{idx + 1}</span>
            <span class="dot-label">{s.badge.split('/')[1]?.trim() || s.title}</span>
          </button>
        {/each}
      </div>

      <!-- Action Buttons -->
      <div class="footer-actions">
        {#if currentStep > 0}
          <button type="button" class="btn nav-action-btn" onclick={prev}>
            <Icon name="prev" size={16} />
            Geri
          </button>
        {/if}

        {#if !isLast}
          <button type="button" class="btn accent nav-action-btn" onclick={next}>
            Sonraki Adım
            <Icon name="next" size={16} />
          </button>
        {:else}
          <button type="button" class="btn accent nav-action-btn" onclick={player.closeOnboarding}>
            <Icon name="check" size={16} />
            Uygulamaya Başla
          </button>
        {/if}
      </div>
    </div>
  </footer>
</div>

<style>
  .fullscreen-guide {
    position: fixed;
    inset: 0;
    z-index: 1000;
    width: 100vw;
    height: 100vh;
    background: var(--surface);
    color: var(--text);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: fadeIn 150ms ease;
  }

  /* Header */
  .guide-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 28px;
    border-bottom: 2px solid var(--border);
    background: var(--surface-2);
    flex-shrink: 0;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .logo {
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    border-radius: var(--radius-sm);
    background: var(--accent);
    color: var(--on-accent);
    box-shadow: 0 var(--edge-depth) 0 var(--accent-edge);
    flex-shrink: 0;
  }

  .brand-text {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .brand-name {
    font-weight: 800;
    font-size: 16px;
    letter-spacing: -0.01em;
  }

  .brand-tag {
    font-size: 12px;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .skip-btn {
    font-size: 13px;
    font-weight: 700;
    padding: 7px 14px;
    background: var(--surface);
    border: 2px solid var(--border);
  }

  .close-icon-btn {
    width: 38px;
    height: 38px;
    min-height: 38px;
    padding: 0;
  }

  /* Main Stage Container */
  .guide-stage {
    flex: 1;
    overflow-y: auto;
    padding: 32px 28px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .stage-container {
    width: 100%;
    max-width: 1180px;
    display: grid;
    grid-template-columns: minmax(0, 1.15fr) minmax(0, 1fr);
    gap: 40px;
    align-items: center;
  }

  /* Stage Info (Left Column) */
  .stage-info {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .badge-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .accent-badge {
    background: var(--accent);
    color: var(--on-accent);
    font-weight: 800;
    font-size: 11px;
    padding: 4px 10px;
    border-radius: var(--radius-sm);
    letter-spacing: 0.05em;
  }

  .step-num {
    font-size: 13px;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
  }

  .stage-title {
    font-size: 32px;
    font-weight: 800;
    line-height: 1.2;
    letter-spacing: -0.02em;
  }

  .stage-subtitle {
    font-size: 15.5px;
    line-height: 1.5;
  }

  .cards-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-top: 6px;
  }

  .info-card {
    display: flex;
    align-items: flex-start;
    gap: 16px;
    padding: 16px 18px;
    background: var(--surface-2);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .card-icon {
    width: 42px;
    height: 42px;
    border-radius: var(--radius-sm);
    background: var(--surface-3);
    color: var(--accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }

  .card-content {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .card-title {
    font-size: 15px;
    font-weight: 700;
  }

  .card-desc {
    font-size: 13px;
    line-height: 1.45;
  }

  .auth-action-block {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-top: 8px;
  }

  .main-auth-btn {
    min-height: 46px;
    font-size: 14.5px;
    font-weight: 800;
  }

  .guest-auth-btn {
    min-height: 40px;
    font-size: 13.5px;
    font-weight: 700;
  }

  /* Stage Visual Spotlight (Right Column) */
  .stage-visual {
    display: flex;
    justify-content: center;
    width: 100%;
  }

  .visual-panel {
    width: 100%;
    min-height: 380px;
    padding: 28px;
    background: var(--surface-2);
    border: 2px solid var(--border);
    border-radius: var(--radius-lg);
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.35), 0 var(--edge-depth) 0 var(--edge);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
  }

  /* Mockups for each step with Micro-animations */
  .mockup-step1 {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    width: 100%;
  }

  .hero-logo-wrap {
    position: relative;
    display: grid;
    place-items: center;
  }

  .hero-logo-large {
    width: 82px;
    height: 82px;
    border-radius: 22px;
    background: var(--accent);
    color: var(--on-accent);
    display: grid;
    place-items: center;
    box-shadow: 0 6px 0 var(--accent-edge);
    position: relative;
    z-index: 2;
  }

  .pulse-ring {
    position: absolute;
    inset: -8px;
    border-radius: 30px;
    border: 2px solid var(--accent);
    opacity: 0.5;
    animation: ringPulse 2.4s infinite ease-out;
  }

  @keyframes ringPulse {
    0% {
      transform: scale(0.92);
      opacity: 0.8;
    }
    100% {
      transform: scale(1.18);
      opacity: 0;
    }
  }

  /* Live Spectrum Visualizer */
  .live-spectrum-card {
    width: 100%;
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    padding: 10px 14px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .spectrum-head {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .live-ping {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #06d6a0;
    box-shadow: 0 0 0 3px rgba(6, 214, 160, 0.25);
    animation: pingBlink 1.4s infinite ease-in-out;
  }

  @keyframes pingBlink {
    0%, 100% {
      transform: scale(0.9);
      opacity: 0.7;
    }
    50% {
      transform: scale(1.2);
      opacity: 1;
    }
  }

  .spec-title {
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .spec-codec {
    margin-left: auto;
    font-size: 10px;
    font-weight: 700;
    color: var(--accent);
  }

  .eq-bars-row {
    display: flex;
    align-items: flex-end;
    gap: 4px;
    height: 44px;
    padding-top: 4px;
  }

  .eq-bar {
    flex: 1;
    background: var(--accent);
    border-radius: 2px;
    animation: eqBounce 1.2s infinite ease-in-out;
  }

  @keyframes eqBounce {
    0%, 100% {
      height: 20%;
      opacity: 0.55;
    }
    50% {
      height: var(--base-h, 90%);
      opacity: 1;
    }
  }

  .stat-pills {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
    width: 100%;
  }

  .stat-pill {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 12px 10px;
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius);
  }

  .sp-val {
    font-weight: 800;
    font-size: 14.5px;
  }

  .sp-lbl {
    font-size: 11px;
  }

  .accent-txt {
    color: var(--accent);
  }

  .green-txt {
    color: #06d6a0;
  }

  .mini-note {
    font-size: 12px;
  }

  /* Mockup Step 2: Auto-resizing split & live lyrics */
  .mockup-step2 {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .mock-panel-wrap.auto-resizing {
    display: grid;
    grid-template-columns: 1fr 12px 1.35fr;
    gap: 8px;
    align-items: center;
    background: var(--surface);
    padding: 16px;
    border-radius: var(--radius);
    border: 2px solid var(--border);
    animation: autoSplitAnim 6s infinite ease-in-out;
  }

  @keyframes autoSplitAnim {
    0%, 100% {
      grid-template-columns: 1fr 12px 1.4fr;
    }
    50% {
      grid-template-columns: 1.35fr 12px 1fr;
    }
  }

  .mock-col {
    display: flex;
    flex-direction: column;
    gap: 6px;
    min-width: 0;
  }

  .mock-cover-box {
    width: 100%;
    aspect-ratio: 1;
    background: var(--surface-3);
    border-radius: var(--radius-sm);
    display: grid;
    place-items: center;
    color: var(--accent);
    position: relative;
    overflow: hidden;
  }

  .vinyl-groove {
    position: absolute;
    inset: 4px;
    border: 1px dashed rgba(255, 255, 255, 0.15);
    border-radius: 50%;
  }

  .mock-song-t {
    font-weight: 800;
    font-size: 13px;
  }

  .mock-song-a {
    font-size: 11px;
  }

  .mock-divider {
    position: relative;
    width: 12px;
    height: 100%;
    min-height: 140px;
    background: var(--surface-3);
    border-radius: 6px;
    display: grid;
    place-items: center;
  }

  .divider-grip {
    width: 4px;
    height: 34px;
    background: var(--accent);
    border-radius: 2px;
  }

  .drag-ghost-tag {
    position: absolute;
    top: -14px;
    left: 50%;
    transform: translateX(-50%);
    width: 22px;
    height: 22px;
    border-radius: 50%;
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--accent);
    display: grid;
    place-items: center;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
    animation: dragGripSway 3s infinite ease-in-out;
  }

  @keyframes dragGripSway {
    0%, 100% {
      transform: translateX(-80%);
    }
    50% {
      transform: translateX(-20%);
    }
  }

  .lrc-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 6px;
  }

  .lrc-source {
    font-size: 10px;
    font-weight: 700;
    color: var(--accent);
  }

  .lrc-scroll-box {
    display: flex;
    flex-direction: column;
    gap: 4px;
    text-align: left;
  }

  .lrc-line {
    padding: 6px 8px;
    border-radius: 6px;
    font-size: 12px;
    color: var(--text-muted);
    transition: all 0.25s ease;
  }

  .lrc-line.active {
    background: color-mix(in srgb, var(--accent) 18%, var(--surface-2));
    color: var(--accent);
    font-weight: 800;
    font-size: 12.5px;
    border-left: 2.5px solid var(--accent);
    transform: translateX(4px);
  }

  .lrc-play-arrow {
    display: inline-block;
    margin-right: 4px;
    font-size: 10px;
  }

  .mock-hint-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 10px;
    background: var(--surface);
    border-radius: 6px;
    border: 1px solid var(--border);
    text-align: left;
  }

  .hint-pulse {
    font-size: 13px;
  }

  .mock-hint {
    font-size: 11.5px;
    line-height: 1.35;
  }

  /* Mockup Step 3: Playlist & Live playing indicator */
  .mockup-step3 {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .mock-mix-card {
    display: grid;
    grid-template-columns: 64px 1fr;
    gap: 12px;
    align-items: center;
    padding: 12px;
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    text-align: left;
  }

  .mock-mix-card.active {
    border-color: var(--accent);
  }

  .mock-mix-art {
    position: relative;
    width: 64px;
    height: 64px;
    background: var(--accent);
    border-radius: var(--radius-sm);
    display: grid;
    place-items: center;
    color: #fff;
    overflow: hidden;
  }

  .mini-equalizer {
    position: absolute;
    bottom: 4px;
    left: 4px;
    display: flex;
    align-items: flex-end;
    gap: 2px;
    height: 12px;
  }

  .me-bar {
    width: 2.5px;
    background: #fff;
    border-radius: 1px;
    animation: meBlink 0.9s infinite ease-in-out;
  }
  .me-bar.b1 {
    height: 6px;
    animation-delay: 0.1s;
  }
  .me-bar.b2 {
    height: 11px;
    animation-delay: 0.3s;
  }
  .me-bar.b3 {
    height: 8px;
    animation-delay: 0.2s;
  }

  @keyframes meBlink {
    0%, 100% {
      height: 3px;
    }
    50% {
      height: 12px;
    }
  }

  .mock-play-btn {
    position: absolute;
    inset: 0;
    margin: auto;
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background: #fff;
    color: #111;
    display: grid;
    place-items: center;
    border: none;
    cursor: pointer;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.4);
    transition: transform 0.1s ease;
  }

  .mock-play-btn:active {
    transform: scale(0.9);
  }

  .mock-mix-meta {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .meta-tag-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .mock-tag {
    font-size: 10px;
    font-weight: 800;
    color: var(--text-muted);
  }

  .playing-live-pill {
    font-size: 9px;
    font-weight: 800;
    background: #06d6a0;
    color: #000;
    padding: 1px 6px;
    border-radius: 4px;
  }

  .mock-mix-t {
    font-weight: 800;
    font-size: 14px;
  }

  .mock-mix-sub {
    font-size: 11.5px;
  }

  .mock-queue-preview {
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    padding: 8px 10px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    text-align: left;
  }

  .queue-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 8px;
    border-radius: 6px;
    font-size: 12px;
  }

  .queue-row.active {
    background: var(--surface-2);
    font-weight: 800;
    color: var(--accent);
  }

  .q-idx {
    font-size: 11px;
    font-weight: 800;
    width: 14px;
  }

  .q-title {
    flex: 1;
    min-width: 0;
  }

  .q-artist {
    font-size: 11px;
  }

  .q-dur {
    font-size: 11px;
    font-variant-numeric: tabular-nums;
  }

  /* Mockup Step 4: Theme live auto-rotating preview */
  .mockup-step4 {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 14px;
    text-align: left;
  }

  .theme-live-banner {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 7px 12px;
    background: var(--surface);
    border: 1.5px solid var(--border);
    border-radius: 6px;
    font-size: 12px;
  }

  .t-banner-txt strong {
    margin-left: 4px;
  }

  .mini-swatches {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .mini-swatch {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    border: 2px solid var(--border);
    cursor: pointer;
    box-shadow: 0 2px 0 var(--edge);
    transition: transform 0.15s ease, border-color 0.15s ease;
  }

  .mini-swatch.active {
    transform: scale(1.18);
    border-color: #fff;
    outline: 2px solid var(--text);
    outline-offset: 1px;
  }

  .mini-theme-showcase {
    display: flex;
    border: 2px solid var(--preview-accent, var(--accent));
    border-radius: var(--radius);
    background: var(--surface);
    height: 90px;
    overflow: hidden;
    transition: border-color 0.3s ease;
  }

  .mini-showcase-sidebar {
    width: 90px;
    background: var(--surface-2);
    border-right: 1.5px solid var(--border);
    padding: 8px 6px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    transition: width 0.25s ease;
  }

  .mini-showcase-sidebar.collapsed {
    width: 36px;
    align-items: center;
  }

  .m-sb-logo {
    width: 18px;
    height: 18px;
    border-radius: 4px;
    display: grid;
    place-items: center;
    color: #fff;
    flex-shrink: 0;
    transition: background-color 0.3s ease;
  }

  .m-sb-text {
    font-size: 10px;
    font-weight: 800;
  }

  .mini-showcase-body {
    flex: 1;
    padding: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .mini-demo-card {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    border-radius: 6px;
  }

  .mini-demo-btn {
    padding: 4px 8px;
    border-radius: 4px;
    color: #fff;
    font-size: 11px;
    font-weight: 800;
    transition: background-color 0.3s ease;
  }

  .mini-demo-zoom {
    font-size: 11px;
    font-weight: 700;
    color: var(--text-muted);
  }

  .mini-toggle-row {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 12.5px;
  }

  .btn-sm {
    min-height: 28px;
    padding: 0 10px;
    font-size: 11.5px;
    font-weight: 700;
  }

  /* Mockup Step 5: Sound Output & Windows 11 Taskbar Tray */
  .mockup-step5 {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .device-box {
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    padding: 12px 14px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    text-align: left;
  }

  .dev-box-head {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 800;
    font-size: 13px;
  }

  .dev-box-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .dev-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    font-size: 12px;
    background: var(--surface-2);
    border: 1.5px solid transparent;
    transition: all 0.2s ease;
  }

  .dev-row.active {
    background: color-mix(in srgb, var(--accent) 15%, var(--surface-2));
    border-color: var(--accent);
    color: var(--text);
  }

  .dev-info-col {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .dev-name {
    font-weight: 800;
  }

  .dev-sub {
    font-size: 10.5px;
  }

  .dev-active-badge {
    font-size: 9.5px;
    font-weight: 800;
    background: #06d6a0;
    color: #000;
    padding: 1px 6px;
    border-radius: 4px;
    margin-right: 6px;
  }

  .windows-tray-mockup {
    height: 44px;
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
  }

  .taskbar-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .win-start-btn {
    width: 22px;
    height: 22px;
    border-radius: 4px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    color: #00a4ef;
    display: grid;
    place-items: center;
  }

  .taskbar-app-icon {
    width: 24px;
    height: 24px;
    border-radius: 5px;
    background: var(--accent);
    color: #fff;
    display: grid;
    place-items: center;
  }

  .taskbar-tray {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .tray-app-item {
    position: relative;
    display: flex;
    align-items: center;
  }

  .tray-icon-red {
    width: 18px;
    height: 18px;
    border-radius: 4px;
    background: #ff4e45;
    color: #fff;
    display: grid;
    place-items: center;
  }

  .tray-floating-tooltip {
    position: absolute;
    bottom: calc(100% + 8px);
    right: 0;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 9px;
    border-radius: 6px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    font-size: 11px;
    font-weight: 700;
    white-space: nowrap;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.35);
    animation: floatTooltip 2.4s infinite ease-in-out;
  }

  @keyframes floatTooltip {
    0%, 100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-4px);
    }
  }

  /* Mockup Step 6: Auth Showcase with Floating 3D Cards */
  .mockup-step6 {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 14px;
  }

  .auth-hero-circle {
    position: relative;
    width: 76px;
    height: 76px;
    border-radius: 50%;
    background: var(--surface);
    border: 3px solid var(--accent);
    color: var(--accent);
    display: grid;
    place-items: center;
  }

  .auth-glow-ring {
    position: absolute;
    inset: -6px;
    border-radius: 50%;
    border: 1.5px dashed var(--accent);
    opacity: 0.6;
    animation: spinRing 12s linear infinite;
  }

  @keyframes spinRing {
    to {
      transform: rotate(360deg);
    }
  }

  .auth-h3 {
    font-size: 17.5px;
    font-weight: 800;
  }

  .auth-p {
    font-size: 12.5px;
    line-height: 1.45;
    max-width: 400px;
  }

  .auth-perk-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 10px;
    width: 100%;
  }

  .perk-card {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
    padding: 10px;
    border-radius: var(--radius);
    background: var(--surface);
    border: 2px solid var(--border);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    text-align: left;
  }

  .perk-card.float-1 {
    animation: floatPerk 3.2s infinite ease-in-out 0s;
  }
  .perk-card.float-2 {
    animation: floatPerk 3.2s infinite ease-in-out 0.4s;
  }
  .perk-card.float-3 {
    animation: floatPerk 3.2s infinite ease-in-out 0.8s;
  }

  @keyframes floatPerk {
    0%, 100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-5px);
    }
  }

  .perk-icon {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    color: var(--accent);
    display: grid;
    place-items: center;
  }

  .perk-meta {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .perk-meta strong {
    font-size: 12px;
    font-weight: 800;
  }

  .perk-meta span {
    font-size: 10px;
  }

  /* Footer */
  .guide-footer {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px 28px;
    border-top: 2px solid var(--border);
    background: var(--surface-2);
    flex-shrink: 0;
  }

  .footer-container {
    width: 100%;
    max-width: 1180px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
  }

  .dots-nav {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }

  .dot-pill {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: var(--radius-sm);
    border: 2px solid var(--border);
    background: var(--surface);
    color: var(--text-muted);
    cursor: pointer;
    font: inherit;
    font-size: 12px;
    font-weight: 700;
    transition: background-color 120ms ease, border-color 120ms ease, color 120ms ease;
  }

  .dot-pill.active {
    background: var(--accent);
    border-color: var(--accent-edge);
    color: var(--on-accent);
    box-shadow: 0 var(--edge-depth) 0 var(--accent-edge);
  }

  .dot-idx {
    font-weight: 800;
  }

  .footer-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .nav-action-btn {
    min-height: 42px;
    font-size: 14px;
    font-weight: 800;
    padding: 0 18px;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  /* Responsive styling */
  @media (max-width: 960px) {
    .stage-container {
      grid-template-columns: 1fr;
      gap: 24px;
    }
    .stage-visual {
      display: none;
    }
    .stage-title {
      font-size: 26px;
    }
    .dot-label {
      display: none;
    }
    .guide-stage {
      padding: 20px 16px;
    }
    .guide-header,
    .guide-footer {
      padding: 12px 16px;
    }
  }
</style>
