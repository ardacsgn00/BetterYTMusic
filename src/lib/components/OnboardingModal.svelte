<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { theme, ACCENT_PRESETS, type ThemeMode, type UiStyle } from '#lib/theme.svelte.ts';
  import Icon, { type IconName } from './Icon.svelte';

  const STEPS: { id: string; tabLabel: string; icon: IconName }[] = [
    { id: 'engine', tabLabel: 'Ses Motoru', icon: 'music' },
    { id: 'modes', tabLabel: '4 Görünüm', icon: 'maximize' },
    { id: 'discovery', tabLabel: 'Akıllı Keşif', icon: 'radio' },
    { id: 'style', tabLabel: 'Kişiselleştirme', icon: 'palette' },
    { id: 'start', tabLabel: 'Masaüstü & Başla', icon: 'user' }
  ];

  const QUICK_THEMES: { id: ThemeMode; label: string; bg: string; surface: string }[] = [
    { id: 'dark', label: 'Koyu', bg: '#121215', surface: '#26262d' },
    { id: 'oled', label: 'Gece', bg: '#070709', surface: '#18181f' },
    { id: 'forest', label: 'Orman', bg: '#0c1512', surface: '#1b2c25' },
    { id: 'plum', label: 'Velvet', bg: '#130e19', surface: '#261d33' },
    { id: 'sepia', label: 'Sepya', bg: '#efe8d8', surface: '#f8f3e8' },
    { id: 'light', label: 'Açık', bg: '#f2f1ed', surface: '#ffffff' }
  ];

  const LYRIC_LINES = [
    { time: '01:08', text: 'Müziğe birebir senkronize akan şarkı sözleri' },
    { time: '01:13', text: 'Dilediğin satıra tıkla, şarkı anında o saniyeye atlasın' },
    { time: '01:19', text: 'Geniş ekranda Kapak, Sözler ve Sıra yan yana' }
  ];

  let currentStep = $state(0);
  let activeLyricIdx = $state(1);
  let viewShowcase = $state<'lyrics' | 'fullscreen' | 'mini'>('lyrics');
  let demoUiStyle = $state<UiStyle>(theme.uiStyle);
  let userInteractedWithStyle = $state(false);

  const previewColors = ACCENT_PRESETS.slice(0, 7);
  const totalSteps = STEPS.length;
  const isLast = $derived(currentStep === totalSteps - 1);

  // Cycle synced lyric highlight while viewing the listening modes step.
  $effect(() => {
    if (currentStep !== 1 || viewShowcase !== 'lyrics') return;
    const timer = setInterval(() => {
      activeLyricIdx = (activeLyricIdx + 1) % LYRIC_LINES.length;
    }, 2200);
    return () => clearInterval(timer);
  });

  // Gently demonstrate seamless vs boxed transition until the user picks one directly.
  $effect(() => {
    if (currentStep !== 3 || userInteractedWithStyle) return;
    const timer = setInterval(() => {
      demoUiStyle = demoUiStyle === 'seamless' ? 'boxed' : 'seamless';
    }, 2600);
    return () => clearInterval(timer);
  });

  function selectUiStyle(style: UiStyle) {
    userInteractedWithStyle = true;
    demoUiStyle = style;
    theme.setUiStyle(style);
  }

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
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  class="guide-backdrop"
  role="dialog"
  aria-modal="true"
  aria-label="BetterYTMusic Tanıtım Rehberi"
  tabindex="-1"
  onclick={(e) => {
    if (e.target === e.currentTarget) player.closeOnboarding();
  }}
>
  <div class="guide-card">
    <header class="guide-header">
      <div class="brand">
        <span class="logo-box">
          <Icon name="play" size={18} />
        </span>
        <div class="brand-text">
          <span class="brand-name">{theme.appName.trim() || 'BetterYTMusic'}</span>
          <span class="brand-tag muted">Tarayıcı Yükü Olmadan YouTube Music</span>
        </div>
      </div>

      <div class="header-actions">
        <button
          type="button"
          class="btn ghost sm skip-btn"
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

    <!-- Clickable Named Step Strip -->
    <nav class="step-nav" aria-label="Rehber adımları">
      {#each STEPS as step, idx (step.id)}
        <button
          type="button"
          class="step-chip"
          class:active={currentStep === idx}
          class:done={idx < currentStep}
          onclick={() => (currentStep = idx)}
        >
          <span class="chip-num">{idx + 1}</span>
          <span class="chip-label">{step.tabLabel}</span>
        </button>
      {/each}
    </nav>

    <div class="guide-body">
      {#if currentStep === 0}
        <!-- Step 1: Pure Rust Audio Engine, Prefetch & Loudness Normalization -->
        <div class="step-split step-fade">
          <div class="showcase-col">
            <div class="bench-card">
              <div class="bench-head">
                <span class="bench-eyebrow muted">TARAYICI SEKMESİ VS YEREL MOTOR</span>
              </div>

              <div class="bench-row muted-row">
                <div class="bench-label">
                  <span>Tarayıcıda YouTube Music</span>
                  <span class="bench-metric">~450 MB RAM · Reklam</span>
                </div>
                <div class="bench-track"><div class="bench-fill slow"></div></div>
              </div>

              <div class="bench-row hero-row">
                <div class="bench-label">
                  <span>{theme.appName.trim() || 'BetterYTMusic'}</span>
                  <span class="bench-metric accent-txt">~60 MB RAM · Saf Rust</span>
                </div>
                <div class="bench-track">
                  <div class="bench-fill fast" style:background-color={theme.accent}></div>
                </div>
              </div>

              <div class="engine-features">
                <div class="eng-pill">
                  <span class="eng-dot"></span>
                  <div>
                    <strong>0 ms Geçiş (Ön Yükleme)</strong>
                    <span class="muted">Sıradaki parça arka planda hazır bekler</span>
                  </div>
                </div>
                <div class="eng-pill">
                  <span class="eng-dot"></span>
                  <div>
                    <strong>Ses Seviyesi Dengeleme</strong>
                    <span class="muted">Şarkılar arası ani ses farkını otomatik dengeler</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="copy-col">
            <span class="eyebrow">01 · SAF PERFORMANS & SES KALİTESİ</span>
            <h2 class="step-title">Tarayıcı Sekmesini Kapat, Müziği Hızlandır.</h2>
            <p class="step-desc muted">
              Ağır Chrome/Electron katmanları yerine doğrudan yerel Rust ses motoruyla çalışır. Ram ve işlemciyi yormadan milisaniyelik tepki verir.
            </p>
            <ul class="benefit-list">
              <li>
                <span class="b-icon"><Icon name="music" size={15} /></span>
                <div>
                  <strong>Yerel Audio Pipeline:</strong> Donanıma doğrudan AAC/M4A ses çözme ve paket düzeyinde takılmasız ileri-geri sarma.
                </div>
              </li>
              <li>
                <span class="b-icon"><Icon name="autoplay" size={15} /></span>
                <div>
                  <strong>Akıllı Ön Yükleme:</strong> Sen mevcut parçayı dinlerken sıradaki şarkı arka planda belleğe alınır, atlarken beklemezsin.
                </div>
              </li>
              <li>
                <span class="b-icon"><Icon name="volume" size={15} /></span>
                <div>
                  <strong>Loudness Normalization:</strong> Bir parça çok kısık, diğeri çok yüksek çalmaz; ses düzeyi doğal dengede kalır.
                </div>
              </li>
            </ul>
          </div>
        </div>

      {:else if currentStep === 1}
        <!-- Step 2: 4 Views + Synced Lyrics + Fullscreen + Mini + Resume Session -->
        <div class="step-split step-fade">
          <div class="showcase-col">
            <div class="mode-switcher">
              <button
                type="button"
                class="m-sw-btn"
                class:active={viewShowcase === 'lyrics'}
                onclick={() => (viewShowcase = 'lyrics')}
              >
                <Icon name="lyrics" size={14} />
                Sözler & Sıra
              </button>
              <button
                type="button"
                class="m-sw-btn"
                class:active={viewShowcase === 'fullscreen'}
                onclick={() => (viewShowcase = 'fullscreen')}
              >
                <Icon name="maximize" size={14} />
                Tam Ekran (F)
              </button>
              <button
                type="button"
                class="m-sw-btn"
                class:active={viewShowcase === 'mini'}
                onclick={() => (viewShowcase = 'mini')}
              >
                <Icon name="mini" size={14} />
                Mini Oynatıcı
              </button>
            </div>

            {#if viewShowcase === 'lyrics'}
              <div class="karaoke-card">
                <div class="karaoke-head">
                  <span class="karaoke-dot"></span>
                  <span class="karaoke-tag muted">CANLI SENKRONİZE LRC</span>
                  <div class="sound-wave" aria-hidden="true">
                    <span class="sw-bar b1" style:background-color={theme.accent}></span>
                    <span class="sw-bar b2" style:background-color={theme.accent}></span>
                    <span class="sw-bar b3" style:background-color={theme.accent}></span>
                    <span class="sw-bar b4" style:background-color={theme.accent}></span>
                  </div>
                </div>
                <div class="karaoke-lines">
                  {#each LYRIC_LINES as line, i (line.time)}
                    <button
                      type="button"
                      class="k-line"
                      class:active={activeLyricIdx === i}
                      onclick={() => (activeLyricIdx = i)}
                    >
                      <span class="k-idx">{line.time}</span>
                      <span>{line.text}</span>
                    </button>
                  {/each}
                </div>
              </div>
            {:else if viewShowcase === 'fullscreen'}
              <div class="fs-mock-card">
                <div class="fs-mock-art" style:background-color={theme.accent}>
                  <Icon name="disc" size={32} />
                </div>
                <div class="fs-mock-right">
                  <span class="fs-mock-badge">TAM EKRAN MODU · F / ESC</span>
                  <strong>Büyük Kapak + Büyük Sözler</strong>
                  <span class="muted">Dikkat dağıtan tüm menüleri gizler; masaüstünde 2 sütunlu sinematik dinleme sunar.</span>
                  <div class="fs-mock-ctrls">
                    <Icon name="prev" size={14} />
                    <span class="fs-mock-play" style:background-color={theme.accent}>
                      <Icon name="play" size={12} />
                    </span>
                    <Icon name="next" size={14} />
                  </div>
                </div>
              </div>
            {:else}
              <div class="mini-mock-stage">
                <div class="mini-mock-box strip">
                  <div class="mm-art" style:background-color={theme.accent}><Icon name="now" size={15} /></div>
                  <div class="mm-txt">
                    <strong>Yatay Şerit Modu</strong>
                    <span class="muted">Her zaman en üstte sabitlenir</span>
                  </div>
                  <span class="mm-play" style:background-color={theme.accent}><Icon name="pause" size={10} /></span>
                </div>
                <div class="mini-mock-box square-pill">
                  <Icon name="pin" size={14} />
                  <span><strong>Kare Kart Modu:</strong> Büyük kapak üstünde kompakt kontroller</span>
                </div>
              </div>
            {/if}

            <div class="resume-pill">
              <Icon name="refresh" size={14} />
              <span><strong>Kaldığın Yerden Devam Et:</strong> Kapatıp açtığında kuyruğun ve saniyen hazır</span>
            </div>
          </div>

          <div class="copy-col">
            <span class="eyebrow">02 · HER ANINA UYUM SAĞLAYAN GÖRÜNÜM</span>
            <h2 class="step-title">Sözlerden Tam Ekrana, Tek Tuşla Geçiş.</h2>
            <p class="step-desc muted">
              İster şarkı sözlerine odaklan, ister çalışırken ekran köşesine sabitle. Kaldığın yer daima korunur.
            </p>
            <ul class="benefit-list">
              <li>
                <span class="b-icon"><Icon name="lyrics" size={15} /></span>
                <div>
                  <strong>Etkileşimli Şarkı Sözleri:</strong> Senkronize LRC satırına tıkladığın anda şarkı doğrudan o saniyeye sarar; geniş ekranda 3 sütunlu görünüm açılır.
                </div>
              </li>
              <li>
                <span class="b-icon"><Icon name="maximize" size={15} /></span>
                <div>
                  <strong>Tam Ekran (F) & Mini Oynatıcı:</strong> Dikkati dağıtan her şeyi gizleyen Tam Ekran sahnesi ya da oyun/iş üstünde kalan Yatay Şerit / Kare Mini Oynatıcı.
                </div>
              </li>
              <li>
                <span class="b-icon"><Icon name="refresh" size={15} /></span>
                <div>
                  <strong>Oturum Kurtarma:</strong> Uygulamayı kapattığında çalma listen, sıran ve milisaniyelik konumun kaydedilir; dönüşte kaldığın yerden başlarsın.
                </div>
              </li>
            </ul>
          </div>
        </div>

      {:else if currentStep === 2}
        <!-- Step 3: Smart Discovery, Infinite Autoplay, Radio & Artist Discography -->
        <div class="step-split step-fade">
          <div class="showcase-col">
            <div class="discovery-card">
              <div class="disc-top">
                <div class="disc-track">
                  <div class="disc-cover" style:background-color={theme.accent}>
                    <Icon name="radio" size={20} />
                  </div>
                  <div>
                    <span class="disc-tag">AUTOPLAY & ŞARKI RADYOSU</span>
                    <strong>Benzer Parçalar Otomatik Eklenir</strong>
                  </div>
                </div>
                <span class="disc-status">Kesintisiz Akış</span>
              </div>

              <div class="disc-chips">
                <span class="d-chip active" style:background-color={theme.accent}>Odaklanma</span>
                <span class="d-chip">Enerji</span>
                <span class="d-chip">Rahatlama</span>
                <span class="d-chip">Gece Sürüşü</span>
              </div>

              <div class="disc-artist-box">
                <div class="da-avatar"><Icon name="user" size={16} /></div>
                <div class="da-meta">
                  <strong>Sanatçı Profili & Diskografi</strong>
                  <span class="muted">Popüler şarkılar · Albümler · Single'lar · Benzer sanatçılar</span>
                </div>
              </div>
            </div>
          </div>

          <div class="copy-col">
            <span class="eyebrow">03 · SINIRSIZ MÜZİK KEŞFİ</span>
            <h2 class="step-title">Müzik Asla Durmaz, Aradığın Mod Hep Hazır.</h2>
            <p class="step-desc muted">
              Beğendiğin tek bir şarkıdan bütün bir geceye yayılan radyo oluştur ya da ruh haline göre çalma listelerini süz.
            </p>
            <ul class="benefit-list">
              <li>
                <span class="b-icon"><Icon name="autoplay" size={15} /></span>
                <div>
                  <strong>Sonsuz Otomatik Çalma (Autoplay):</strong> Kuyruğun sonuna yaklaştığında YouTube Music üzerinden zevkine uygun parçalar otomatik eklenir.
                </div>
              </li>
              <li>
                <span class="b-icon"><Icon name="radio" size={15} /></span>
                <div>
                  <strong>Tek Tıkla Şarkı Radyosu:</strong> Herhangi bir şarkı veya sanatçıdan anında kesintisiz radyo akışı başlat.
                </div>
              </li>
              <li>
                <span class="b-icon"><Icon name="user" size={15} /></span>
                <div>
                  <strong>Sanatçı Sayfaları & Ruh Halleri:</strong> Sanatçı adına tıklayıp tam diskografiyi keşfet, Ana Sayfa'dan ruh hali/tür chip'leriyle filtrele.
                </div>
              </li>
            </ul>
          </div>
        </div>

      {:else if currentStep === 3}
        <!-- Step 4: Live Interactive Customization (12 Solid Themes, Fonts, Tactile vs Seamless) -->
        <div class="step-split step-fade">
          <div class="showcase-col">
            <div class="theme-showcase">
              <div class="theme-row-block">
                <span class="theme-hint muted">Canlı Tema Dene (12 Tema)</span>
                <div class="quick-theme-grid">
                  {#each QUICK_THEMES as qt (qt.id)}
                    <button
                      type="button"
                      class="qt-btn"
                      class:active={theme.resolved === qt.id}
                      onclick={() => theme.setMode(qt.id)}
                    >
                      <span class="qt-swatch" style:background-color={qt.bg}>
                        <span style:background-color={qt.surface}></span>
                      </span>
                      <span>{qt.label}</span>
                    </button>
                  {/each}
                </div>
              </div>

              <div class="theme-row-block inline-colors">
                <span class="theme-hint muted">Vurgu Rengi</span>
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
                        <Icon name="check" size={12} />
                      {/if}
                    </button>
                  {/each}
                </div>
              </div>

              <div
                class="ui-morph-card"
                class:is-boxed={demoUiStyle === 'boxed'}
                class:is-seamless={demoUiStyle === 'seamless'}
              >
                <div class="mock-card-inner">
                  <div class="mock-cover-box" style:background-color={theme.accent}>
                    <Icon name="now" size={18} />
                  </div>
                  <div class="mock-meta">
                    <span class="mock-mode-pill" class:boxed-pill={demoUiStyle === 'boxed'}>
                      {demoUiStyle === 'boxed' ? '3B DOKUNSAL (KUTULU)' : 'AKICI (KUTUSUZ)'}
                    </span>
                    <span class="mock-song truncate">Süpermix & Çalma Listesi</span>
                  </div>
                  <span class="mock-play-btn" class:tactile-btn={demoUiStyle === 'boxed'} style:background-color={theme.accent}>
                    <Icon name="play" size={13} />
                  </span>
                </div>
              </div>

              <div class="style-toggle-row">
                <button
                  type="button"
                  class="btn sm style-btn"
                  class:accent={demoUiStyle === 'seamless'}
                  onclick={() => selectUiStyle('seamless')}
                >
                  Kutusuz & Akıcı
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
          </div>

          <div class="copy-col">
            <span class="eyebrow">04 · TAM KİŞİSELLEŞTİRME</span>
            <h2 class="step-title">Senin Tarzın, Anında Canlı Değişim.</h2>
            <p class="step-desc muted">
              Sol taraftaki tema, vurgu veya kutu stiline dokunarak tüm uygulamanın anında nasıl dönüştüğünü gör.
            </p>
            <ul class="benefit-list">
              <li>
                <span class="b-icon"><Icon name="palette" size={15} /></span>
                <div>
                  <strong>12 Solid Tema & Vurgu Rengi:</strong> OLED siyahından Çam Ormanı, Velvet Erik ve Parşömen Sepya'ya kadar göz yormayan düz renk paletleri.
                </div>
              </li>
              <li>
                <span class="b-icon"><Icon name="columns" size={15} /></span>
                <div>
                  <strong>Kutusuz Akıcı veya 3B Dokunsal:</strong> Sade modern düzen ile belirgin alt kenarlı 3B buton/kart stili arasında tek tıkla geçiş.
                </div>
              </li>
              <li>
                <span class="b-icon"><Icon name="settings" size={15} /></span>
                <div>
                  <strong>10 Yazı Tipi & Esnek Menü:</strong> Menü sekme sıralaması, ikon modu, liste sıklığı ve sürüklenebilir panel genişlikleri.
                </div>
              </li>
            </ul>
          </div>
        </div>

      {:else}
        <!-- Step 5: Desktop Integration, Audio Output, Discord & Start Now -->
        <div class="step-split step-fade">
          <div class="showcase-col">
            <div class="onboard-discord-card">
              <div class="ob-d-head">
                <div class="ob-d-user">
                  <div class="ob-d-avatar">
                    <Icon name="now" size={15} />
                    <span class="ob-d-status-dot"></span>
                  </div>
                  <div class="ob-d-meta">
                    <span class="ob-d-name">{player.auth.accountName || 'Senin Profilin'}</span>
                    <span class="ob-d-tag">DISCORD ZENGİN VARLIK</span>
                  </div>
                </div>
                <span class="ob-d-badge">Otomatik</span>
              </div>

              <div class="ob-d-body">
                <div class="ob-d-art" style:background-color={theme.accent}>
                  <Icon name="music" size={22} />
                  <span class="ob-d-play-pip"><Icon name="play" size={9} /></span>
                </div>
                <div class="ob-d-info">
                  <span class="ob-d-title">{theme.appName.trim() || 'BetterYTMusic'}</span>
                  <span class="ob-d-song truncate">{player.state.track?.title || 'Süpermix & Çalma Listeleri'}</span>
                  <span class="ob-d-artist truncate">{player.state.track?.artist || 'Canlı şarkı ve süre görünümü'}</span>
                </div>
              </div>
            </div>

            <div class="desktop-perks-grid">
              <div class="perk-box">
                <Icon name="speaker" size={16} />
                <span><strong>Ses Cihazı Seçimi</strong><small class="muted">Kulaklık/hoparlör anında değişir</small></span>
              </div>
              <div class="perk-box">
                <Icon name="windows" size={16} />
                <span><strong>Tepsi & Medya Tuşları</strong><small class="muted">Windows & macOS tam uyumu</small></span>
              </div>
            </div>
          </div>

          <div class="copy-col">
            <span class="eyebrow">05 · MASAÜSTÜ GÜCÜ & HEMEN BAŞLA</span>
            <h2 class="step-title">Kurulum Bitti, Müzik Seni Bekliyor!</h2>
            <p class="step-desc muted">
              Google hesabını bağlayıp kişisel Süpermix ve beğenilerine hemen ulaşabilir ya da hesap açmadan arama ve radyoyla doğrudan başlayabilirsin.
            </p>

            <div class="auth-actions">
              <button type="button" class="btn accent main-auth-btn" onclick={player.login}>
                <Icon name="user" size={17} />
                Google ile Hesabını Bağla
              </button>
              <button type="button" class="btn guest-auth-btn" onclick={player.closeOnboarding}>
                <Icon name="play" size={16} />
                Hesapsız Devam Et (Misafir)
              </button>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <footer class="guide-footer">
      <div class="footer-left">
        {#if currentStep > 0}
          <button type="button" class="btn sm" onclick={prev}>
            <Icon name="prev" size={13} />
            Geri
          </button>
        {/if}
      </div>

      <span class="step-counter muted">Adım {currentStep + 1} / {totalSteps}</span>

      <div class="footer-right">
        {#if !isLast}
          <button type="button" class="btn accent sm" onclick={next}>
            İleri
            <Icon name="next" size={13} />
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
    background: rgb(0 0 0 / 0.78);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    animation: fade-in 150ms ease-out;
  }

  .guide-card {
    width: min(860px, 95vw);
    max-height: min(680px, 92dvh);
    background: var(--surface) !important;
    border: 2px solid var(--border) !important;
    border-radius: var(--radius-lg);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: pop-in 180ms cubic-bezier(0.16, 1, 0.3, 1);
  }

  /* Header */
  .guide-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 20px;
    border-bottom: 2px solid var(--border);
    background: var(--surface-2) !important;
    gap: 12px;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 11px;
    min-width: 0;
  }

  .logo-box {
    width: 34px;
    height: 34px;
    border-radius: var(--radius-sm);
    background: var(--accent);
    color: var(--on-accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
    border: 2px solid var(--accent-edge);
  }

  .brand-text {
    display: flex;
    flex-direction: column;
    line-height: 1.2;
    min-width: 0;
  }

  .brand-name {
    font-size: 14.5px;
    font-weight: 800;
    letter-spacing: -0.01em;
  }

  .brand-tag {
    font-size: 11.5px;
    font-weight: 600;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
  }

  .skip-btn {
    min-height: 32px;
    padding: 0 10px;
    font-size: 12.5px;
    font-weight: 700;
  }

  .close-btn {
    width: 34px;
    height: 34px;
    min-height: 34px;
  }

  /* Named Step Chips */
  .step-nav {
    display: grid;
    grid-template-columns: repeat(5, minmax(0, 1fr));
    gap: 6px;
    padding: 10px 18px;
    background: var(--surface);
    border-bottom: 1.5px solid var(--border);
    flex-shrink: 0;
  }

  .step-chip {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 7px 8px;
    border-radius: var(--radius-sm);
    border: 1.5px solid transparent;
    background: var(--surface-2);
    color: var(--text-muted);
    font-family: inherit;
    font-size: 12px;
    font-weight: 700;
    cursor: pointer;
    min-width: 0;
    transition: background-color 120ms ease, color 120ms ease, border-color 120ms ease;
  }

  .step-chip:hover {
    background: var(--surface-3);
    color: var(--text);
  }

  .step-chip.done {
    color: var(--text);
    border-color: var(--border);
  }

  .step-chip.active {
    background: var(--accent);
    border-color: var(--accent-edge);
    color: var(--on-accent);
    font-weight: 800;
  }

  .chip-num {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: rgb(0 0 0 / 0.22);
    display: grid;
    place-items: center;
    font-size: 10.5px;
    font-weight: 800;
    flex-shrink: 0;
  }

  .chip-label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  /* Main Split Body */
  .guide-body {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 24px 24px;
    display: flex;
    align-items: center;
    background: var(--surface) !important;
  }

  .step-split {
    width: 100%;
    display: grid;
    grid-template-columns: minmax(0, 1.05fr) minmax(0, 1fr);
    gap: 24px;
    align-items: center;
  }

  .step-fade {
    animation: step-appear 180ms ease-out;
  }

  .showcase-col {
    display: flex;
    flex-direction: column;
    gap: 10px;
    min-width: 0;
  }

  .copy-col {
    display: flex;
    flex-direction: column;
    gap: 10px;
    text-align: left;
    min-width: 0;
  }

  .eyebrow {
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 0.06em;
    color: var(--accent);
  }

  .step-title {
    font-size: clamp(20px, 2.2vw, 25px);
    font-weight: 800;
    letter-spacing: -0.02em;
    line-height: 1.2;
  }

  .step-desc {
    font-size: 13.5px;
    line-height: 1.5;
  }

  .benefit-list {
    list-style: none;
    margin: 4px 0 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .benefit-list li {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    font-size: 12.8px;
    line-height: 1.45;
    color: var(--text-muted);
  }

  .benefit-list strong {
    color: var(--text);
  }

  .b-icon {
    width: 28px;
    height: 28px;
    border-radius: 8px;
    background: var(--accent-soft);
    color: var(--accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
    margin-top: 1px;
  }

  /* Step 1: Engine Comparison Card */
  .bench-card {
    background: var(--surface-2);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .bench-eyebrow {
    font-size: 10.5px;
    font-weight: 800;
    letter-spacing: 0.05em;
  }

  .bench-row {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .bench-label {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12.5px;
    font-weight: 700;
    gap: 8px;
  }

  .muted-row .bench-label {
    color: var(--text-muted);
  }

  .bench-metric {
    font-size: 11.5px;
    font-variant-numeric: tabular-nums;
  }

  .accent-txt {
    color: var(--accent);
    font-weight: 800;
  }

  .bench-track {
    height: 10px;
    border-radius: 999px;
    background: var(--surface);
    border: 1px solid var(--border);
    overflow: hidden;
  }

  .bench-fill.slow {
    width: 88%;
    height: 100%;
    background: var(--surface-3);
  }

  .bench-fill.fast {
    width: 14%;
    height: 100%;
    border-radius: 999px;
  }

  .engine-features {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-top: 4px;
    border-top: 1px solid var(--border);
  }

  .eng-pill {
    display: flex;
    align-items: flex-start;
    gap: 9px;
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    background: var(--surface);
    border: 1.5px solid var(--border);
    font-size: 12px;
  }

  .eng-pill > div {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .eng-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #06d6a0;
    margin-top: 4px;
    flex-shrink: 0;
  }

  /* Step 2: 4 Views Switcher */
  .mode-switcher {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 6px;
  }

  .m-sw-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 5px;
    height: 34px;
    padding: 0 8px;
    border-radius: var(--radius-sm);
    border: 2px solid var(--border);
    background: var(--surface-2);
    color: var(--text-muted);
    font-family: inherit;
    font-size: 11.5px;
    font-weight: 700;
    cursor: pointer;
    white-space: nowrap;
  }

  .m-sw-btn.active {
    background: var(--accent);
    border-color: var(--accent-edge);
    color: var(--on-accent);
    font-weight: 800;
  }

  .karaoke-card {
    background: var(--surface-2);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    padding: 14px 16px;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
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
    font-size: 10.5px;
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
    gap: 6px;
  }

  .k-line {
    width: 100%;
    text-align: left;
    display: flex;
    align-items: center;
    gap: 9px;
    padding: 7px 10px;
    border-radius: var(--radius-sm);
    border: 1.5px solid transparent;
    background: transparent;
    font-family: inherit;
    font-size: 12.5px;
    font-weight: 600;
    color: var(--text-muted);
    cursor: pointer;
    transition: background-color 150ms ease, color 150ms ease;
  }

  .k-idx {
    font-size: 11px;
    font-variant-numeric: tabular-nums;
    opacity: 0.65;
  }

  .k-line.active {
    background: var(--surface);
    border-color: var(--accent);
    color: var(--text);
    font-weight: 700;
  }

  .fs-mock-card {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 16px;
    border-radius: var(--radius);
    background: var(--bg);
    border: 2px solid var(--border);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .fs-mock-art {
    width: 82px;
    height: 82px;
    border-radius: var(--radius);
    color: var(--on-accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }

  .fs-mock-right {
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 12px;
    min-width: 0;
  }

  .fs-mock-badge {
    font-size: 10px;
    font-weight: 800;
    color: var(--accent);
    letter-spacing: 0.04em;
  }

  .fs-mock-ctrls {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 4px;
    color: var(--text-muted);
  }

  .fs-mock-play {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    color: var(--on-accent);
    display: grid;
    place-items: center;
  }

  .mini-mock-stage {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 14px;
    border-radius: var(--radius);
    background: var(--bg);
    border: 2px solid var(--border);
  }

  .mini-mock-box.strip {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    border: 2px solid var(--border);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .mm-art {
    width: 34px;
    height: 34px;
    border-radius: 8px;
    color: var(--on-accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }

  .mm-txt {
    flex: 1;
    display: flex;
    flex-direction: column;
    font-size: 12px;
    min-width: 0;
  }

  .mm-play {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    color: var(--on-accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }

  .mini-mock-box.square-pill {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    background: var(--surface);
    border: 1.5px solid var(--border);
    font-size: 11.5px;
    color: var(--text-muted);
  }

  .mini-mock-box.square-pill strong {
    color: var(--text);
  }

  .resume-pill {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-radius: var(--radius-sm);
    background: var(--accent-soft);
    border: 1.5px solid var(--accent);
    font-size: 12px;
    color: var(--text);
  }

  /* Step 3: Discovery Card */
  .discovery-card {
    background: var(--surface-2);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .disc-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    flex-wrap: wrap;
  }

  .disc-track {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .disc-cover {
    width: 42px;
    height: 42px;
    border-radius: var(--radius-sm);
    color: var(--on-accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }

  .disc-track > div:last-child {
    display: flex;
    flex-direction: column;
    font-size: 13px;
  }

  .disc-tag {
    font-size: 10px;
    font-weight: 800;
    color: var(--accent);
    letter-spacing: 0.04em;
  }

  .disc-status {
    font-size: 11px;
    font-weight: 800;
    padding: 3px 8px;
    border-radius: 999px;
    background: var(--accent-soft);
    color: var(--accent);
  }

  .disc-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .d-chip {
    padding: 4px 10px;
    border-radius: 999px;
    font-size: 11.5px;
    font-weight: 700;
    background: var(--surface);
    border: 1.5px solid var(--border);
    color: var(--text-muted);
  }

  .d-chip.active {
    color: var(--on-accent);
    border-color: var(--accent-edge);
  }

  .disc-artist-box {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    border-radius: var(--radius-sm);
    background: var(--surface);
    border: 1.5px solid var(--border);
    font-size: 12px;
  }

  .da-avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: var(--surface-3);
    color: var(--accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }

  .da-meta {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  /* Step 4: Theme & Style Showcase */
  .theme-showcase {
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 11px;
    background: var(--surface-2) !important;
    border: 2px solid var(--border) !important;
    border-radius: var(--radius);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .theme-row-block {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .theme-row-block.inline-colors {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .theme-hint {
    font-size: 11px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .quick-theme-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 6px;
  }

  .qt-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    border-radius: var(--radius-sm);
    border: 1.5px solid var(--border);
    background: var(--surface);
    color: var(--text);
    font-family: inherit;
    font-size: 11.5px;
    font-weight: 700;
    cursor: pointer;
  }

  .qt-btn.active {
    border-color: var(--accent);
    background: var(--accent-soft);
  }

  .qt-swatch {
    width: 16px;
    height: 16px;
    border-radius: 5px;
    border: 1px solid var(--border);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }

  .qt-swatch > span {
    width: 7px;
    height: 7px;
    border-radius: 2px;
  }

  .color-swatches {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
  }

  .color-dot {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    border: 2px solid transparent;
    color: #fff;
    cursor: pointer;
    display: grid;
    place-items: center;
    transition: transform 100ms ease;
  }

  .color-dot:hover {
    transform: scale(1.12);
  }

  .color-dot.active {
    border-color: var(--text);
    transform: scale(1.12);
  }

  .ui-morph-card {
    border-radius: var(--radius);
    padding: 10px 12px;
    background: var(--surface);
    transition: border-color 180ms ease, box-shadow 180ms ease, transform 180ms ease;
  }

  .ui-morph-card.is-seamless {
    border: 1px solid var(--border);
    box-shadow: none;
  }

  .ui-morph-card.is-boxed {
    border: 2px solid var(--border);
    box-shadow: 0 4px 0 var(--edge);
    transform: translateY(-1px);
  }

  .mock-card-inner {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .mock-cover-box {
    width: 38px;
    height: 38px;
    border-radius: var(--radius-sm);
    color: var(--on-accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }

  .mock-meta {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .mock-mode-pill {
    align-self: flex-start;
    font-size: 9.5px;
    font-weight: 800;
    letter-spacing: 0.04em;
    padding: 1px 6px;
    border-radius: 4px;
    background: var(--surface-3);
    color: var(--text-muted);
  }

  .mock-mode-pill.boxed-pill {
    background: var(--accent-soft);
    color: var(--accent);
  }

  .mock-song {
    font-size: 12.5px;
    font-weight: 800;
  }

  .mock-play-btn {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    color: var(--on-accent);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }

  .mock-play-btn.tactile-btn {
    box-shadow: 0 3px 0 var(--accent-edge);
  }

  .style-toggle-row {
    display: flex;
    gap: 8px;
  }

  .style-btn {
    flex: 1;
    font-size: 12px;
    font-weight: 700;
    min-height: 34px;
  }

  /* Step 5: Discord + Desktop Perks + Auth */
  .onboard-discord-card {
    padding: 12px 14px;
    background: #1e1f22 !important;
    border: 2px solid var(--border) !important;
    border-radius: var(--radius);
    display: flex;
    flex-direction: column;
    gap: 10px;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .ob-d-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .ob-d-user {
    display: flex;
    align-items: center;
    gap: 9px;
  }

  .ob-d-avatar {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    background: #2b2d31;
    color: var(--accent);
    display: grid;
    place-items: center;
    position: relative;
  }

  .ob-d-status-dot {
    position: absolute;
    bottom: -1px;
    right: -1px;
    width: 8px;
    height: 8px;
    background: #23a55a;
    border-radius: 50%;
    border: 2px solid #1e1f22;
  }

  .ob-d-meta {
    display: flex;
    flex-direction: column;
  }

  .ob-d-name {
    font-size: 12.5px;
    font-weight: 800;
    color: #f2f3f5;
  }

  .ob-d-tag {
    font-size: 9.5px;
    font-weight: 800;
    color: #949ba4;
    letter-spacing: 0.04em;
  }

  .ob-d-badge {
    font-size: 10.5px;
    font-weight: 800;
    padding: 2px 8px;
    border-radius: 6px;
    background: rgb(35 165 90 / 0.18);
    color: #23a55a;
  }

  .ob-d-body {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 9px 10px;
    background: #2b2d31;
    border-radius: var(--radius-sm);
    border: 1px solid #35373c;
  }

  .ob-d-art {
    width: 44px;
    height: 44px;
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
    width: 15px;
    height: 15px;
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
    gap: 1px;
  }

  .ob-d-title {
    font-size: 12.5px;
    font-weight: 800;
    color: #f2f3f5;
  }

  .ob-d-song {
    font-size: 11.5px;
    font-weight: 700;
    color: #dbdee1;
  }

  .ob-d-artist {
    font-size: 10.5px;
    color: #949ba4;
  }

  .desktop-perks-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .perk-box {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 10px;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    border: 1.5px solid var(--border);
    color: var(--accent);
  }

  .perk-box > span {
    display: flex;
    flex-direction: column;
    gap: 2px;
    font-size: 11.5px;
    color: var(--text);
  }

  .perk-box small {
    font-size: 10.5px;
    line-height: 1.25;
  }

  .auth-actions {
    display: flex;
    flex-direction: column;
    gap: 9px;
    margin-top: 6px;
  }

  .main-auth-btn {
    height: 44px;
    font-size: 14px;
    font-weight: 800;
  }

  .guest-auth-btn {
    height: 40px;
    font-size: 13px;
    font-weight: 700;
  }

  /* Footer */
  .guide-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px;
    border-top: 2px solid var(--border);
    background: var(--surface-2) !important;
    flex-shrink: 0;
  }

  .footer-left,
  .footer-right {
    min-width: 100px;
    display: flex;
  }

  .footer-right {
    justify-content: flex-end;
  }

  .step-counter {
    font-size: 12px;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
  }

  @keyframes wave-bar {
    0% { transform: scaleY(0.35); }
    100% { transform: scaleY(1); }
  }

  @keyframes fade-in {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes pop-in {
    from { opacity: 0; transform: scale(0.97) translateY(6px); }
    to { opacity: 1; transform: scale(1) translateY(0); }
  }

  @keyframes step-appear {
    from { opacity: 0; transform: translateY(4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  @media (max-width: 780px) {
    .guide-backdrop {
      padding: 0;
    }

    .guide-card {
      width: 100%;
      height: 100dvh;
      max-height: 100dvh;
      border-radius: 0;
      border: none !important;
    }

    .step-nav {
      padding: 8px 12px;
      gap: 4px;
    }

    .step-chip {
      padding: 6px 4px;
      font-size: 11px;
    }

    .chip-label {
      display: none;
    }

    .step-chip.active .chip-label {
      display: inline;
    }

    .guide-body {
      padding: 18px 16px;
      align-items: flex-start;
    }

    .step-split {
      grid-template-columns: 1fr;
      gap: 18px;
    }

    .copy-col {
      order: -1;
    }

    .desktop-perks-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
