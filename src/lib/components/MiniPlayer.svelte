<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { theme } from '#lib/theme.svelte.ts';
  import { mini } from '#lib/mini.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import Cover from './Cover.svelte';
  import Icon from './Icon.svelte';

  const s = $derived(player.state);
  const square = $derived(theme.miniLayout === 'square');
  const progressPct = $derived(s.durationMs > 0 ? Math.min(100, (player.position / s.durationMs) * 100) : 0);
  const repeatLabel = $derived(
    s.repeat === 'one' ? 'Tekrar: tek şarkı' : s.repeat === 'all' ? 'Tekrar: tüm kuyruk' : 'Tekrar: kapalı'
  );

  // Brief volume badge shown while scrolling the mouse wheel.
  let volBadge = $state<number | null>(null);
  let volTimer: ReturnType<typeof setTimeout> | undefined;

  function onWheel(e: WheelEvent) {
    e.preventDefault();
    const step = e.deltaY < 0 ? 0.05 : -0.05;
    const next = Math.max(0, Math.min(1, Math.round((s.volume + step) * 100) / 100));
    player.setVolume(next);
    volBadge = Math.round(next * 100);
    clearTimeout(volTimer);
    volTimer = setTimeout(() => (volBadge = null), 900);
  }

  // Drag the window from any empty area (buttons stay clickable).
  function onPointerDown(e: PointerEvent) {
    if (e.button !== 0) return;
    if ((e.target as HTMLElement).closest('button')) return;
    mini.startDrag();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="mini"
  class:square
  onpointerdown={onPointerDown}
  onwheel={{ handleEvent: onWheel, passive: false } as any}
>
  {#if square}
    <div class="sq-bg">
      <Cover track={s.track} size="100%" radius="0" />
    </div>
  {/if}

  <div class="tools">
    <button
      class="btn ghost icon tool"
      class:on={s.shuffle}
      title={s.shuffle ? 'Karışık çal: açık' : 'Karışık çal: kapalı'}
      aria-label="Karışık çal"
      onclick={player.toggleShuffle}
    >
      <Icon name="shuffle" size={15} />
    </button>
    <button
      class="btn ghost icon tool"
      class:on={s.repeat !== 'off'}
      title={repeatLabel}
      aria-label={repeatLabel}
      onclick={player.cycleRepeat}
    >
      <Icon name={s.repeat === 'one' ? 'repeat-one' : 'repeat'} size={15} />
    </button>
    <button
      class="btn ghost icon tool"
      class:on={mini.onTop}
      title={mini.onTop ? 'Her zaman üstte: açık' : 'Her zaman üstte: kapalı'}
      aria-label="Her zaman üstte"
      onclick={mini.toggleOnTop}
    >
      <Icon name="pin" size={15} />
    </button>
    <button class="btn ghost icon tool" title="Büyüt" aria-label="Büyüt" onclick={mini.exit}>
      <Icon name="maximize" size={15} />
    </button>
  </div>

  <div class="body">
    {#if !square}
      <Cover track={s.track} size="72px" radius="var(--radius-sm)" />
    {/if}

    <div class="meta">
      <span class="title truncate" title={s.track?.title ?? ''}>{s.track?.title ?? 'Şarkı seçilmedi'}</span>
      <span class="artist truncate">{s.track?.artist ?? ''}</span>
      {#if square}
        <span class="times">
          {formatTime(player.position)} / {formatTime(s.durationMs)}
        </span>
      {/if}
    </div>

    <div class="controls">
      <button class="btn icon ctrl" title="Önceki" aria-label="Önceki" onclick={player.previous}>
        <Icon name="prev" size={18} />
      </button>
      <button
        class="btn accent icon ctrl play"
        class:squircle={theme.playButtonShape === 'squircle'}
        title={s.isPlaying ? 'Duraklat' : 'Oynat'}
        aria-label={s.isPlaying ? 'Duraklat' : 'Oynat'}
        onclick={player.toggle}
      >
        <Icon name={s.isPlaying ? 'pause' : 'play'} size={20} />
      </button>
      <button class="btn icon ctrl" title="Sonraki" aria-label="Sonraki" onclick={player.next}>
        <Icon name="next" size={18} />
      </button>
    </div>
  </div>

  <div class="progress" aria-hidden="true">
    <div class="fill" style:width={`${progressPct}%`}></div>
  </div>

  {#if volBadge !== null}
    <div class="vol-badge">
      <Icon name={volBadge === 0 ? 'mute' : volBadge < 50 ? 'volume-low' : 'volume'} size={14} />
      %{volBadge}
    </div>
  {/if}
</div>

<style>
  .mini {
    position: fixed;
    inset: 0;
    overflow: hidden;
    background: var(--surface);
    border: 2px solid var(--border);
    color: var(--text);
    user-select: none;
    cursor: grab;
    display: flex;
    flex-direction: column;
  }
  .mini:active {
    cursor: grabbing;
  }
  .mini button {
    cursor: pointer;
  }

  /* ---------- Strip layout (380x108) ---------- */
  .body {
    flex: 1;
    min-height: 0;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 12px 4px 14px;
  }
  .meta {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .title {
    font-weight: 800;
    font-size: 14px;
    line-height: 1.25;
  }
  .artist {
    font-size: 12px;
    color: var(--text-muted);
  }
  .controls {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }
  .ctrl {
    width: 34px;
    height: 34px;
    min-height: 34px;
  }
  .play {
    width: 42px;
    height: 42px;
    min-height: 42px;
    border-radius: 50%;
  }
  .play.squircle {
    border-radius: var(--radius);
  }

  /* Hover tools: revealed on hover only to keep the widget calm. */
  .tools {
    position: absolute;
    top: 4px;
    right: 6px;
    z-index: 3;
    display: flex;
    gap: 2px;
    padding: 2px;
    border-radius: var(--radius-sm);
    background: var(--surface);
    opacity: 0;
    transform: translateY(-4px);
    pointer-events: none;
    transition:
      opacity 120ms ease,
      transform 120ms ease;
  }
  .mini:hover .tools {
    opacity: 1;
    transform: none;
    pointer-events: auto;
  }
  .tool {
    width: 26px;
    height: 26px;
    min-height: 26px;
    color: var(--text-muted);
  }
  .tool:hover {
    color: var(--text);
  }
  .tool.on {
    color: var(--accent);
  }

  .progress {
    height: 4px;
    background: var(--surface-3);
    flex-shrink: 0;
    position: relative;
    z-index: 2;
  }
  .fill {
    height: 100%;
    background: var(--accent);
  }

  .vol-badge {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    z-index: 5;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: var(--radius);
    background: var(--surface-2);
    border: 2px solid var(--border);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    font-weight: 800;
    font-size: 13px;
    font-variant-numeric: tabular-nums;
    pointer-events: none;
  }

  /* ---------- Square layout (300x300) ---------- */
  .sq-bg {
    position: absolute;
    inset: 0;
    z-index: 0;
  }
  .sq-bg :global(.cover) {
    width: 100% !important;
    height: 100%;
  }
  .square .body {
    position: relative;
    z-index: 2;
    flex: none;
    margin-top: auto;
    /* Solid translucent panel keeps text legible over any cover (no gradient). */
    background: rgb(12 12 14 / 0.8);
    border-top: 2px solid rgb(255 255 255 / 0.08);
    flex-direction: column;
    align-items: stretch;
    justify-content: flex-end;
    gap: 10px;
    padding: 14px 16px 14px;
  }
  .square .title {
    color: #fff;
    font-size: 16px;
  }
  .square .artist {
    color: rgb(255 255 255 / 0.78);
    font-size: 12.5px;
  }
  .times {
    margin-top: 2px;
    font-size: 11px;
    color: rgb(255 255 255 / 0.7);
    font-variant-numeric: tabular-nums;
  }
  .square .controls {
    justify-content: center;
    gap: 14px;
  }
  .square .ctrl {
    width: 40px;
    height: 40px;
    min-height: 40px;
  }
  .square .play {
    width: 52px;
    height: 52px;
    min-height: 52px;
  }
  .square .tools {
    top: 8px;
    right: 8px;
  }
</style>
