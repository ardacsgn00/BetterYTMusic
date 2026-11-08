<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import Icon, { type IconName } from './Icon.svelte';

  let currentStep = $state(0);

  interface StepInfo {
    badge: string;
    icon: IconName;
    title: string;
    subtitle: string;
    points: { icon: IconName; label: string; desc: string }[];
  }

  const steps: StepInfo[] = [
    {
      badge: 'Başlangıç',
      icon: 'now',
      title: "BetterYTMusic'e Hoş Geldin!",
      subtitle: 'Hızlı, hafif ve tam donanımlı masaüstü YouTube Music istemcisi.',
      points: [
        {
          icon: 'play',
          label: 'Ultra Hafif & Hızlı',
          desc: 'Tauri 2 ve Rust yerel ses motoru sayesinde tarayıcı ağırlığı olmadan minimum bellek ve CPU tüketir.'
        },
        {
          icon: 'check',
          label: 'Sade & Odaklı',
          desc: 'Gereksiz reklam izleyicilerinden ve tarayıcı sekmelerinden bağımsız, kesintisiz müzik deneyimi.'
        }
      ]
    },
    {
      badge: 'Oynatıcı',
      icon: 'lyrics',
      title: 'Dinle & Canlı Şarkı Sözleri',
      subtitle: 'Müziğe göre gerçek zamanlı kayan sözler ve esnek panel düzeni.',
      points: [
        {
          icon: 'lyrics',
          label: 'Senkronize Sözler (LRC)',
          desc: 'Şarkı çalarken sözler satır satır ilerler; dilediğin satıra tıklayarak o saniyeye doğrudan atlayabilirsin.'
        },
        {
          icon: 'columns',
          label: 'Fareyle Sürüklenebilir Paneller',
          desc: 'Kapak, sözler ve sıradaki liste arasındaki dikey çizgilerden tutarak genişlikleri serbestçe ayarla.'
        }
      ]
    },
    {
      badge: 'Keşif',
      icon: 'home',
      title: 'Süpermix & Çalma Listeleri',
      subtitle: 'Sana özel miksler, keşif listeleri ve detaylı parça görünümleri.',
      points: [
        {
          icon: 'play',
          label: 'Kapak Üzerinden Tek Tıkla Başlat',
          desc: 'Kapağın ortasındaki oynat simgesine tıklayarak çalma listesini doğrudan çalmaya başlayabilirsin.'
        },
        {
          icon: 'now',
          label: 'Liste İçeriğini İncele',
          desc: 'Kartın gövdesine tıklandığında liste içindeki tüm şarkılar sıralanır ve istediğin parçayı seçebilirsin.'
        }
      ]
    },
    {
      badge: 'Tasarım',
      icon: 'palette',
      title: 'Kişisel Görünüm & Özelleştirme',
      subtitle: 'Temalar, renkler, yazı tipleri ve pencere boyutu tamamen senin kontrolünde.',
      points: [
        {
          icon: 'palette',
          label: 'Vurgu Renkleri & 3B Derinlik',
          desc: '10 canlı renk paleti, OLED/Koyu/Slate temaları ve dokunsal alt kenar kalınlıkları.'
        },
        {
          icon: 'search',
          label: 'Tek Düğmeli Zoom & Sol Menü Boyutlandırma',
          desc: 'Ana sayfadaki % zoom menüsüyle ölçeği ayarla; sol menüyü fareyle daraltarak ikon moduna geçir.'
        }
      ]
    },
    {
      badge: 'Sistem',
      icon: 'speaker',
      title: 'Ses Çıkışı & Arka Planda Çalma',
      subtitle: 'Müziğini hiçbir zaman yarıda kesmek zorunda kalma.',
      points: [
        {
          icon: 'speaker',
          label: 'Hızlı Çıkış Cihazı Seçimi',
          desc: 'Alt bardaki hoparlör simgesinden kulaklık veya hoparlörünü şarkı duraksamadan anında değiştir.'
        },
        {
          icon: 'settings',
          label: 'Sistem Tepsisi & Görev Çubuğu',
          desc: 'Pencere kapansa da müzik arka planda sürer; klavye medya tuşları ve küçük resim düğmeleriyle yönet.'
        }
      ]
    },
    {
      badge: 'Hesap',
      icon: 'user',
      title: 'Google Hesabını Bağla',
      subtitle: 'Kitaplığına, kişisel çalma listelerine ve beğenilen parçalarına eriş.',
      points: [
        {
          icon: 'user',
          label: 'Kitaplık & Beğenilenler Senkronizasyonu',
          desc: 'Hesabınla bağlanarak kendi listelerine ve sana özel YouTube Music önerilerine eksiksiz ulaş.'
        },
        {
          icon: 'check',
          label: 'Misafir Olarak da Kullanabilirsin',
          desc: 'Dilersen hesap bağlamadan misafir olarak hemen müzik dinlemeye ve arama yapmaya başlayabilirsin.'
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

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="backdrop" onclick={player.closeOnboarding}>
  <div class="card modal" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="-1">
    <header class="modal-head">
      <div class="head-left">
        <span class="badge accent-badge">{step.badge}</span>
        <span class="step-indicator muted">Adım {currentStep + 1} / {totalSteps}</span>
      </div>
      <button
        class="btn icon close-btn"
        title="Rehberi Kapat"
        aria-label="Rehberi Kapat"
        onclick={player.closeOnboarding}
      >
        <Icon name="close" size={18} />
      </button>
    </header>

    <div class="modal-body">
      <div class="icon-hero">
        <div class="hero-bubble">
          <Icon name={step.icon} size={36} />
        </div>
      </div>

      <div class="text-block">
        <h2 class="title">{step.title}</h2>
        <p class="subtitle muted">{step.subtitle}</p>
      </div>

      <div class="points-grid">
        {#each step.points as point}
          <div class="point-card">
            <div class="point-icon">
              <Icon name={point.icon} size={20} />
            </div>
            <div class="point-text">
              <span class="point-label">{point.label}</span>
              <span class="point-desc muted">{point.desc}</span>
            </div>
          </div>
        {/each}
      </div>

      {#if isLast}
        <div class="auth-box">
          <button class="btn accent full login-btn" onclick={player.login}>
            <Icon name="user" size={18} />
            Google ile Bağlan
          </button>
          <button class="btn full guest-btn" onclick={player.closeOnboarding}>
            Misafir Olarak Devam Et
          </button>
        </div>
      {/if}
    </div>

    <footer class="modal-foot">
      <div class="dots" role="tablist" aria-label="Adım seçici">
        {#each steps as _, idx}
          <button
            type="button"
            class="dot"
            class:active={currentStep === idx}
            aria-label={`Adım ${idx + 1}`}
            onclick={() => (currentStep = idx)}
          ></button>
        {/each}
      </div>

      <div class="nav-btns">
        {#if currentStep > 0}
          <button type="button" class="btn" onclick={prev}>
            <Icon name="prev" size={15} />
            Geri
          </button>
        {/if}

        {#if !isLast}
          <button type="button" class="btn accent" onclick={next}>
            İleri
            <Icon name="next" size={15} />
          </button>
        {:else}
          <button type="button" class="btn" onclick={player.closeOnboarding}>
            Tamamla
          </button>
        {/if}
      </div>
    </footer>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 1000;
    background: rgba(0, 0, 0, 0.78);
    backdrop-filter: blur(4px);
    display: grid;
    place-items: center;
    padding: 16px;
    animation: fadeIn 140ms ease;
  }

  .modal {
    width: 100%;
    max-width: 580px;
    max-height: calc(100vh - 32px);
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius-lg);
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.45), 0 var(--edge-depth) 0 var(--edge);
    animation: popUp 160ms cubic-bezier(0.16, 1, 0.3, 1);
  }

  .modal-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px 12px;
    border-bottom: 1px solid var(--border);
  }

  .head-left {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .accent-badge {
    background: var(--accent);
    color: var(--on-accent);
    font-weight: 800;
    font-size: 11px;
    padding: 3px 9px;
    border-radius: var(--radius-sm);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .step-indicator {
    font-size: 12px;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
  }

  .close-btn {
    width: 32px;
    height: 32px;
    min-height: 32px;
    padding: 0;
    border-radius: var(--radius-sm);
  }

  .modal-body {
    padding: 24px 24px 20px;
    display: flex;
    flex-direction: column;
    gap: 18px;
  }

  .icon-hero {
    display: flex;
    justify-content: center;
  }

  .hero-bubble {
    width: 72px;
    height: 72px;
    border-radius: 20px;
    background: var(--accent-soft);
    color: var(--accent);
    border: 2px solid var(--accent);
    display: grid;
    place-items: center;
    box-shadow: 0 var(--edge-depth) 0 var(--accent-edge);
  }

  .text-block {
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .title {
    font-size: 22px;
    font-weight: 800;
    line-height: 1.25;
  }

  .subtitle {
    font-size: 14px;
    line-height: 1.45;
  }

  .points-grid {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-top: 4px;
  }

  .point-card {
    display: flex;
    align-items: flex-start;
    gap: 14px;
    padding: 12px 14px;
    background: var(--surface-2);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    box-shadow: 0 min(2px, var(--edge-depth)) 0 var(--edge);
  }

  .point-icon {
    width: 36px;
    height: 36px;
    border-radius: var(--radius-sm);
    background: var(--surface-3);
    color: var(--accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }

  .point-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .point-label {
    font-weight: 700;
    font-size: 14px;
  }

  .point-desc {
    font-size: 12.5px;
    line-height: 1.4;
  }

  .auth-box {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding-top: 6px;
  }

  .login-btn {
    min-height: 44px;
    font-weight: 800;
    font-size: 14px;
  }

  .guest-btn {
    min-height: 38px;
    font-weight: 600;
    font-size: 13px;
  }

  .modal-foot {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 20px 16px;
    border-top: 1px solid var(--border);
    background: var(--surface);
  }

  .dots {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .dot {
    width: 10px;
    height: 10px;
    border-radius: 5px;
    border: 0;
    padding: 0;
    background: var(--border);
    cursor: pointer;
    transition: background-color 140ms ease, width 140ms ease;
  }

  .dot.active {
    background: var(--accent);
    width: 22px;
  }

  .nav-btns {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes popUp {
    from {
      transform: scale(0.96) translateY(8px);
      opacity: 0;
    }
    to {
      transform: scale(1) translateY(0);
      opacity: 1;
    }
  }

  @media (max-width: 600px) {
    .modal {
      max-height: calc(100vh - 16px);
    }
    .modal-body {
      padding: 16px 14px 14px;
      gap: 14px;
    }
    .hero-bubble {
      width: 58px;
      height: 58px;
    }
    .title {
      font-size: 19px;
    }
    .modal-foot {
      padding: 12px 14px;
    }
  }
</style>
