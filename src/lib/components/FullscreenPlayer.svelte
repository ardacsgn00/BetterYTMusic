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

  const s = $derived(player.state);
  const track = $derived(s.track);
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

<div class="fs-root" role="dialog" aria-label="Tam Ekran Oynatıcı">
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

  {#if track}
    <main class="fs-stage">
      <section class="fs-left">
        <div class="fs-cover-frame">
          <div class="fs-cover-wrap">
            <Cover {track} size="100%" radius="16px" />
          </div>
        </div>

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
    gap: 18px;
    overflow: hidden;
  }

  .fs-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 14px;
    max-width: 1420px;
    width: 100%;
    margin: 0 auto;
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
    gap: 16px;
    min-height: 0;
    padding: 8px 12px;
  }

  .fs-cover-frame {
    padding: 10px;
    border-radius: 22px;
    background: var(--surface);
    border: 2px solid var(--border);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }

  .fs-cover-wrap {
    width: min(360px, 34vw, 39dvh);
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

    .fs-right {
      min-height: 400px;
      max-height: 62dvh;
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
