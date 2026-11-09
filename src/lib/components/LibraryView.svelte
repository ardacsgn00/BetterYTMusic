<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { library } from '#lib/library.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import type { Track } from '#lib/types.ts';
  import Cover from './Cover.svelte';
  import Icon from './Icon.svelte';

  let ytLiked = $state<Track[]>([]);
  let loadingYt = $state(false);

  $effect(() => {
    if (player.auth.loggedIn) {
      loadingYt = true;
      player
        .likedTracks()
        .then((res) => (ytLiked = res))
        .catch(() => {})
        .finally(() => (loadingYt = false));
    } else {
      ytLiked = [];
    }
  });

  const combined = $derived.by(() => {
    const out: Track[] = [...library.favorites];
    for (const t of ytLiked) {
      if (!out.some((x) => x.id === t.id)) out.push(t);
    }
    return out;
  });
</script>

<section class="view">
  <header class="head">
    <div>
      <h1>Kitaplık</h1>
      <p class="muted">
        {#if player.auth.loggedIn}
          YouTube Music hesabın ve yerel beğenilerin ({combined.length} şarkı)
        {:else}
          Beğendiğin şarkılar ({combined.length}) — YouTube Music kitaplığın için sol alttan Google ile bağlanabilirsin.
        {/if}
      </p>
    </div>
    {#if combined.length > 0}
      <button class="btn accent" onclick={() => player.playTrack(combined[0], combined, 'Kitaplık')}>
        <Icon name="play" size={18} />
        Tümünü Çal
      </button>
    {/if}
  </header>

  {#if loadingYt && combined.length === 0}
    <div class="card empty">
      <p class="muted">YouTube Music kitaplığın yükleniyor...</p>
    </div>
  {:else if combined.length === 0}
    <div class="card empty">
      <p class="muted">Henüz kitaplığına şarkı eklemedin. Arama veya Ana Sayfa'dan kalp ikonuna basarak ekleyebilir ya da Google hesabını bağlayabilirsin.</p>
    </div>
  {:else}
    <div class="card list">
      <ol>
        {#each combined as track (track.id)}
          {@const active = player.state.track?.id === track.id}
          {@const liked = library.isFavorite(track.id)}
          <li class="row" class:active>
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="main-hit" onclick={() => player.playTrack(track, combined, 'Kitaplık')}>
              <Cover {track} size="48px" radius="var(--radius-sm)" />
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
              <span class="dur muted">{track.durationMs ? formatTime(track.durationMs) : ''}</span>
            </div>
            <button
              class="btn ghost icon"
              title="Şarkı Radyosu Başlat"
              aria-label="Şarkı Radyosu Başlat"
              onclick={() => player.startRadio(track)}
            >
              <Icon name="radio" size={17} />
            </button>
            <button
              class="btn ghost icon"
              class:on={liked}
              title={liked ? 'Yerel favorilerden çıkar' : 'Yerel favorilere ekle'}
              aria-label={liked ? 'Yerel favorilerden çıkar' : 'Yerel favorilere ekle'}
              onclick={() => library.toggle(track)}
            >
              <Icon name={liked ? 'heart-fill' : 'heart'} size={18} />
            </button>
          </li>
        {/each}
      </ol>
    </div>
  {/if}
</section>

<style>
  .view {
    display: flex;
    flex-direction: column;
    gap: 18px;
    width: 100%;
    max-width: 1100px;
  }
  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }
  h1 {
    font-size: 26px;
    font-weight: 800;
  }
  .empty {
    padding: 24px;
  }
  .list {
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
