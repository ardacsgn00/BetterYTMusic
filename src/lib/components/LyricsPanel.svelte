<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import Icon from './Icon.svelte';

  let {
    embedded = false,
    ontogglecolumns,
    threeCol = false
  }: {
    embedded?: boolean;
    ontogglecolumns?: () => void;
    threeCol?: boolean;
  } = $props();

  let listEl = $state<HTMLElement | null>(null);

  const track = $derived(player.state.track);
  const data = $derived(player.lyrics);
  const loading = $derived(player.lyricsLoading);

  const activeIndex = $derived.by(() => {
    if (!data?.synced || !data.lines.length) return -1;
    const pos = player.position + 180;
    let idx = -1;
    for (let i = 0; i < data.lines.length; i++) {
      if (data.lines[i].timeMs <= pos) {
        idx = i;
      } else {
        break;
      }
    }
    return idx;
  });

  $effect(() => {
    const idx = activeIndex;
    if (idx < 0 || !listEl) return;
    const target = listEl.querySelector<HTMLElement>(`[data-line="${idx}"]`);
    if (target) {
      target.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }
  });
</script>

<div class="lyrics-wrap" class:card={!embedded}>
  {#if !embedded}
    <header class="lyr-head">
      <div class="title-group">
        <Icon name="lyrics" size={18} />
        <h2>Şarkı Sözleri</h2>
        {#if data}
          <span class="tag">{data.synced ? 'SENKRON' : 'METİN'}</span>
        {/if}
      </div>
      {#if ontogglecolumns}
        <button
          class="btn icon sm-btn accent"
          title="3 sütun görünümünü kapat"
          aria-label="3 sütun görünümünü kapat"
          aria-pressed={threeCol}
          onclick={ontogglecolumns}
        >
          <Icon name="columns" size={17} />
        </button>
      {/if}
    </header>
  {/if}

  <div class="lyr-body" bind:this={listEl}>
    {#if !track}
      <div class="state-box">
        <p class="muted">Şarkı sözlerini görmek için bir parça başlat.</p>
      </div>
    {:else if loading}
      <div class="state-box">
        <p class="muted">"{track.title}" için şarkı sözleri aranıyor...</p>
      </div>
    {:else if data?.synced && data.lines.length > 0}
      <div class="synced-list">
        {#each data.lines as line, i (i)}
          {@const isCurrent = i === activeIndex}
          {@const isPast = activeIndex > i}
          <button
            type="button"
            class="lyric-line"
            class:current={isCurrent}
            class:past={isPast}
            data-line={i}
            title={`${formatTime(line.timeMs)} konumuna atla`}
            onclick={() => player.seek(line.timeMs)}
          >
            <span class="line-text">{line.text}</span>
            <span class="line-time">{formatTime(line.timeMs)}</span>
          </button>
        {/each}
        <div class="source-foot muted">
          Kaynak: {data.source} · Satıra tıklayarak o saniyeye atlayabilirsin
        </div>
      </div>
    {:else if data?.plain}
      <div class="plain-list">
        {#each data.plain.split(/\r?\n/) as paragraph, i (i)}
          {#if paragraph.trim().length === 0}
            <div class="verse-gap"></div>
          {:else}
            <p class="plain-line">{paragraph}</p>
          {/if}
        {/each}
        <div class="source-foot muted">Kaynak: {data.source}</div>
      </div>
    {:else}
      <div class="state-box">
        <p class="muted">Bu parça için şarkı sözü bulunamadı.</p>
        <button class="btn" onclick={() => track && player.loadLyricsFor(track)}>
          <Icon name="refresh" size={16} />
          Tekrar Dene
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .lyrics-wrap {
    display: flex;
    flex-direction: column;
    min-height: 0;
    min-width: 0;
    height: 100%;
    overflow: hidden;
  }
  .lyr-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding: 14px 18px 10px;
    border-bottom: 2px solid var(--border);
  }
  .title-group {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  h2 {
    font-size: 17px;
    font-weight: 800;
  }
  .tag {
    padding: 2px 7px;
    border-radius: 6px;
    background: var(--accent-soft);
    color: var(--accent);
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 0.05em;
  }
  .sm-btn {
    width: 34px;
    height: 34px;
    min-height: 34px;
  }
  .lyr-body {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    min-height: 0;
  }
  .state-box {
    height: 100%;
    min-height: 220px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    text-align: center;
    padding: 24px;
  }
  .synced-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 18px 0 48px;
  }
  .lyric-line {
    width: 100%;
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 12px;
    padding: 10px 14px;
    border: 2px solid transparent;
    border-radius: var(--radius);
    background: transparent;
    color: var(--text-muted);
    font-size: 16px;
    font-weight: 700;
    line-height: 1.4;
    text-align: left;
    cursor: pointer;
    transition:
      background-color 120ms ease,
      color 120ms ease,
      border-color 120ms ease,
      transform 70ms ease;
  }
  .lyric-line:hover {
    background: var(--surface-2);
    color: var(--text);
  }
  .lyric-line:active {
    transform: translateY(1px);
  }
  .lyric-line.past {
    opacity: 0.58;
  }
  .lyric-line.current {
    background: var(--accent-soft);
    border-color: var(--accent);
    color: var(--text);
    font-size: 18px;
    font-weight: 800;
    box-shadow: 0 var(--edge-depth) 0 var(--accent-edge);
    opacity: 1;
  }
  .line-text {
    flex: 1;
    min-width: 0;
    overflow-wrap: anywhere;
  }
  .line-time {
    font-size: 11px;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    color: var(--text-muted);
    opacity: 0;
    flex-shrink: 0;
    transition: opacity 100ms ease;
  }
  .lyric-line:hover .line-time,
  .lyric-line.current .line-time {
    opacity: 1;
  }
  .lyric-line.current .line-time {
    color: var(--accent);
  }
  .plain-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 8px 6px 32px;
  }
  .plain-line {
    font-size: 15.5px;
    font-weight: 600;
    line-height: 1.55;
    color: var(--text);
  }
  .verse-gap {
    height: 14px;
  }
  .source-foot {
    margin-top: 18px;
    padding-top: 12px;
    border-top: 1px solid var(--border);
    font-size: 11.5px;
  }
</style>

