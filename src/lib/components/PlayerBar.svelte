<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { library } from '#lib/library.svelte.ts';
  import { theme } from '#lib/theme.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import Cover from './Cover.svelte';
  import Icon from './Icon.svelte';
  import Slider from './Slider.svelte';

  const s = $derived(player.state);
  const liked = $derived(library.isFavorite(s.track?.id));
  const endTimeLabel = $derived.by(() => {
    if (theme.showRemainingTime && s.durationMs > 0) {
      return `-${formatTime(Math.max(0, s.durationMs - player.position))}`;
    }
    return formatTime(s.durationMs);
  });

  // Remember volume before muting so unmute restores it.
  let lastVolume = 0.8;
  function toggleMute() {
    if (s.volume > 0) {
      lastVolume = s.volume;
      player.setVolume(0);
    } else {
      player.setVolume(lastVolume || 0.8);
    }
  }

  const volumeIcon = $derived(s.volume === 0 ? 'mute' : s.volume < 0.5 ? 'volume-low' : 'volume');
  const repeatLabel = $derived(
    s.repeat === 'one' ? 'Tekrar: tek şarkı' : s.repeat === 'all' ? 'Tekrar: tüm kuyruk' : 'Tekrar: kapalı'
  );
</script>

<footer class="bar">
  <div class="info">
    <Cover track={s.track} size="56px" radius="var(--radius-sm)" />
    <div class="text">
      <span class="title truncate">{s.track?.title ?? '—'}</span>
      <span class="artist truncate muted">{s.track?.artist ?? ''}</span>
    </div>
    {#if s.track}
      <button
        class="btn ghost icon fav-btn"
        class:on={liked}
        title={liked ? 'Kitaplıktan çıkar' : 'Kitaplığa ekle'}
        aria-label={liked ? 'Kitaplıktan çıkar' : 'Kitaplığa ekle'}
        onclick={() => library.toggle(s.track)}
      >
        <Icon name={liked ? 'heart-fill' : 'heart'} size={18} />
      </button>
    {/if}
  </div>

  <div class="center">
    <div class="controls">
      <button
        class="btn ghost icon repeat"
        class:on={s.repeat !== 'off'}
        title={repeatLabel}
        aria-label={repeatLabel}
        onclick={player.cycleRepeat}
      >
        <Icon name={s.repeat === 'one' ? 'repeat-one' : 'repeat'} size={20} />
      </button>
      <button class="btn icon" title="Önceki" aria-label="Önceki" onclick={player.previous}>
        <Icon name="prev" />
      </button>
      <button
        class="btn accent icon play"
        class:is-play={!s.isPlaying}
        class:squircle={theme.playButtonShape === 'squircle'}
        title={s.isPlaying ? 'Duraklat' : 'Oynat'}
        aria-label={s.isPlaying ? 'Duraklat' : 'Oynat'}
        onclick={player.toggle}
      >
        <Icon name={s.isPlaying ? 'pause' : 'play'} size={24} />
      </button>
      <button class="btn icon" title="Sonraki" aria-label="Sonraki" onclick={player.next}>
        <Icon name="next" />
      </button>
      <button
        class="btn ghost icon shuffle"
        class:on={s.shuffle}
        title={s.shuffle ? 'Karışık çal: açık' : 'Karışık çal: kapalı'}
        aria-label={s.shuffle ? 'Karışık çal: açık' : 'Karışık çal: kapalı'}
        onclick={player.toggleShuffle}
      >
        <Icon name="shuffle" size={20} />
      </button>
    </div>

    <div class="progress">
      <span class="time">{formatTime(player.position)}</span>
      <Slider
        label="Şarkı konumu"
        value={player.position}
        max={s.durationMs}
        step={250}
        oncommit={(v) => player.seek(v)}
      />
      <button
        type="button"
        class="time time-btn"
        title={theme.showRemainingTime ? 'Toplam süreyi göster' : 'Kalan süreyi göster'}
        onclick={() => theme.toggleRemainingTime()}
      >
        {endTimeLabel}
      </button>
    </div>
  </div>

  <div class="volume">
    <button class="btn ghost icon" title="Sessiz" aria-label="Sessiz" onclick={toggleMute}>
      <Icon name={volumeIcon} size={20} />
    </button>
    <div class="vol-slider">
      <Slider
        label="Ses seviyesi"
        value={s.volume * 100}
        max={100}
        onpreview={(v) => player.setVolume(v / 100)}
        oncommit={(v) => player.setVolume(v / 100)}
      />
    </div>
  </div>
</footer>

<style>
  .bar {
    grid-area: bar;
    display: grid;
    grid-template-columns: minmax(200px, 1fr) minmax(340px, 1.6fr) minmax(200px, 1fr);
    align-items: center;
    gap: 24px;
    padding: 12px 22px 14px;
    background: var(--surface);
    border-top: 2px solid var(--border);
  }
  .info {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
  }
  .text {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .title {
    font-weight: 700;
    font-size: 15px;
  }
  .artist {
    font-size: 13px;
  }
  .center {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-width: 0;
  }
  .controls {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
  }
  .play {
    width: 54px;
    height: 54px;
    border-radius: 50%;
  }
  .play.squircle {
    border-radius: var(--radius);
  }
  .play.is-play :global(svg) {
    transform: translateX(-1px);
  }
  .progress {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .time {
    min-width: 44px;
    font-size: 12px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
    text-align: center;
  }
  .time-btn {
    background: transparent;
    border: 0;
    padding: 2px 4px;
    border-radius: 6px;
    cursor: pointer;
    font: inherit;
    font-size: 12px;
    font-variant-numeric: tabular-nums;
    color: var(--text-muted);
    transition:
      color 120ms ease,
      background-color 120ms ease;
  }
  .time-btn:hover {
    color: var(--text);
    background: var(--surface-2);
  }
  .volume {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 8px;
  }
  .vol-slider {
    width: min(140px, 100%);
  }

  @media (max-width: 1000px) {
    .bar {
      grid-template-columns: minmax(160px, 1fr) minmax(300px, 2fr) auto;
      gap: 16px;
    }
    .vol-slider {
      display: none;
    }
  }

  /* Small windows: info + controls on one row, progress spans full width below. */
  @media (max-width: 760px) {
    .bar {
      grid-template-columns: 1fr auto;
      grid-template-areas:
        'info controls'
        'progress progress';
      gap: 8px 12px;
      padding: 10px 12px 12px;
    }
    .info {
      grid-area: info;
    }
    .center {
      display: contents;
    }
    .controls {
      grid-area: controls;
      gap: 8px;
    }
    .progress {
      grid-area: progress;
    }
    .volume,
    .repeat,
    .shuffle {
      display: none;
    }
    .info :global(.cover) {
      width: 44px !important;
    }
    .play {
      width: 48px;
      height: 48px;
    }
  }
</style>
