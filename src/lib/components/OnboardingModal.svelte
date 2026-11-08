<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { theme, ACCENT_PRESETS } from '#lib/theme.svelte.ts';
  import Icon, { type IconName } from './Icon.svelte';

  let currentStep = $state(0);

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
              <div class="hero-logo-large">
                <Icon name="play" size={54} />
              </div>
              <div class="stat-pills">
                <div class="stat-pill">
                  <span class="sp-val accent-txt">0 ms</span>
                  <span class="sp-lbl muted">Yerel Ses Gecikmesi</span>
                </div>
                <div class="stat-pill">
                  <span class="sp-val green-txt">~60 MB</span>
                  <span class="sp-lbl muted">Hafif RAM Kullanımı</span>
                </div>
                <div class="stat-pill">
                  <span class="sp-val">Tauri 2 + Rust</span>
                  <span class="sp-lbl muted">Güçlü Yerel Motor</span>
                </div>
              </div>
              <div class="mini-note muted">
                Windows 10/11 için optimize edilmiş bağımsız masaüstü oynatıcı.
              </div>
            </div>
          {:else if currentStep === 1}
            <!-- Step 2: Now Playing & Lyrics Split Preview -->
            <div class="mockup-step2">
              <div class="mock-panel-wrap">
                <div class="mock-col mock-cover-col">
                  <div class="mock-cover-box">
                    <Icon name="now" size={32} />
                  </div>
                  <span class="mock-song-t truncate">Örnek Parça</span>
                  <span class="mock-song-a truncate muted">Sanatçı Adı</span>
                </div>
                <div class="mock-divider" title="Fareyle sürüklenebilir sınır">
                  <span class="divider-grip"></span>
                </div>
                <div class="mock-col mock-lyrics-col">
                  <span class="mock-badge">Canlı Sözler</span>
                  <p class="lrc-line muted">Bu bir önceki dize...</p>
                  <p class="lrc-line active">▶ Müziğe göre eşzamanlı söz akışı</p>
                  <p class="lrc-line muted">Tıklayarak doğrudan bu saniyeye atla</p>
                </div>
              </div>
              <span class="mock-hint muted">↔️ Sütun genişliklerini farenizle dilediğiniz gibi sürükleyebilirsiniz.</span>
            </div>
          {:else if currentStep === 2}
            <!-- Step 3: Mixes & Playlist Discovery Preview -->
            <div class="mockup-step3">
              <div class="mock-mix-card active">
                <div class="mock-mix-art">
                  <Icon name="now" size={24} />
                  <div class="mock-play-btn">
                    <Icon name="play" size={14} />
                  </div>
                </div>
                <div class="mock-mix-meta">
                  <span class="mock-tag">KİŞİSEL MİKS</span>
                  <span class="mock-mix-t">Süpermix'im</span>
                  <span class="mock-mix-sub muted">Kapak üzerindeki oynat düğmesiyle hemen başlat</span>
                </div>
              </div>
              <div class="mock-mix-card">
                <div class="mock-mix-art purple-art">
                  <Icon name="now" size={24} />
                </div>
                <div class="mock-mix-meta">
                  <span class="mock-tag">ÇALMA LİSTESİ</span>
                  <span class="mock-mix-t">Favori Şarkılarım</span>
                  <span class="mock-mix-sub muted">Karta tıklayıp içindeki tüm parçaları gör</span>
                </div>
              </div>
            </div>
          {:else if currentStep === 3}
            <!-- Step 4: Appearance, Theme & Zoom Preview -->
            <div class="mockup-step4">
              <span class="vis-label muted">Arayüz & Vurgu Rengi Özelleştirmesi</span>
              <div class="mini-swatches">
                {#each ACCENT_PRESETS.slice(0, 6) as p}
                  <button
                    type="button"
                    class="mini-swatch"
                    class:active={theme.accent === p.value}
                    style:background-color={p.value}
                    title={p.name}
                    onclick={() => theme.setAccent(p.value)}
                  ></button>
                {/each}
              </div>
              <div class="mini-toggle-row">
                <span class="muted">Görünürlük (Zoom):</span>
                <span class="zoom-badge">%{theme.zoom}</span>
                <button type="button" class="btn btn-sm" onclick={() => theme.stepZoom(-5)}>−</button>
                <button type="button" class="btn btn-sm" onclick={() => theme.stepZoom(5)}>+</button>
              </div>
              <div class="mini-toggle-row">
                <span class="muted">Sol Menü Modu:</span>
                <button type="button" class="btn btn-sm" onclick={() => theme.toggleSidebarCollapsed()}>
                  {theme.sidebarCollapsed ? 'İkon Modu (Aktif)' : 'Tam Menü (Aktif)'}
                </button>
              </div>
            </div>
          {:else if currentStep === 4}
            <!-- Step 5: Sound Output & Background Controls -->
            <div class="mockup-step5">
              <div class="device-box">
                <div class="dev-box-head">
                  <Icon name="speaker" size={20} />
                  <span class="dev-box-t">Ses Çıkış Cihazı Seçimi</span>
                </div>
                <div class="dev-box-list">
                  <div class="dev-row active">
                    <span>Sistem Varsayılanı (Hoparlör)</span>
                    <Icon name="check" size={16} />
                  </div>
                  <div class="dev-row">
                    <span>Bluetooth Kulaklık</span>
                  </div>
                </div>
              </div>
              <div class="tray-preview-box">
                <span class="mock-badge">Pencereyi Kapatınca</span>
                <p class="muted">Müzik sistem tepsisinde (tray) kesintisiz çalmaya devam eder.</p>
              </div>
            </div>
          {:else if currentStep === 5}
            <!-- Step 6: Auth Showcase -->
            <div class="mockup-step6">
              <div class="auth-hero-circle">
                <Icon name="user" size={44} />
              </div>
              <h3 class="auth-h3">Tam YouTube Music Deneyimi</h3>
              <p class="auth-p muted">
                Giriş yaparak özel çalma listelerine, beğendiğin parçalara ve kitaplığına her cihazdan kesintisiz eriş.
              </p>
              <div class="auth-perk-row">
                <span class="perk-pill">🎵 70+ Parçalık Çalma Listeleri</span>
                <span class="perk-pill">⭐ Beğenilen Şarkılar</span>
                <span class="perk-pill">📻 Sana Özel Radyolar</span>
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

  /* Mockups for each step */
  .mockup-step1 {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
    width: 100%;
  }

  .hero-logo-large {
    width: 88px;
    height: 88px;
    border-radius: 24px;
    background: var(--accent);
    color: var(--on-accent);
    display: grid;
    place-items: center;
    box-shadow: 0 6px 0 var(--accent-edge);
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
    font-size: 15px;
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

  /* Mockup Step 2 */
  .mockup-step2 {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .mock-panel-wrap {
    display: grid;
    grid-template-columns: 1fr 10px 1.4fr;
    gap: 8px;
    align-items: center;
    background: var(--surface);
    padding: 16px;
    border-radius: var(--radius);
    border: 2px solid var(--border);
  }

  .mock-col {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .mock-cover-box {
    width: 100%;
    aspect-ratio: 1;
    background: var(--surface-3);
    border-radius: var(--radius-sm);
    display: grid;
    place-items: center;
    color: var(--accent);
  }

  .mock-song-t {
    font-weight: 700;
    font-size: 13px;
  }

  .mock-song-a {
    font-size: 11px;
  }

  .mock-divider {
    width: 10px;
    height: 100%;
    background: var(--accent-soft);
    border-radius: 5px;
    display: grid;
    place-items: center;
  }

  .divider-grip {
    width: 4px;
    height: 32px;
    background: var(--accent);
    border-radius: 2px;
  }

  .mock-badge {
    align-self: flex-start;
    padding: 2px 7px;
    border-radius: 5px;
    background: var(--surface-3);
    font-size: 10.5px;
    font-weight: 800;
  }

  .lrc-line {
    font-size: 12px;
    margin: 2px 0;
    text-align: left;
  }

  .lrc-line.active {
    color: var(--accent);
    font-weight: 800;
    font-size: 13px;
  }

  .mock-hint {
    font-size: 12px;
  }

  /* Mockup Step 3 */
  .mockup-step3 {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 12px;
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
    background: var(--accent-soft);
  }

  .mock-mix-art {
    position: relative;
    width: 64px;
    height: 64px;
    background: #ff4e45;
    border-radius: var(--radius-sm);
    display: grid;
    place-items: center;
    color: #fff;
  }

  .purple-art {
    background: #8338ec;
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
  }

  .mock-mix-meta {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .mock-tag {
    font-size: 10px;
    font-weight: 800;
    color: var(--text-muted);
  }

  .mock-mix-t {
    font-weight: 800;
    font-size: 14px;
  }

  .mock-mix-sub {
    font-size: 11.5px;
  }

  /* Mockup Step 4 */
  .mockup-step4 {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 16px;
    text-align: left;
  }

  .vis-label {
    font-size: 12px;
    font-weight: 800;
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
    box-shadow: 0 min(2px, var(--edge-depth)) 0 var(--edge);
  }

  .mini-swatch.active {
    transform: scale(1.15);
    border-color: #fff;
  }

  .mini-toggle-row {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 13px;
  }

  .zoom-badge {
    padding: 3px 8px;
    border-radius: 6px;
    background: var(--accent);
    color: var(--on-accent);
    font-weight: 800;
  }

  .btn-sm {
    min-height: 30px;
    padding: 0 10px;
    font-size: 12px;
  }

  /* Mockup Step 5 */
  .mockup-step5 {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .device-box {
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    text-align: left;
  }

  .dev-box-head {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 800;
    font-size: 13.5px;
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
    font-size: 12.5px;
    font-weight: 600;
    background: var(--surface-2);
  }

  .dev-row.active {
    background: var(--accent-soft);
    border: 1px solid var(--accent);
    color: var(--accent);
    font-weight: 800;
  }

  .tray-preview-box {
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    text-align: left;
    font-size: 12px;
  }

  /* Mockup Step 6 */
  .mockup-step6 {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 14px;
  }

  .auth-hero-circle {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: var(--accent-soft);
    border: 3px solid var(--accent);
    color: var(--accent);
    display: grid;
    place-items: center;
  }

  .auth-h3 {
    font-size: 18px;
    font-weight: 800;
  }

  .auth-p {
    font-size: 13px;
    line-height: 1.45;
  }

  .auth-perk-row {
    display: flex;
    flex-direction: column;
    gap: 6px;
    width: 100%;
  }

  .perk-pill {
    padding: 8px 12px;
    border-radius: var(--radius-sm);
    background: var(--surface);
    border: 1px solid var(--border);
    font-size: 12px;
    font-weight: 700;
    text-align: left;
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
