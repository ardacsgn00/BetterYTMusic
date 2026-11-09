<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import Cover from './Cover.svelte';
  import Icon from './Icon.svelte';
  import LyricsPanel from './LyricsPanel.svelte';

  let {
    threeCol = false,
    ontogglecolumns
  }: {
    threeCol?: boolean;
    ontogglecolumns?: () => void;
  } = $props();

  let tab = $state<'queue' | 'lyrics'>('queue');

  const queue = $derived(player.state.queue);
  const current = $derived(player.state.index);
  const sourceName = $derived(player.state.sourceName);
  const hasLyrics = $derived(Boolean(player.lyrics));
</script>

<section class="queue card">
  <header class="q-head">
    {#if threeCol}
      <div class="head-left">
        <h2>Sıradaki</h2>
        <span class="count-pill">{queue.length} şarkı</span>
        {#if sourceName}
          <span class="src-pill truncate" title={sourceName}>{sourceName}</span>
        {/if}
      </div>
    {:else}
      <div class="tabs" role="tablist">
        <button
          type="button"
          role="tab"
          class="tab-btn"
          class:active={tab === 'queue'}
          aria-selected={tab === 'queue'}
          onclick={() => (tab = 'queue')}
        >
          <span>Sıradaki</span>
          <span class="count-pill">{queue.length}</span>
        </button>
        <button
          type="button"
          role="tab"
          class="tab-btn"
          class:active={tab === 'lyrics'}
          aria-selected={tab === 'lyrics'}
          onclick={() => (tab = 'lyrics')}
        >
          <Icon name="lyrics" size={16} />
          <span>Şarkı Sözleri</span>
          {#if hasLyrics}
            <span class="lyr-dot" title="Şarkı sözleri hazır"></span>
          {/if}
        </button>
        {#if sourceName}
          <span class="src-pill truncate" title={`Çalınan kaynak: ${sourceName}`}>
            {sourceName}
          </span>
        {/if}
      </div>
    {/if}

    {#if ontogglecolumns}
      <button
        type="button"
        class="btn icon col-btn"
        class:accent={threeCol}
        title={threeCol ? 'Sekmeli görünüme dön (2 sütun)' : 'Şarkı sözlerini ve kuyruğu yan yana aç (3 sütun)'}
        aria-label="3 sütun görünümü"
        aria-pressed={threeCol}
        onclick={ontogglecolumns}
      >
        <Icon name="columns" size={17} />
      </button>
    {/if}
  </header>

  {#if !threeCol && tab === 'lyrics'}
    <LyricsPanel embedded={true} />
  {:else}
    <ol>
      {#each queue as track, i (`${track.id}-${i}`)}
        <li class="q-item" class:active={i === current}>
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div class="row" onclick={() => player.playIndex(i)}>
            <span class="idx">
              {#if i === current && player.state.isPlaying}
                <span class="eq" aria-label="Çalıyor"><i></i><i></i><i></i></span>
              {:else}
                {i + 1}
              {/if}
            </span>
            <Cover {track} size="44px" radius="var(--radius-sm)" />
            <span class="info">
              <span class="t truncate">{track.title}</span>
              <span class="a truncate muted">
                <button
                  type="button"
                  class="artist-inline-btn"
                  title="{track.artist} Sanatçı Sayfasını Aç"
                  onclick={(e) => {
                    e.stopPropagation();
                    player.openArtist(track.artist);
                  }}
                >
                  {track.artist}
                </button>
              </span>
            </span>
            <span class="dur muted">{formatTime(track.durationMs)}</span>
          </div>
          <button
            class="btn ghost icon q-radio-btn"
            title="Şarkı Radyosu Başlat"
            aria-label="Şarkı Radyosu Başlat"
            onclick={() => player.startRadio(track)}
          >
            <Icon name="radio" size={15} />
          </button>
        </li>
      {/each}
    </ol>
  {/if}
</section>

<style>
  .queue {
    display: flex;
    flex-direction: column;
    min-height: 0;
    min-width: 0;
    overflow: hidden;
  }
  .q-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding: 12px 14px 10px;
    border-bottom: 2px solid var(--border);
  }
  .head-left {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 0 4px;
  }
  h2 {
    font-size: 17px;
    font-weight: 800;
  }
  .tabs {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }
  .tab-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    height: 36px;
    padding: 0 12px;
    border-radius: var(--radius);
    border: 2px solid transparent;
    background: transparent;
    color: var(--text-muted);
    font-weight: 700;
    font-size: 13.5px;
    cursor: pointer;
    transition:
      background-color 120ms ease,
      color 120ms ease,
      transform 70ms ease;
  }
  .tab-btn:hover {
    background: var(--surface-2);
    color: var(--text);
  }
  .tab-btn.active {
    background: var(--accent-soft);
    border-color: var(--accent);
    color: var(--text);
    box-shadow: 0 var(--edge-depth) 0 var(--accent-edge);
  }
  .tab-btn:active {
    transform: translateY(1px);
  }
  .count-pill {
    padding: 1px 7px;
    border-radius: 6px;
    background: var(--surface-3);
    color: var(--text-muted);
    font-size: 11px;
    font-weight: 800;
    font-variant-numeric: tabular-nums;
  }
  .tab-btn.active .count-pill {
    background: var(--accent);
    color: var(--on-accent);
  }
  .lyr-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--accent);
  }
  .src-pill {
    max-width: 190px;
    padding: 3px 9px;
    border-radius: 6px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    color: var(--text-muted);
    font-size: 11.5px;
    font-weight: 700;
  }
  .col-btn {
    width: 36px;
    height: 36px;
    min-height: 36px;
    flex-shrink: 0;
  }
  ol {
    list-style: none;
    margin: 0;
    padding: 8px 10px 14px;
    overflow-y: auto;
    flex: 1;
  }
  .q-item {
    display: flex;
    align-items: center;
    gap: 4px;
    border-radius: var(--radius);
    padding-right: 6px;
    border: 2px solid transparent;
  }
  .q-item:hover {
    background: var(--surface-2);
  }
  .q-item.active {
    background: var(--accent-soft);
    border-color: var(--accent);
  }
  .q-item .row {
    flex: 1;
    min-width: 0;
    border: none;
    background: transparent;
  }
  .q-radio-btn {
    width: 32px;
    height: 32px;
    min-height: 32px;
    opacity: 0;
    transition: opacity 120ms ease;
  }
  .q-item:hover .q-radio-btn,
  .q-radio-btn:focus-visible {
    opacity: 1;
  }
  .artist-inline-btn {
    border: none;
    background: transparent;
    padding: 0;
    font: inherit;
    color: inherit;
    cursor: pointer;
    text-align: left;
    display: inline;
    transition: color 100ms ease;
  }
  .artist-inline-btn:hover {
    color: var(--accent);
    text-decoration: underline;
  }

  .row {
    width: 100%;
    display: grid;
    grid-template-columns: 28px 44px 1fr auto;
    align-items: center;
    gap: 12px;
    padding: 8px 10px;
    border: 2px solid transparent;
    border-radius: var(--radius);
    background: transparent;
    cursor: pointer;
    text-align: left;
  }
  .q-item.active .t {
    color: var(--accent);
  }
  .idx {
    text-align: center;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
    font-weight: 600;
  }
  .info {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .t {
    font-weight: 700;
  }
  .a {
    font-size: 13px;
  }
  .dur {
    font-variant-numeric: tabular-nums;
    font-size: 13px;
  }

  .eq {
    display: inline-flex;
    align-items: flex-end;
    gap: 2px;
    height: 14px;
  }
  .eq i {
    width: 3px;
    background: var(--accent);
    border-radius: 1px;
    animation: eq 0.9s ease-in-out infinite;
  }
  .eq i:nth-child(2) {
    animation-delay: -0.3s;
  }
  .eq i:nth-child(3) {
    animation-delay: -0.6s;
  }
  @keyframes eq {
    0%,
    100% {
      height: 4px;
    }
    50% {
      height: 14px;
    }
  }
  @media (prefers-reduced-motion: reduce) {
    .eq i {
      animation: none;
      height: 10px;
    }
  }
  @media (max-width: 1120px) {
    .col-btn {
      display: none;
    }
  }
</style>
