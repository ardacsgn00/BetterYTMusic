<script lang="ts">
  import { onMount } from 'svelte';
  import { player } from '#lib/player.svelte.ts';
  import { library } from '#lib/library.svelte.ts';
  import { theme } from '#lib/theme.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import type { PlaylistCard, Track } from '#lib/types.ts';
  import Cover from './Cover.svelte';
  import Icon from './Icon.svelte';

  let tracks = $state<Track[]>([]);
  let playlists = $state<PlaylistCard[]>([]);
  let openedPlaylist = $state<PlaylistCard | null>(null);
  let openedTracks = $state<Track[]>([]);
  let openingPlaylist = $state(false);
  let playingPlaylistId = $state<string | null>(null);
  let zoomOpen = $state(false);
  let loading = $state(true);
  let error = $state<string | null>(null);

  const featured = $derived(tracks[0] ?? null);
  const quickPicks = $derived(tracks.slice(1, 9));
  const shelfTracks = $derived(tracks.length > 9 ? tracks.slice(9) : tracks.slice(1));

  async function loadHome() {
    loading = true;
    error = null;
    try {
      const [homeTracks, homeLists] = await Promise.all([
        player.home(),
        player.homePlaylists().catch(() => [] as PlaylistCard[])
      ]);
      tracks = homeTracks;
      playlists = homeLists;
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  /** Clicking the card body opens the playlist detail view so the user can browse all songs inside. */
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

  /** Clicking the hover Play button centered on the playlist cover starts playing immediately. */
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
    const copy = [...openedTracks];
    for (let i = copy.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [copy[i], copy[j]] = [copy[j], copy[i]];
    }
    player.playTrack(copy[0], copy, openedPlaylist.title);
  }

  function playShuffled() {
    if (tracks.length === 0) return;
    const copy = [...tracks];
    for (let i = copy.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [copy[i], copy[j]] = [copy[j], copy[i]];
    }
    player.playTrack(copy[0], copy, 'Ana Sayfa Karışık');
  }

  onMount(loadHome);
</script>

<svelte:window onclick={() => (zoomOpen = false)} />

<section class="view" data-card-size={theme.cardSize} class:hide-subs={theme.hideSubtitles}>
  {#if openedPlaylist}
    <!-- Dedicated Playlist Detail View -->
    <div class="pl-topbar">
      <button class="btn" onclick={() => (openedPlaylist = null)}>
        <Icon name="prev" size={16} />
        Ana Sayfaya Dön
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
        <span class="badge">
          {openedPlaylist.playlistId.startsWith('RD') ? 'KİŞİSEL MİKS' : 'ÇALMA LİSTESİ'}
        </span>
        <h2 class="hero-title">{openedPlaylist.title}</h2>
        <p class="hero-meta muted">
          {openedPlaylist.subtitle}
          {#if !openingPlaylist && openedTracks.length > 0}
            · {openedTracks.length} şarkı
          {/if}
        </p>
        <div class="hero-btns">
          <button
            class="btn accent"
            disabled={openingPlaylist || openedTracks.length === 0}
            onclick={playOpenedAll}
          >
            <Icon name="play" size={18} />
            Tümünü Oynat
          </button>
          <button
            class="btn"
            disabled={openingPlaylist || openedTracks.length === 0}
            onclick={playOpenedShuffled}
          >
            <Icon name="shuffle" size={17} />
            Karışık Çal
          </button>
        </div>
      </div>
    </div>

    {#if openingPlaylist}
      <div class="card msg">
        <p class="muted">"{openedPlaylist.title}" içeriği yükleniyor...</p>
      </div>
    {:else if openedTracks.length === 0}
      <div class="card msg">
        <p class="muted">Bu listede oynatılabilir parça bulunamadı.</p>
      </div>
    {:else}
      <div class="card pl-list">
        <ol>
          {#each openedTracks as track, idx (`${track.id}-${idx}`)}
            {@const active = player.state.track?.id === track.id}
            {@const liked = library.isFavorite(track.id)}
            <li class="pl-row" class:active>
              <button
                class="pl-hit"
                onclick={() => player.playTrack(track, openedTracks, openedPlaylist?.title)}
              >
                <span class="pl-idx muted">{idx + 1}</span>
                <Cover {track} size="46px" radius="var(--radius-sm)" />
                <span class="q-info">
                  <span class="q-title truncate">{track.title}</span>
                  <span class="q-artist truncate muted">
                    {track.artist}{track.album ? ` · ${track.album}` : ''}
                  </span>
                </span>
                <span class="q-dur muted">
                  {track.durationMs ? formatTime(track.durationMs) : ''}
                </span>
              </button>
              <button
                class="btn ghost icon q-fav"
                class:on={liked}
                title={liked ? 'Kitaplıktan çıkar' : 'Kitaplığa ekle'}
                onclick={() => library.toggle(track)}
              >
                <Icon name={liked ? 'heart-fill' : 'heart'} size={17} />
              </button>
            </li>
          {/each}
        </ol>
      </div>
    {/if}
  {:else}
    <!-- Standard Home View -->
    <header class="head">
      <div>
        <h1>Ana Sayfa</h1>
        <p class="muted">Sana özel seçimler, kişisel mixlerin ve güncel parçalar</p>
      </div>
      <div class="head-actions">
        <div class="zoom-pop-wrap">
          <button
            type="button"
            class="btn"
            class:accent={zoomOpen || theme.zoom !== 100}
            aria-expanded={zoomOpen}
            title="Görünürlük oranını ayarla"
            onclick={(e) => {
              e.stopPropagation();
              zoomOpen = !zoomOpen;
            }}
          >
            <Icon name="search" size={16} />
            %{theme.zoom}
          </button>

          {#if zoomOpen}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="card zoom-popover" onclick={(e) => e.stopPropagation()}>
              <span class="zp-label muted">Görünürlük Oranı</span>
              <div class="zp-row">
                <button
                  type="button"
                  class="btn zoom-step"
                  disabled={theme.zoom <= 75}
                  aria-label="Küçült"
                  onclick={() => theme.stepZoom(-5)}
                >
                  −
                </button>
                <span class="zp-val">%{theme.zoom}</span>
                <button
                  type="button"
                  class="btn zoom-step"
                  disabled={theme.zoom >= 135}
                  aria-label="Büyüt"
                  onclick={() => theme.stepZoom(5)}
                >
                  +
                </button>
              </div>
              <button
                type="button"
                class="btn zp-reset"
                disabled={theme.zoom === 100}
                onclick={() => theme.setZoom(100)}
              >
                %100 Varsayılana Sıfırla
              </button>
            </div>
          {/if}
        </div>

        {#if tracks.length > 0}
          <button class="btn accent" onclick={playShuffled}>
            <Icon name="shuffle" size={17} />
            Karışık Çal
          </button>
        {/if}
        <button class="btn" onclick={loadHome} disabled={loading}>
          <Icon name="refresh" size={17} />
          Yenile
        </button>
      </div>
    </header>

    {#if error}
      <div class="card msg">
        <p>{error}</p>
      </div>
    {:else if loading}
      <div class="card msg">
        <p class="muted">YouTube Music akışı hazırlanıyor...</p>
      </div>
    {:else if tracks.length > 0}
      {#if featured}
        {@const featActive = player.state.track?.id === featured.id}
        {@const featLiked = library.isFavorite(featured.id)}
        <div class="card hero" class:active={featActive}>
          <div class="hero-cover">
            <Cover track={featured} size="136px" radius="var(--radius)" />
          </div>
          <div class="hero-body">
            <span class="badge">ÖNE ÇIKAN SEÇİM</span>
            <h2 class="hero-title truncate">{featured.title}</h2>
            <p class="hero-meta muted truncate">
              {featured.artist}{featured.album ? ` · ${featured.album}` : ''}
              {featured.durationMs ? ` · ${formatTime(featured.durationMs)}` : ''}
            </p>
            <div class="hero-btns">
              <button
                class="btn accent"
                onclick={() => player.playTrack(featured, tracks, 'Ana Sayfa')}
              >
                <Icon name="play" size={18} />
                Hemen Dinle
              </button>
              <button
                class="btn"
                onclick={() => player.playTrack(featured, undefined, `${featured.title} Radyosu`)}
              >
                Radyo Başlat
              </button>
              <button
                class="btn"
                class:accent={featLiked}
                onclick={() => library.toggle(featured)}
              >
                <Icon name={featLiked ? 'heart-fill' : 'heart'} size={17} />
                {featLiked ? 'Favorilerde' : 'Favorilere Ekle'}
              </button>
            </div>
          </div>
        </div>
      {/if}

      {#if playlists.length > 0}
        <div class="section">
          <div class="sec-head">
            <h2>Süpermix & Çalma Listeleri</h2>
            <span class="muted sec-sub">
              Kapağın üzerindeki oynat düğmesiyle doğrudan başlat veya karta tıklayıp içeriğini gör
            </span>
          </div>
          <div class="mix-grid">
            {#each playlists as pl (pl.playlistId)}
              {@const isPlayingThis = player.state.sourceName === pl.title}
              {@const isBusy = playingPlaylistId === pl.playlistId}
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
                  <span class="mix-tag">
                    {pl.playlistId.startsWith('RD') ? 'MİKS' : 'LİSTE'}
                  </span>
                  <span class="mix-title truncate">{pl.title}</span>
                  <span class="mix-sub truncate muted">
                    {isBusy ? 'Başlatılıyor...' : pl.subtitle}
                  </span>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      {#if quickPicks.length > 0}
        <div class="section">
          <div class="sec-head">
            <h2>Hızlı Seçimler</h2>
            <span class="muted sec-sub">Tek tıkla sıraya ekle ve başlat</span>
          </div>
          <div class="quick-grid">
            {#each quickPicks as track (track.id)}
              {@const active = player.state.track?.id === track.id}
              {@const liked = library.isFavorite(track.id)}
              <div class="card quick-item" class:active>
                <button
                  class="quick-hit"
                  onclick={() => player.playTrack(track, tracks, 'Hızlı Seçimler')}
                >
                  <Cover {track} size="52px" radius="var(--radius-sm)" />
                  <span class="q-info">
                    <span class="q-title truncate">{track.title}</span>
                    <span class="q-artist truncate muted">{track.artist}</span>
                  </span>
                  <span class="q-dur muted">
                    {track.durationMs ? formatTime(track.durationMs) : ''}
                  </span>
                </button>
                <button
                  class="btn ghost icon q-fav"
                  class:on={liked}
                  title={liked ? 'Kitaplıktan çıkar' : 'Kitaplığa ekle'}
                  onclick={() => library.toggle(track)}
                >
                  <Icon name={liked ? 'heart-fill' : 'heart'} size={17} />
                </button>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      {#if shelfTracks.length > 0}
        <div class="section">
          <div class="sec-head">
            <h2>Sana Özel Öneriler</h2>
            <span class="muted sec-sub">{shelfTracks.length} parça</span>
          </div>
          <div class="grid">
            {#each shelfTracks as track (track.id)}
              {@const active = player.state.track?.id === track.id}
              {@const liked = library.isFavorite(track.id)}
              <div class="card item" class:active>
                <button
                  class="card-hit"
                  onclick={() => player.playTrack(track, tracks, 'Ana Sayfa')}
                >
                  <div class="cover-wrap">
                    <Cover {track} size="100%" radius="var(--radius)" />
                    <span class="play-pill">
                      <Icon name={active && player.state.isPlaying ? 'pause' : 'play'} size={16} />
                    </span>
                  </div>
                  <span class="t truncate">{track.title}</span>
                  <span class="a truncate muted">{track.artist}</span>
                </button>
                <button
                  class="btn ghost icon fav"
                  class:on={liked}
                  title={liked ? 'Kitaplıktan çıkar' : 'Kitaplığa ekle'}
                  onclick={() => library.toggle(track)}
                >
                  <Icon name={liked ? 'heart-fill' : 'heart'} size={17} />
                </button>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    {/if}
  {/if}
</section>

<style>
  .view {
    display: flex;
    flex-direction: column;
    gap: 24px;
    width: 100%;
    max-width: 1480px;
    padding-bottom: 24px;
  }
  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    flex-wrap: wrap;
  }
  .head-actions {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
  }
  .zoom-pop-wrap {
    position: relative;
  }
  .zoom-popover {
    position: absolute;
    top: calc(100% + 8px);
    right: 0;
    z-index: 40;
    min-width: 196px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .zp-label {
    font-size: 11px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    text-align: center;
  }
  .zp-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }
  .zoom-step {
    width: 38px;
    height: 36px;
    min-height: 36px;
    padding: 0;
    font-weight: 800;
    font-size: 18px;
    border-radius: var(--radius-sm);
  }
  .zp-val {
    font-weight: 800;
    font-size: 15px;
    font-variant-numeric: tabular-nums;
    min-width: 52px;
    text-align: center;
  }
  .zp-reset {
    width: 100%;
    min-height: 32px;
    font-size: 12px;
    font-weight: 700;
  }
  h1 {
    font-size: 26px;
    font-weight: 800;
  }
  h2 {
    font-size: 18px;
    font-weight: 800;
  }
  .msg {
    padding: 24px;
  }

  /* Hero Featured Card */
  .hero {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    align-items: center;
    gap: 22px;
    padding: 20px;
  }
  .hero.active {
    border-color: var(--accent);
    background: var(--accent-soft);
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
    font-size: 24px;
    font-weight: 800;
  }
  .hero-meta {
    font-size: 14px;
  }
  .hero-btns {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    margin-top: 6px;
  }

  /* Playlist Detail View */
  .pl-topbar {
    display: flex;
    align-items: center;
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
  .pl-list {
    padding: 10px;
  }
  .pl-list ol {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .pl-row {
    display: flex;
    align-items: center;
    gap: 6px;
    padding-right: 6px;
    border: 2px solid transparent;
    border-radius: var(--radius);
  }
  .pl-row:hover {
    background: var(--surface-2);
  }
  .pl-row.active {
    background: var(--accent-soft);
    border-color: var(--accent);
  }
  .pl-hit {
    flex: 1;
    display: grid;
    grid-template-columns: 32px 46px minmax(0, 1fr) auto;
    align-items: center;
    gap: 12px;
    padding: 7px 10px;
    border: 0;
    background: transparent;
    cursor: pointer;
    text-align: left;
    min-width: 0;
  }
  .pl-idx {
    text-align: center;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    font-size: 13px;
  }

  /* Sections */
  .section {
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
  .view.hide-subs .head p,
  .view.hide-subs .sec-sub {
    display: none;
  }

  /* Mix & Discovery Playlists Grid */
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
  /* Centered Play button on top of the album cover, visible ONLY on hover */
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
  .mix-play-center:active {
    transform: scale(0.94);
  }
  .mix-info {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
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
  .mix-title {
    font-weight: 800;
    font-size: 15px;
  }
  .mix-sub {
    font-size: 12px;
  }

  /* Quick Picks 2-col / 3-col Grid */
  .quick-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(310px, 1fr));
    gap: 12px;
  }
  .view[data-card-size='compact'] .quick-grid {
    grid-template-columns: repeat(auto-fill, minmax(255px, 1fr));
    gap: 10px;
  }
  .view[data-card-size='large'] .quick-grid {
    grid-template-columns: repeat(auto-fill, minmax(370px, 1fr));
    gap: 14px;
  }
  .quick-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px 6px 6px;
    border-radius: var(--radius);
  }
  .quick-item:hover {
    background: var(--surface-2);
  }
  .quick-item.active {
    border-color: var(--accent);
    background: var(--accent-soft);
  }
  .quick-hit {
    flex: 1;
    display: grid;
    grid-template-columns: 52px minmax(0, 1fr) auto;
    align-items: center;
    gap: 12px;
    border: 0;
    padding: 2px;
    background: transparent;
    cursor: pointer;
    text-align: left;
    min-width: 0;
  }
  .q-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .q-title {
    font-weight: 700;
    font-size: 14px;
  }
  .quick-item.active .q-title,
  .pl-row.active .q-title {
    color: var(--accent);
  }
  .q-artist {
    font-size: 12px;
  }
  .q-dur {
    font-size: 12px;
    font-variant-numeric: tabular-nums;
  }
  .q-fav {
    width: 34px;
    height: 34px;
    min-height: 34px;
  }

  /* Shelf Cards Grid */
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(175px, 1fr));
    gap: 16px;
  }
  .view[data-card-size='compact'] .grid {
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 12px;
  }
  .view[data-card-size='large'] .grid {
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 18px;
  }
  .item {
    position: relative;
    padding: 12px;
    display: flex;
    flex-direction: column;
    transition: transform 70ms ease;
  }
  .item:hover {
    background: var(--surface-2);
  }
  .item.active {
    border-color: var(--accent);
    background: var(--accent-soft);
  }
  .card-hit {
    display: flex;
    flex-direction: column;
    gap: 8px;
    border: 0;
    padding: 0;
    background: transparent;
    cursor: pointer;
    text-align: left;
    min-width: 0;
  }
  .cover-wrap {
    position: relative;
    width: 100%;
  }
  .play-pill {
    position: absolute;
    bottom: 8px;
    right: 8px;
    width: 34px;
    height: 34px;
    border-radius: 50%;
    background: var(--accent);
    color: var(--on-accent);
    border: 2px solid var(--accent-edge);
    box-shadow: 0 var(--edge-depth) 0 var(--accent-edge);
    display: grid;
    place-items: center;
    opacity: 0;
    transform: translateY(4px);
    transition: opacity 120ms ease, transform 120ms ease;
  }
  .item:hover .play-pill,
  .item.active .play-pill {
    opacity: 1;
    transform: translateY(0);
  }
  .t {
    font-weight: 700;
    font-size: 14px;
    margin-top: 2px;
  }
  .a {
    font-size: 12px;
  }
  .fav {
    position: absolute;
    top: 16px;
    right: 16px;
    width: 32px;
    height: 32px;
    min-height: 32px;
    border-radius: 50%;
    background: var(--surface);
    border: 2px solid var(--border);
  }

  @media (max-width: 640px) {
    .hero {
      grid-template-columns: 1fr;
      text-align: center;
    }
    .hero-cover,
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
