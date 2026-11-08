<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { library } from '#lib/library.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import type { Track } from '#lib/types.ts';
  import Cover from './Cover.svelte';
  import Icon from './Icon.svelte';

  let query = $state('');
  let results = $state<Track[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let searched = $state(false);

  async function doSearch(q = query) {
    const trimmed = q.trim();
    if (!trimmed) return;
    query = trimmed;
    loading = true;
    error = null;
    searched = true;
    try {
      results = await player.search(trimmed);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function clearSearch() {
    query = '';
    results = [];
    searched = false;
    error = null;
  }

  function onSubmit(e: SubmitEvent) {
    e.preventDefault();
    doSearch();
  }
</script>

<section class="view">
  <header class="head">
    <h1>YouTube Music'te Ara</h1>
    <form class="search-bar" onsubmit={onSubmit}>
      <span class="s-icon"><Icon name="search" size={18} /></span>
      <input
        type="text"
        placeholder="Şarkı, sanatçı veya albüm ara..."
        bind:value={query}
      />
      {#if query.length > 0}
        <button type="button" class="btn ghost icon clear-btn" title="Temizle" onclick={clearSearch}>
          <Icon name="close" size={16} />
        </button>
      {/if}
      <button type="submit" class="btn accent" disabled={loading || !query.trim()}>
        {loading ? 'Aranıyor...' : 'Ara'}
      </button>
    </form>
  </header>

  {#if error}
    <div class="card msg error">
      <p>{error}</p>
    </div>
  {:else if loading}
    <div class="card msg">
      <p class="muted">YouTube Music sonuçları getiriliyor...</p>
    </div>
  {:else if results.length > 0}
    <div class="card results">
      <ol>
        {#each results as track (track.id)}
          {@const active = player.state.track?.id === track.id}
          {@const liked = library.isFavorite(track.id)}
          <li class="row" class:active>
            <button class="main-hit" onclick={() => player.playTrack(track)}>
              <Cover {track} size="48px" radius="var(--radius-sm)" />
              <span class="info">
                <span class="t truncate">{track.title}</span>
                <span class="a truncate muted">
                  {track.artist}{track.album ? ` · ${track.album}` : ''}
                </span>
              </span>
              <span class="dur muted">{track.durationMs ? formatTime(track.durationMs) : ''}</span>
            </button>
            <button
              class="btn ghost icon fav"
              class:on={liked}
              title={liked ? 'Kitaplıktan çıkar' : 'Kitaplığa ekle'}
              aria-label={liked ? 'Kitaplıktan çıkar' : 'Kitaplığa ekle'}
              onclick={() => library.toggle(track)}
            >
              <Icon name={liked ? 'heart-fill' : 'heart'} size={18} />
            </button>
          </li>
        {/each}
      </ol>
    </div>
  {:else if searched}
    <div class="card msg">
      <p class="muted">Sonuç bulunamadı.</p>
    </div>
  {/if}
</section>

<style>
  .view {
    display: flex;
    flex-direction: column;
    gap: 18px;
    width: 100%;
    max-width: 1200px;
  }
  .head {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  h1 {
    font-size: 26px;
    font-weight: 800;
  }
  .search-bar {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px 8px 14px;
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius-lg);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }
  .s-icon {
    color: var(--text-muted);
    display: flex;
  }
  input {
    flex: 1;
    border: 0;
    background: transparent;
    color: var(--text);
    font: inherit;
    font-size: 15px;
    outline: none;
    min-width: 0;
  }
  .clear-btn {
    width: 32px;
    height: 32px;
    min-height: 32px;
    color: var(--text-muted);
  }
  .msg {
    padding: 24px;
  }
  .results {
    padding: 10px;
  }
  ol {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 6px;
    padding-right: 6px;
    border: 2px solid transparent;
    border-radius: var(--radius);
  }
  .row:hover {
    background: var(--surface-2);
  }
  .row.active {
    background: var(--accent-soft);
    border-color: var(--accent);
  }
  .main-hit {
    flex: 1;
    display: grid;
    grid-template-columns: 48px minmax(0, 1fr) auto;
    align-items: center;
    gap: 14px;
    padding: 8px 10px;
    border: 0;
    background: transparent;
    cursor: pointer;
    text-align: left;
    min-width: 0;
  }
  :global(:root[data-density='compact']) .main-hit {
    padding: 4px 8px;
  }
  .info {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .t {
    font-weight: 700;
    font-size: 15px;
  }
  .row.active .t {
    color: var(--accent);
  }
  .a {
    font-size: 13px;
  }
  .dur {
    font-variant-numeric: tabular-nums;
    font-size: 13px;
  }
</style>
