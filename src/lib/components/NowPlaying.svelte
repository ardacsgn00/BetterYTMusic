<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import Cover from './Cover.svelte';
  import Icon from './Icon.svelte';

  const track = $derived(player.state.track);
  const sourceName = $derived(player.state.sourceName);
</script>

<section class="now card">
  <div class="cover-wrap">
    <Cover {track} radius="var(--radius-lg)" />
  </div>

  <div class="meta">
    {#if track}
      <div class="badges">
        <span class="eyebrow">{player.state.isPlaying ? 'Çalıyor' : 'Duraklatıldı'}</span>
        {#if sourceName}
          <span class="source-badge truncate" title={`Çalınan kaynak: ${sourceName}`}>
            <Icon name="library" size={12} />
            <span>{sourceName}</span>
          </span>
        {/if}
      </div>
      <h1 class="title">{track.title}</h1>
      <p class="artist">
        <button
          type="button"
          class="artist-btn"
          title="{track.artist} Sanatçı Sayfasını Aç"
          onclick={() => player.openArtist(track.artist)}
        >
          {track.artist}
        </button>
      </p>
      <p class="muted">
        {track.album ? `${track.album} · ` : ''}{formatTime(track.durationMs)}
      </p>
      <div class="now-actions">
        <button
          class="btn now-action-btn"
          title="Bu şarkıyla benzer parçalardan radyo başlat"
          onclick={() => player.startRadio(track)}
        >
          <Icon name="radio" size={15} />
          <span>Şarkı Radyosu</span>
        </button>
        <button
          class="btn now-action-btn"
          title="{track.artist} Sanatçı Sayfasını Aç"
          onclick={() => player.openArtist(track.artist)}
        >
          <Icon name="user" size={15} />
          <span>Sanatçı Profili</span>
        </button>
      </div>
    {:else}
      <h1 class="title">Kuyruk boş</h1>
      <p class="muted">Çalmak için bir şarkı seç.</p>
    {/if}
  </div>
</section>

<style>
  .now {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 22px;
    padding: 24px;
    min-height: 0;
    min-width: 0;
    container-type: inline-size;
  }
  .cover-wrap {
    /* Scales smoothly with both panel width (when resized via splitter) and window height. */
    width: min(88cqi, 52vh, 500px);
    border-radius: var(--radius-lg);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }
  .meta {
    width: min(100%, 460px);
    display: flex;
    flex-direction: column;
    gap: 4px;
    text-align: center;
  }
  .badges {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 6px;
  }
  .eyebrow {
    font-size: 11px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    padding: 3px 9px;
    border-radius: 6px;
    background: var(--accent-soft);
    color: var(--accent);
  }
  .source-badge {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    max-width: 260px;
    font-size: 11.5px;
    font-weight: 800;
    padding: 3px 10px;
    border-radius: 6px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    color: var(--text);
  }
  .title {
    font-size: clamp(20px, 2.6vw, 30px);
    font-weight: 800;
    line-height: 1.15;
    overflow-wrap: anywhere;
  }
  .artist {
    font-size: 16px;
    font-weight: 600;
  }
  .artist-btn {
    border: none;
    background: transparent;
    padding: 0;
    font: inherit;
    color: inherit;
    cursor: pointer;
    text-decoration: none;
    transition: color 100ms ease;
  }
  .artist-btn:hover {
    color: var(--accent);
    text-decoration: underline;
  }
  .now-actions {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    margin-top: 10px;
    flex-wrap: wrap;
  }
  .now-action-btn {
    font-size: 12.5px;
    padding: 6px 12px;
    gap: 6px;
  }

  @media (max-width: 760px) {
    .now {
      flex-direction: row;
      padding: 14px;
      gap: 14px;
    }
    .cover-wrap {
      width: 96px;
      box-shadow: 0 var(--edge-depth) 0 var(--edge);
    }
    .meta {
      text-align: left;
      min-width: 0;
    }
    .badges {
      justify-content: flex-start;
    }
  }
</style>
