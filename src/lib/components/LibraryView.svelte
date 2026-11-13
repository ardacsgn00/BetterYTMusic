<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { library } from '#lib/library.svelte.ts';
  import { theme } from '#lib/theme.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import type { PlaylistCard, Track } from '#lib/types.ts';
  import Cover from './Cover.svelte';
  import Icon from './Icon.svelte';

  let ytLiked = $state<Track[]>([]);
  let loadingYt = $state(false);
  let ytPlaylists = $state<PlaylistCard[]>([]);
  let loadingYtPlaylists = $state(false);

  let libTab = $state<'all' | 'playlists' | 'songs'>('all');
  let openedPlaylist = $state<PlaylistCard | null>(null);
  let openedTracks = $state<Track[]>([]);
  let openingPlaylist = $state(false);
  let playingPlaylistId = $state<string | null>(null);

  $effect(() => {
    if (player.auth.loggedIn) {
      loadingYt = true;
      loadingYtPlaylists = true;
      player
        .likedTracks()
        .then((res) => (ytLiked = res))
        .catch(() => {})
        .finally(() => (loadingYt = false));
      player
        .libraryPlaylists()
        .then((res) => (ytPlaylists = res))
        .catch(() => {})
        .finally(() => (loadingYtPlaylists = false));
    } else {
      ytLiked = [];
      ytPlaylists = [];
    }
  });

  const combinedPlaylists = $derived.by(() => {
    const out: PlaylistCard[] = [...library.savedPlaylists];
    for (const pl of ytPlaylists) {
      if (!out.some((x) => x.playlistId === pl.playlistId)) out.push(pl);
    }
    return out;
  });

  const combined = $derived.by(() => {
    const out: Track[] = [...library.favorites];
    for (const t of ytLiked) {
      if (!out.some((x) => x.id === t.id)) out.push(t);
    }
    return out;
  });

  function playlistBadgeLabel(playlistId: string): string {
    if (playlistId.startsWith('RD')) return 'MİKS';
    if (playlistId.startsWith('MPREb_') || playlistId.startsWith('OLAK')) return 'ALBÜM';
    return 'LİSTE';
  }

  async function openPlaylistView(pl: PlaylistCard) {
    openedPlaylist = pl;
    openedTracks = [];
    openingPlaylist = true;
    try {
      openedTracks = await player.playlistTracks(pl.playlistId);
    } catch {
      openedTracks = [];
    } finally {
      openingPlaylist = false;
    }
  }

  async function playPlaylistDirect(e: MouseEvent, pl: PlaylistCard) {
    e.stopPropagation();
    if (playingPlaylistId) return;
    playingPlaylistId = pl.playlistId;
    try {
      await player.playPlaylist(pl.playlistId, pl.title);
    } finally {
      playingPlaylistId = null;
    }
  }

  function playOpenedAll() {
    if (!openedPlaylist || openedTracks.length === 0) return;
    player.playTrack(openedTracks[0], openedTracks, openedPlaylist.title);
  }

  function playOpenedShuffled() {
    if (!openedPlaylist || openedTracks.length === 0) return;
    const shuffled = [...openedTracks].sort(() => Math.random() - 0.5);
    player.playTrack(shuffled[0], shuffled, `${openedPlaylist.title} (Karışık)`);
  }
</script>

<section class="view" data-card-size={theme.cardSize}>
  {#if openedPlaylist}
    {@const plSaved = library.isPlaylistSaved(openedPlaylist.playlistId)}
    <div class="pl-topbar">
      <button type="button" class="btn" onclick={() => (openedPlaylist = null)}>
        <Icon name="prev" size={16} />
        Kitaplığa Dön
      </button>
    </div>

    <div class="card hero pl-hero">
      <div class="pl-hero-art" style:background-color={openedPlaylist.color}>
        {#if openedPlaylist.thumbnail}
          <img src={openedPlaylist.thumbnail} alt="" draggable="false" />
        {:else}
          <Icon name="now" size={44} />
        {/if}
      </div>
      <div class="hero-body">
        <span class="badge">{playlistBadgeLabel(openedPlaylist.playlistId)}</span>
        <h2 class="hero-title">{openedPlaylist.title}</h2>
        <p class="hero-meta muted">
          {openedPlaylist.subtitle}
          {#if !openingPlaylist && openedTracks.length > 0}
            · {openedTracks.length} şarkı
          {/if}
        </p>
        <div class="hero-btns">
          <button
            type="button"
            class="btn accent"
            disabled={openingPlaylist || openedTracks.length === 0}
            onclick={playOpenedAll}
          >
            <Icon name="play" size={18} />
            Tümünü Oynat
          </button>
          <button
            type="button"
            class="btn"
            disabled={openingPlaylist || openedTracks.length === 0}
            onclick={playOpenedShuffled}
          >
            <Icon name="shuffle" size={17} />
            Karışık Çal
          </button>
          <button
            type="button"
            class="btn"
            class:accent={plSaved}
            title={plSaved ? 'Kitaplıktan kaldır' : 'Yerel kitaplığa kaydet'}
            onclick={() => openedPlaylist && library.togglePlaylist(openedPlaylist)}
          >
            <Icon name={plSaved ? 'bookmark-fill' : 'bookmark'} size={17} />
            <span>{plSaved ? 'Kaydedildi' : 'Kitaplığa Kaydet'}</span>
          </button>
        </div>
      </div>
    </div>

    {#if openingPlaylist}
      <div class="card empty">
        <p class="muted">"{openedPlaylist.title}" içeriği yükleniyor...</p>
      </div>
    {:else if openedTracks.length === 0}
      <div class="card empty">
        <p class="muted">Bu listede oynatılabilir parça bulunamadı.</p>
      </div>
    {:else}
      <div class="card list">
        <ol>
          {#each openedTracks as track, idx (`${track.id}-${idx}`)}
            {@const active = player.state.track?.id === track.id}
            {@const liked = library.isFavorite(track.id)}
            <li class="row" class:active>
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div
                class="main-hit"
                onclick={() => player.playTrack(track, openedTracks, openedPlaylist?.title)}
              >
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
                    {#if track.album}
                      <span> · {track.album}</span>
                    {/if}
                  </span>
                </span>
                <span class="dur muted">{track.durationMs ? formatTime(track.durationMs) : ''}</span>
              </div>
              <button
                type="button"
                class="btn ghost icon"
                title="Şarkı Radyosu Başlat"
                aria-label="Şarkı Radyosu Başlat"
                onclick={() => player.startRadio(track)}
              >
                <Icon name="radio" size={17} />
              </button>
              <button
                type="button"
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
  {:else}
    <header class="head">
      <div>
        <h1>Kitaplık</h1>
        <p class="muted">
          {#if player.auth.loggedIn}
            YouTube Music hesabın ve yerel kayıtların ({combinedPlaylists.length} liste · {combined.length} şarkı)
          {:else}
            Kaydettiğin listeler ({combinedPlaylists.length}) ve beğendiğin şarkılar ({combined.length}) — Google hesabın için sol alttan bağlanabilirsin.
          {/if}
        </p>
      </div>
      {#if combined.length > 0}
        <button
          type="button"
          class="btn accent"
          onclick={() => player.playTrack(combined[0], combined, 'Kitaplık')}
        >
          <Icon name="play" size={18} />
          Şarkıları Çal
        </button>
      {/if}
    </header>

    <div class="lib-tabs" role="tablist" aria-label="Kitaplık görünümü">
      <button
        type="button"
        role="tab"
        aria-selected={libTab === 'all'}
        class="tab-pill"
        class:active={libTab === 'all'}
        onclick={() => (libTab = 'all')}
      >
        Tümü
      </button>
      <button
        type="button"
        role="tab"
        aria-selected={libTab === 'playlists'}
        class="tab-pill"
        class:active={libTab === 'playlists'}
        onclick={() => (libTab = 'playlists')}
      >
        <span>Çalma Listeleri</span>
        <span class="tab-count">{combinedPlaylists.length}</span>
      </button>
      <button
        type="button"
        role="tab"
        aria-selected={libTab === 'songs'}
        class="tab-pill"
        class:active={libTab === 'songs'}
        onclick={() => (libTab = 'songs')}
      >
        <span>Beğenilen Şarkılar</span>
        <span class="tab-count">{combined.length}</span>
      </button>
    </div>

    {#if libTab !== 'songs'}
      {#if loadingYtPlaylists && combinedPlaylists.length === 0}
        <div class="card empty">
          <p class="muted">Kayıtlı listeler yükleniyor...</p>
        </div>
      {:else if combinedPlaylists.length > 0}
        <section class="lib-section">
          {#if libTab === 'all'}
            <div class="sec-head">
              <h2>Çalma Listeleri & Albümler</h2>
              <span class="muted sec-sub">
                Kartı tıklayarak şarkıları incele veya kapağın üzerindeki oynat düğmesiyle başlat
              </span>
            </div>
          {/if}
          <div class="mix-grid">
            {#each combinedPlaylists as pl (pl.playlistId)}
              {@const isPlayingThis = player.state.sourceName === pl.title}
              {@const isBusy = playingPlaylistId === pl.playlistId}
              {@const plSaved = library.isPlaylistSaved(pl.playlistId)}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div
                class="card mix-card"
                class:active={isPlayingThis}
                onclick={() => openPlaylistView(pl)}
              >
                <div class="mix-art" style:background-color={pl.color}>
                  {#if pl.thumbnail}
                    <img src={pl.thumbnail} alt="" loading="lazy" draggable="false" />
                  {:else}
                    <div class="mix-badge-icon">
                      <Icon name="now" size={28} />
                    </div>
                  {/if}
                  <button
                    type="button"
                    class="mix-play-center"
                    title={`${pl.title} listesini hemen oynat`}
                    aria-label={`${pl.title} listesini hemen oynat`}
                    disabled={isBusy}
                    onclick={(e) => playPlaylistDirect(e, pl)}
                  >
                    <Icon name={isBusy ? 'refresh' : 'play'} size={20} />
                  </button>
                </div>
                <div class="mix-info">
                  <div class="mix-head-row">
                    <span class="mix-tag">{playlistBadgeLabel(pl.playlistId)}</span>
                    <button
                      type="button"
                      class="mix-save-btn"
                      class:on={plSaved}
                      title={plSaved ? 'Kitaplıktan kaldır' : 'Yerel kitaplığa sabitle'}
                      aria-label={plSaved ? 'Kitaplıktan kaldır' : 'Yerel kitaplığa sabitle'}
                      onclick={(e) => {
                        e.stopPropagation();
                        library.togglePlaylist(pl);
                      }}
                    >
                      <Icon name={plSaved ? 'bookmark-fill' : 'bookmark'} size={15} />
                    </button>
                  </div>
                  <span class="mix-title truncate">{pl.title}</span>
                  <span class="mix-sub truncate muted">
                    {isBusy ? 'Başlatılıyor...' : pl.subtitle}
                  </span>
                </div>
              </div>
            {/each}
          </div>
        </section>
      {:else if libTab === 'playlists'}
        <div class="card empty">
          <p class="muted">
            Henüz kayıtlı çalma listen yok. Ana Sayfa, Arama veya Sanatçı sayfalarındaki yer imi butonuna basarak çalma listelerini ve albümleri buraya kaydedebilirsin.
          </p>
        </div>
      {/if}
    {/if}

    {#if libTab !== 'playlists'}
      {#if loadingYt && combined.length === 0}
        <div class="card empty">
          <p class="muted">YouTube Music kitaplığın yükleniyor...</p>
        </div>
      {:else if combined.length === 0 && (libTab === 'songs' || combinedPlaylists.length === 0)}
        <div class="card empty">
          <p class="muted">
            Henüz kitaplığına şarkı veya liste eklemedin. Arama veya Ana Sayfa'dan kalp ve yer imi ikonlarıyla ekleyebilir ya da Google hesabını bağlayabilirsin.
          </p>
        </div>
      {:else if combined.length > 0}
        <section class="lib-section">
          {#if libTab === 'all' && combinedPlaylists.length > 0}
            <div class="sec-head">
              <h2>Beğenilen Şarkılar</h2>
            </div>
          {/if}
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
                    type="button"
                    class="btn ghost icon"
                    title="Şarkı Radyosu Başlat"
                    aria-label="Şarkı Radyosu Başlat"
                    onclick={() => player.startRadio(track)}
                  >
                    <Icon name="radio" size={17} />
                  </button>
                  <button
                    type="button"
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
        </section>
      {/if}
    {/if}
  {/if}
</section>

<style>
  .view {
    display: flex;
    flex-direction: column;
    gap: 18px;
    width: 100%;
    max-width: 1280px;
  }
  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    flex-wrap: wrap;
  }
  h1 {
    font-size: 26px;
    font-weight: 800;
  }
  h2 {
    font-size: 18px;
    font-weight: 800;
  }
  .lib-tabs {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }
  .tab-pill {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    min-height: 36px;
    padding: 0 14px;
    border-radius: 999px;
    background: var(--surface);
    border: 2px solid var(--border);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    color: var(--text-muted);
    font: inherit;
    font-size: 13px;
    font-weight: 800;
    cursor: pointer;
    transition: background-color 120ms ease, color 120ms ease, border-color 120ms ease;
  }
  .tab-pill:hover {
    background: var(--surface-2);
    color: var(--text);
  }
  .tab-pill.active {
    background: var(--accent);
    border-color: var(--accent-edge);
    box-shadow: 0 var(--edge-depth) 0 var(--accent-edge);
    color: var(--on-accent);
  }
  .tab-count {
    padding: 1px 7px;
    border-radius: 999px;
    background: rgb(0 0 0 / 0.16);
    font-size: 11px;
    font-variant-numeric: tabular-nums;
  }
  .lib-section {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .sec-head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 12px;
    flex-wrap: wrap;
  }
  .sec-sub {
    font-size: 12px;
  }

  /* Saved Playlists Grid */
  .mix-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(255px, 1fr));
    gap: 14px;
  }
  .view[data-card-size='compact'] .mix-grid {
    grid-template-columns: repeat(auto-fill, minmax(210px, 1fr));
    gap: 10px;
  }
  .view[data-card-size='large'] .mix-grid {
    grid-template-columns: repeat(auto-fill, minmax(310px, 1fr));
    gap: 16px;
  }
  .mix-card {
    display: grid;
    grid-template-columns: 74px minmax(0, 1fr);
    align-items: center;
    gap: 14px;
    padding: 12px;
    text-align: left;
    cursor: pointer;
    transition: transform 70ms ease, background-color 120ms ease;
  }
  .view[data-card-size='compact'] .mix-card {
    grid-template-columns: 58px minmax(0, 1fr);
    gap: 10px;
    padding: 9px;
  }
  .view[data-card-size='large'] .mix-card {
    grid-template-columns: 88px minmax(0, 1fr);
    gap: 16px;
    padding: 14px;
  }
  .mix-card:hover {
    background: var(--surface-2);
  }
  .mix-card.active {
    border-color: var(--accent);
    background: var(--accent-soft);
  }
  .mix-art {
    position: relative;
    width: 74px;
    height: 74px;
    border-radius: var(--radius);
    overflow: hidden;
    display: grid;
    place-items: center;
    color: #fff;
    border: 2px solid var(--border);
    flex-shrink: 0;
  }
  .view[data-card-size='compact'] .mix-art {
    width: 58px;
    height: 58px;
  }
  .view[data-card-size='large'] .mix-art {
    width: 88px;
    height: 88px;
  }
  .mix-art img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .mix-badge-icon {
    display: grid;
    place-items: center;
    width: 44px;
    height: 44px;
    border-radius: 12px;
    background: rgb(0 0 0 / 0.22);
  }
  .mix-play-center {
    position: absolute;
    inset: 0;
    margin: auto;
    width: 42px;
    height: 42px;
    border-radius: 50%;
    background: var(--accent);
    color: var(--on-accent);
    border: 2px solid var(--accent-edge);
    box-shadow: 0 var(--edge-depth) 0 var(--accent-edge);
    display: grid;
    place-items: center;
    cursor: pointer;
    opacity: 0;
    transform: scale(0.88);
    transition:
      opacity 120ms ease,
      transform 120ms ease,
      background-color 120ms ease;
  }
  .mix-art:hover .mix-play-center,
  .mix-play-center:focus-visible {
    opacity: 1;
    transform: scale(1);
  }
  .mix-play-center:hover {
    background: color-mix(in srgb, var(--accent) 88%, #fff);
  }
  .mix-info {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }
  .mix-head-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 6px;
  }
  .mix-tag {
    align-self: flex-start;
    padding: 2px 7px;
    border-radius: 6px;
    background: var(--surface-3);
    color: var(--text-muted);
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 0.05em;
  }
  .mix-card.active .mix-tag {
    background: var(--accent);
    color: var(--on-accent);
  }
  .mix-save-btn {
    width: 28px;
    height: 28px;
    padding: 0;
    border: 0;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-muted);
    display: grid;
    place-items: center;
    cursor: pointer;
    flex-shrink: 0;
    transition: background-color 120ms ease, color 120ms ease;
  }
  .mix-save-btn:hover {
    background: var(--surface-3);
    color: var(--text);
  }
  .mix-save-btn.on {
    color: var(--accent);
  }
  .mix-title {
    font-weight: 800;
    font-size: 15px;
  }
  .mix-sub {
    font-size: 12px;
  }

  /* Opened Playlist Detail Hero */
  .pl-topbar {
    display: flex;
    align-items: center;
  }
  .pl-hero {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    align-items: center;
    gap: 20px;
    padding: 20px;
  }
  .pl-hero-art {
    width: 132px;
    height: 132px;
    border-radius: var(--radius);
    border: 2px solid var(--border);
    overflow: hidden;
    display: grid;
    place-items: center;
    color: #fff;
    flex-shrink: 0;
  }
  .pl-hero-art img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .hero-body {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-width: 0;
  }
  .badge {
    align-self: flex-start;
    padding: 3px 10px;
    border-radius: var(--radius-sm);
    background: var(--accent);
    color: var(--on-accent);
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 0.05em;
  }
  .hero-title {
    font-size: clamp(20px, 2.4vw, 28px);
    font-weight: 800;
    line-height: 1.15;
  }
  .hero-meta {
    font-size: 13px;
  }
  .hero-btns {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
    margin-top: 4px;
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

  @media (max-width: 768px) {
    .mix-grid {
      grid-template-columns: 1fr;
    }
    .pl-hero {
      grid-template-columns: 1fr;
      text-align: center;
      padding: 16px;
    }
    .pl-hero-art {
      justify-self: center;
    }
    .badge {
      align-self: center;
    }
    .hero-btns {
      justify-content: center;
    }
  }
</style>
