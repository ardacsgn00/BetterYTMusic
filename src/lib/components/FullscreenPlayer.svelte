<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { library } from '#lib/library.svelte.ts';
  import { theme } from '#lib/theme.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import Cover from './Cover.svelte';
  import Icon from './Icon.svelte';
  import Slider from './Slider.svelte';
  import LyricsPanel from './LyricsPanel.svelte';
  import Queue from './Queue.svelte';

  let rightTab = $state<'lyrics' | 'queue'>('lyrics');
  let lastVolume = 0.8;
  let isIdle = $state(false);

  const IDLE_TIMEOUT_MS = 2600;
  const s = $derived(player.state);
  const track = $derived(s.track);
  const hasTrack = $derived(Boolean(track));
  const hideChrome = $derived(isIdle && hasTrack);
  const liked = $derived(library.isFavorite(track?.id));
  const endTimeLabel = $derived.by(() => {
    if (theme.showRemainingTime && s.durationMs > 0) {
      return `-${formatTime(Math.max(0, s.durationMs - player.position))}`;
    }
    return formatTime(s.durationMs);
  });
  const volumeIcon = $derived(s.volume === 0 ? 'mute' : s.volume < 0.5 ? 'volume-low' : 'volume');
  const repeatLabel = $derived(
    s.repeat === 'one' ? 'Tekrar: tek şarkı' : s.repeat === 'all' ? 'Tekrar: tüm kuyruk' : 'Tekrar: kapalı'
  );

  // Auto-hide controls after pointer inactivity when a track is loaded.
  // Depend only on `hasTrack` so advancing between tracks keeps idle cinema state undisturbed.
  $effect(() => {
    if (!hasTrack) {
      isIdle = false;
      return;
    }

    let timer: ReturnType<typeof setTimeout> | null = null;
    let isPointerDown = false;
    let lastX: number | null = null;
    let lastY: number | null = null;

    const clearTimer = () => {
      if (timer !== null) {
        clearTimeout(timer);
        timer = null;
      }
    };

    const scheduleIdle = () => {
      clearTimer();
      if (isPointerDown) return;
      timer = setTimeout(() => {
        isIdle = true;
      }, IDLE_TIMEOUT_MS);
    };

    const wake = () => {
      if (isIdle) isIdle = false;
      scheduleIdle();
    };

    const onPointerMove = (e: PointerEvent | MouseEvent) => {
      // Ignore synthetic 0-1px jitter caused by layout animations under a stationary cursor.
      if (
        lastX !== null &&
        lastY !== null &&
        Math.abs(e.clientX - lastX) < 2 &&
        Math.abs(e.clientY - lastY) < 2
      ) {
        return;
      }
      lastX = e.clientX;
      lastY = e.clientY;
      wake();
    };

    const onPointerDown = (e: PointerEvent) => {
      isPointerDown = true;
      lastX = e.clientX;
      lastY = e.clientY;
      wake();
    };

    const onPointerUp = () => {
      isPointerDown = false;
      scheduleIdle();
    };

    const onKeyDown = () => wake();
    const onTouchStart = () => wake();
    const onWindowBlur = () => {
      isPointerDown = false;
    };

    isIdle = false;
    scheduleIdle();

    window.addEventListener('pointermove', onPointerMove, { passive: true });
    window.addEventListener('mousemove', onPointerMove, { passive: true });
    window.addEventListener('pointerdown', onPointerDown, { passive: true });
    window.addEventListener('pointerup', onPointerUp, { passive: true });
    window.addEventListener('pointercancel', onPointerUp, { passive: true });
    window.addEventListener('keydown', onKeyDown, { passive: true });
    window.addEventListener('touchstart', onTouchStart, { passive: true });
    window.addEventListener('blur', onWindowBlur, { passive: true });

    return () => {
      clearTimer();
      window.removeEventListener('pointermove', onPointerMove);
      window.removeEventListener('mousemove', onPointerMove);
      window.removeEventListener('pointerdown', onPointerDown);
      window.removeEventListener('pointerup', onPointerUp);
      window.removeEventListener('pointercancel', onPointerUp);
      window.removeEventListener('keydown', onKeyDown);
      window.removeEventListener('touchstart', onTouchStart);
      window.removeEventListener('blur', onWindowBlur);
    };
  });

  function toggleMute() {
    if (s.volume > 0) {
      lastVolume = s.volume;
      player.setVolume(0);
    } else {
      player.setVolume(lastVolume || 0.8);
    }
  }

  function openArtistFromFullscreen() {
    if (!track) return;
    player.setFullscreen(false);
    player.openArtist(track.artist);
  }
</script>

<div class="fs-root" class:is-idle={hideChrome} role="dialog" aria-label="Tam Ekran Oynatıcı">
  <div class="fs-top-collapse" aria-hidden={hideChrome}>
    <div class="fs-top-inner">
      <header class="fs-top">
        <div class="fs-source-group">
          <span class="fs-pill">{s.sourceName || 'Şimdi Çalıyor'}</span>
          {#if s.queue.length > 0 && s.index !== null}
            <span class="fs-count">{s.index + 1} / {s.queue.length}</span>
          {/if}
        </div>

        <div class="fs-top-actions">
          <div class="fs-tabs" role="tablist" aria-label="Tam ekran sağ panel">
            <button
              type="button"
              role="tab"
              aria-selected={rightTab === 'lyrics'}
              class="fs-tab"
              class:active={rightTab === 'lyrics'}
              onclick={() => (rightTab = 'lyrics')}
            >
              <Icon name="lyrics" size={16} />
              <span>Şarkı Sözleri</span>
            </button>
            <button
              type="button"
              role="tab"
              aria-selected={rightTab === 'queue'}
              class="fs-tab"
              class:active={rightTab === 'queue'}
              onclick={() => (rightTab = 'queue')}
            >
              <Icon name="now" size={16} />
              <span>Sıradaki</span>
              {#if s.queue.length > 0}
                <span class="fs-tab-count">{s.queue.length}</span>
              {/if}
            </button>
          </div>

          <span class="fs-hint muted">F / Esc</span>

          <button
            type="button"
            class="btn fs-exit"
            title="Tam Ekrandan Çık (Esc)"
            aria-label="Tam ekrandan çık"
            onclick={() => player.setFullscreen(false)}
          >
            <Icon name="close" size={16} />
            <span class="fs-exit-label">Tam Ekrandan Çık</span>
          </button>
        </div>
      </header>
    </div>
  </div>

  {#if track}
    <main class="fs-stage">
      <section class="fs-left">
        <div class="fs-cover-frame">
          <div class="fs-cover-wrap">
            <Cover {track} size="100%" radius="16px" />
          </div>
        </div>

        <div class="fs-dock-collapse" aria-hidden={hideChrome}>
          <div class="fs-dock-inner">
            <div class="fs-dock-content">
              <div class="fs-meta">
                <h1 class="fs-title">{track.title}</h1>
                <div class="fs-sub-row">
                  <button
                    type="button"
                    class="fs-artist"
                    title="{track.artist} Sanatçı Sayfasını Aç"
                    onclick={openArtistFromFullscreen}
                  >
                    {track.artist}
                  </button>
                  {#if track.album}
                    <span class="fs-dot">•</span>
                    <span class="fs-album truncate">{track.album}</span>
                  {/if}
                </div>
              </div>

              <div class="fs-actions">
                <button
                  type="button"
                  class="btn fs-action-btn"
                  class:accent={liked}
                  title={liked ? 'Beğenilenlerden kaldır' : 'Beğenilenlere ekle'}
                  onclick={() => library.toggle(track)}
                >
                  <Icon name={liked ? 'heart-fill' : 'heart'} size={16} />
                  <span>{liked ? 'Beğenildi' : 'Beğen'}</span>
                </button>

                <button
                  type="button"
                  class="btn fs-action-btn"
                  title="Bu şarkıyla benzer parçalardan radyo başlat"
                  onclick={() => player.startRadio(track)}
                >
                  <Icon name="radio" size={16} />
                  <span>Şarkı Radyosu</span>
                </button>

                <button
                  type="button"
                  class="btn fs-action-btn"
                  class:accent={player.settings.autoplay}
                  title="Kuyruk bitince benzer şarkıları otomatik çal"
                  onclick={() => player.updateSettings({ autoplay: !player.settings.autoplay })}
                >
                  <Icon name="autoplay" size={16} />
                  <span>Otomatik</span>
                </button>
              </div>

              <div class="fs-progress">
                <Slider
                  label="Şarkı konumu"
                  value={player.position}
                  max={s.durationMs}
                  step={250}
                  oncommit={(v) => player.seek(v)}
                />
                <div class="fs-times">
                  <span>{formatTime(player.position)}</span>
                  <button
                    type="button"
                    class="fs-time-btn"
                    title={theme.showRemainingTime ? 'Toplam süreyi göster' : 'Kalan süreyi göster'}
                    onclick={() => theme.toggleRemainingTime()}
                  >
                    {endTimeLabel}
                  </button>
                </div>
              </div>

              <div class="fs-controls">
                <button
                  type="button"
                  class="btn ghost icon fs-side-ctrl"
                  class:on={s.repeat !== 'off'}
                  title={repeatLabel}
                  aria-label={repeatLabel}
                  onclick={player.cycleRepeat}
                >
                  <Icon name={s.repeat === 'one' ? 'repeat-one' : 'repeat'} size={21} />
                </button>

                <button
                  type="button"
                  class="btn icon fs-skip-btn"
                  title="Önceki"
                  aria-label="Önceki"
                  onclick={player.previous}
                >
                  <Icon name="prev" size={22} />
                </button>

                <button
                  type="button"
                  class="btn accent icon fs-play-btn"
                  class:squircle={theme.playButtonShape === 'squircle'}
                  title={s.isPlaying ? 'Duraklat' : 'Oynat'}
                  aria-label={s.isPlaying ? 'Duraklat' : 'Oynat'}
                  onclick={player.toggle}
                >
                  <Icon name={s.isPlaying ? 'pause' : 'play'} size={28} />
                </button>

                <button
                  type="button"
                  class="btn icon fs-skip-btn"
                  title="Sonraki"
                  aria-label="Sonraki"
                  onclick={player.next}
                >
                  <Icon name="next" size={22} />
                </button>

                <button
                  type="button"
                  class="btn ghost icon fs-side-ctrl"
                  class:on={s.shuffle}
                  title={s.shuffle ? 'Karışık çal: açık' : 'Karışık çal: kapalı'}
                  aria-label={s.shuffle ? 'Karışık çal: açık' : 'Karışık çal: kapalı'}
                  onclick={player.toggleShuffle}
                >
                  <Icon name="shuffle" size={21} />
                </button>
              </div>

              <div class="fs-volume">
                <button
                  type="button"
                  class="btn ghost icon"
                  title="Sessiz"
                  aria-label="Sessiz"
                  onclick={toggleMute}
                >
                  <Icon name={volumeIcon} size={20} />
                </button>
                <div class="fs-vol-slider">
                  <Slider
                    label="Ses seviyesi"
                    value={s.volume * 100}
                    max={100}
                    onpreview={(v) => player.setVolume(v / 100)}
                    oncommit={(v) => player.setVolume(v / 100)}
                  />
                </div>
                <span class="fs-vol-pct muted">{Math.round(s.volume * 100)}%</span>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section class="card fs-right" aria-label={rightTab === 'lyrics' ? 'Şarkı Sözleri' : 'Sıradaki Şarkılar'}>
        {#if rightTab === 'lyrics'}
          <LyricsPanel embedded={true} />
        {:else}
          <Queue threeCol={true} />
        {/if}
      </section>
    </main>
  {:else}
    <div class="fs-empty">
      <div class="card fs-empty-card">
        <Icon name="disc" size={44} />
        <h2>Şu an çalan şarkı yok</h2>
        <p class="muted">Tam ekran dinleme modunu kullanmak için kuyruktan veya ana sayfadan bir parça başlat.</p>
        <button type="button" class="btn accent" onclick={() => player.setFullscreen(false)}>
          Ana Sayfaya Dön
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .fs-root {
    position: fixed;
    inset: 0;
    z-index: 200;
    display: grid;
    grid-template-rows: auto minmax(0, 1fr);
    background: var(--bg);
    color: var(--text);
    padding: 18px 28px 24px;
    gap: 0;
    overflow: hidden;
  }

  .fs-root.is-idle {
    cursor: none;
  }

  .fs-top-collapse {
    display: grid;
    grid-template-rows: 1fr;
    margin-bottom: 12px;
    opacity: 1;
    transform: translateY(0);
    transition:
      grid-template-rows 360ms cubic-bezier(0.22, 1, 0.36, 1),
      margin-bottom 360ms cubic-bezier(0.22, 1, 0.36, 1),
      opacity 220ms ease,
      transform 340ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .fs-top-inner {
    min-height: 0;
    overflow: hidden;
  }

  .fs-root.is-idle .fs-top-collapse {
    grid-template-rows: 0fr;
    margin-bottom: 0;
    opacity: 0;
    transform: translateY(-10px);
    pointer-events: none;
    transition:
      grid-template-rows 460ms cubic-bezier(0.22, 1, 0.36, 1),
      margin-bottom 460ms cubic-bezier(0.22, 1, 0.36, 1),
      opacity 300ms ease,
      transform 420ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .fs-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 14px;
    max-width: 1420px;
    width: 100%;
    margin: 0 auto;
    padding: 2px 4px 6px;
    flex-shrink: 0;
  }

  .fs-source-group {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .fs-pill {
    padding: 6px 12px;
    border-radius: 999px;
    background: var(--surface);
    border: 2px solid var(--border);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    font-size: 12.5px;
    font-weight: 800;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 280px;
  }

  .fs-count {
    padding: 5px 10px;
    border-radius: 999px;
    background: var(--surface-2);
    border: 2px solid var(--border);
    font-size: 12px;
    font-weight: 800;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }

  .fs-top-actions {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  .fs-tabs {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px;
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .fs-tab {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    height: 34px;
    padding: 0 12px;
    border-radius: calc(var(--radius) - 4px);
    border: 2px solid transparent;
    background: transparent;
    color: var(--text-muted);
    font-family: inherit;
    font-size: 13px;
    font-weight: 800;
    cursor: pointer;
    transition: background-color 120ms ease, color 120ms ease;
  }

  .fs-tab:hover {
    background: var(--surface-2);
    color: var(--text);
  }

  .fs-tab.active {
    background: var(--accent);
    border-color: var(--accent-edge);
    color: var(--on-accent);
  }

  .fs-tab-count {
    padding: 1px 6px;
    border-radius: 6px;
    background: rgb(0 0 0 / 0.18);
    font-size: 11px;
    font-weight: 800;
  }

  .fs-hint {
    font-size: 12px;
    font-weight: 700;
    padding: 0 4px;
  }

  .fs-exit {
    gap: 7px;
    min-height: 40px;
    padding: 0 14px;
    font-size: 13px;
  }

  .fs-stage {
    max-width: 1420px;
    width: 100%;
    height: 100%;
    margin: 0 auto;
    display: grid;
    grid-template-columns: minmax(330px, 0.95fr) minmax(380px, 1.05fr);
    gap: 28px;
    min-height: 0;
    align-items: stretch;
  }

  .fs-left {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 0;
    min-height: 0;
    padding: 8px 12px;
  }

  .fs-cover-frame {
    padding: 10px;
    border-radius: 22px;
    background: var(--surface);
    border: 2px solid var(--border);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    transform: scale(1);
    transform-origin: center;
    transition: transform 380ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .fs-root.is-idle .fs-cover-frame {
    transform: scale(1.12);
    transition: transform 480ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .fs-cover-wrap {
    width: min(360px, 34vw, 39dvh);
  }

  .fs-dock-collapse {
    display: grid;
    grid-template-rows: 1fr;
    width: 100%;
    margin-top: 14px;
    opacity: 1;
    transform: translateY(0);
    transition:
      grid-template-rows 360ms cubic-bezier(0.22, 1, 0.36, 1),
      margin-top 360ms cubic-bezier(0.22, 1, 0.36, 1),
      opacity 220ms ease,
      transform 340ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .fs-dock-inner {
    min-height: 0;
    overflow: hidden;
  }

  .fs-dock-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    width: 100%;
    padding: 2px 6px 8px;
  }

  .fs-root.is-idle .fs-dock-collapse {
    grid-template-rows: 0fr;
    margin-top: 0;
    opacity: 0;
    transform: translateY(14px);
    pointer-events: none;
    transition:
      grid-template-rows 460ms cubic-bezier(0.22, 1, 0.36, 1),
      margin-top 460ms cubic-bezier(0.22, 1, 0.36, 1),
      opacity 300ms ease,
      transform 420ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .fs-meta {
    width: 100%;
    max-width: 520px;
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .fs-title {
    margin: 0;
    font-size: clamp(22px, 2.4vw, 32px);
    font-weight: 800;
    line-height: 1.15;
    letter-spacing: -0.02em;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .fs-sub-row {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    flex-wrap: wrap;
    min-width: 0;
  }

  .fs-artist {
    background: none;
    border: none;
    padding: 2px 6px;
    border-radius: 8px;
    font-family: inherit;
    font-size: 16px;
    font-weight: 700;
    color: var(--accent);
    cursor: pointer;
    transition: background-color 120ms ease;
  }

  .fs-artist:hover {
    background: var(--accent-soft);
    text-decoration: underline;
  }

  .fs-dot,
  .fs-album {
    color: var(--text-muted);
    font-size: 14px;
    font-weight: 600;
    max-width: 240px;
  }

  .fs-actions {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    flex-wrap: wrap;
  }

  .fs-action-btn {
    min-height: 36px;
    padding: 0 12px;
    font-size: 12.5px;
    gap: 6px;
    border-radius: 999px;
  }

  .fs-progress {
    width: 100%;
    max-width: 500px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .fs-times {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12px;
    font-weight: 700;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }

  .fs-time-btn {
    background: none;
    border: none;
    padding: 0;
    color: inherit;
    font: inherit;
    cursor: pointer;
  }

  .fs-time-btn:hover {
    color: var(--text);
  }

  .fs-controls {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 14px;
  }

  .fs-side-ctrl.on {
    color: var(--accent);
  }

  .fs-skip-btn {
    width: 46px;
    height: 46px;
    min-height: 46px;
  }

  .fs-play-btn {
    width: 62px;
    height: 62px;
    min-height: 62px;
    border-radius: 50%;
  }

  .fs-play-btn.squircle {
    border-radius: 18px;
  }

  .fs-volume {
    width: 100%;
    max-width: 320px;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .fs-vol-slider {
    flex: 1;
    min-width: 0;
  }

  .fs-vol-pct {
    min-width: 38px;
    text-align: right;
    font-size: 12px;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
  }

  .fs-right {
    display: flex;
    flex-direction: column;
    min-height: 0;
    min-width: 0;
    overflow: hidden;
    background: var(--surface);
  }

  .fs-right :global(.queue) {
    flex: 1;
    height: 100%;
    border: none;
    box-shadow: none;
    background: transparent;
  }

  .fs-right :global(.lyric-line) {
    font-size: clamp(16px, 1.45vw, 21px);
    padding: 12px 16px;
  }

  .fs-empty {
    display: grid;
    place-items: center;
    height: 100%;
  }

  .fs-empty-card {
    max-width: 420px;
    padding: 32px 28px;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 12px;
    color: var(--text-muted);
  }

  .fs-empty-card h2 {
    margin: 0;
    color: var(--text);
  }

  @media (max-width: 900px) {
    .fs-root {
      padding: 14px 16px 22px;
      overflow-y: auto;
    }

    .fs-hint {
      display: none;
    }

    .fs-stage {
      grid-template-columns: 1fr;
      grid-template-rows: auto minmax(380px, 1fr);
      height: auto;
      gap: 20px;
    }

    .fs-cover-wrap {
      width: min(260px, 66vw);
    }

    .fs-root.is-idle .fs-cover-frame {
      transform: scale(1.06);
    }

    .fs-right {
      min-height: 400px;
      max-height: 62dvh;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .fs-top-collapse,
    .fs-dock-collapse,
    .fs-cover-frame {
      transition-duration: 140ms;
    }
  }

  @media (max-width: 540px) {
    .fs-exit-label {
      display: none;
    }

    .fs-exit {
      width: 40px;
      padding: 0;
    }

    .fs-pill {
      max-width: 145px;
    }

    .fs-tab {
      padding: 0 10px;
      font-size: 12px;
    }
  }
</style>
