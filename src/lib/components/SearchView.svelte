<script lang="ts">
  import { onMount } from 'svelte';
  import { player } from '#lib/player.svelte.ts';
  import { library } from '#lib/library.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import type { Track, PlaylistCard, ArtistCard } from '#lib/types.ts';
  import Cover from './Cover.svelte';
  import Icon from './Icon.svelte';

  type SearchTab = 'all' | 'songs' | 'playlists';

  let query = $state('');
  let activeTab = $state<SearchTab>('all');
  let songResults = $state<Track[]>([]);
  let playlistResults = $state<PlaylistCard[]>([]);
  let artistResult = $state<ArtistCard | null>(null);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let searched = $state(false);

  // Explore & Trending recommendations when not searching
  let explorePlaylists = $state<PlaylistCard[]>([]);
  let loadingExplore = $state(false);

  // Playlist Inspection View
  let openedPlaylist = $state<PlaylistCard | null>(null);
  let openedTracks = $state<Track[]>([]);
  let loadingOpened = $state(false);
  let openedError = $state<string | null>(null);

  const SUGGESTIONS = [
    'Rock Klasikleri',
    'Türkçe Pop',
    'Lo-Fi Beats',
    'Anadolu Rock',
    'Yabancı Hit',
    '90lar & 2000ler',
    'Workout & Gym',
    'Akustik'
  ];

  onMount(async () => {
    loadingExplore = true;
    try {
      const lists = await player.homePlaylists();
      explorePlaylists = lists;
    } catch {
      // non-fatal
    } finally {
      loadingExplore = false;
    }
  });

  async function doSearch(q = query) {
    const trimmed = q.trim();
    if (!trimmed) return;
    query = trimmed;
    loading = true;
    error = null;
    searched = true;
    openedPlaylist = null;

    try {
      const [songsRes, listsRes, artistRes] = await Promise.allSettled([
        player.search(trimmed),
        player.searchPlaylists(trimmed),
        player.artistSearch(trimmed)
      ]);

      songResults = songsRes.status === 'fulfilled' ? songsRes.value : [];
      playlistResults = listsRes.status === 'fulfilled' ? listsRes.value : [];
      artistResult = artistRes.status === 'fulfilled' ? artistRes.value : null;

      if (songsRes.status === 'rejected' && listsRes.status === 'rejected') {
        error = 'Arama sonuçları alınamadı. Lütfen internet bağlantınızı kontrol edin.';
      }
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function clearSearch() {
    query = '';
    songResults = [];
    playlistResults = [];
    artistResult = null;
    searched = false;
    error = null;
    openedPlaylist = null;
  }

  function onSubmit(e: SubmitEvent) {
    e.preventDefault();
    doSearch();
  }

  function applySuggestion(term: string) {
    query = term;
    doSearch(term);
  }

  async function openPlaylistView(pl: PlaylistCard) {
    openedPlaylist = pl;
    loadingOpened = true;
    openedError = null;
    openedTracks = [];

    try {
      const tracks = await player.playlistTracks(pl.playlistId);
      openedTracks = tracks;
    } catch (e) {
      openedError = `Liste içeriği alınamadı: ${e}`;
    } finally {
      loadingOpened = false;
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
</script>

<section class="view">
  <!-- SEARCH BAR & QUERY HEADER -->
  <header class="head">
    <div class="head-top">
      <h1>Müzik & Çalma Listesi Ara</h1>
    </div>

    <form class="search-bar" onsubmit={onSubmit}>
      <span class="s-icon"><Icon name="search" size={18} /></span>
      <input
        type="text"
        placeholder="Şarkı, sanatçı, albüm veya çalma listesi ara..."
        bind:value={query}
      />
      {#if query.length > 0}
        <button
          type="button"
          class="btn ghost icon clear-btn"
          title="Aramayı Temizle"
          aria-label="Aramayı Temizle"
          onclick={clearSearch}
        >
          <Icon name="close" size={16} />
        </button>
      {/if}
      <button type="submit" class="btn accent" disabled={loading || !query.trim()}>
        {loading ? 'Aranıyor...' : 'Ara'}
      </button>
    </form>

    <!-- QUICK VIBE / SUGGESTION CHIPS -->
    <div class="chip-scroll">
      {#each SUGGESTIONS as tag}
        <button
          type="button"
          class="chip-btn"
          class:active={query.toLowerCase() === tag.toLowerCase()}
          onclick={() => applySuggestion(tag)}
        >
          {tag}
        </button>
      {/each}
    </div>
  </header>

  <!-- PLAYLIST INSPECTION VIEW (WHEN USER CLICKS ANY PLAYLIST OR ALBUM) -->
  {#if openedPlaylist}
    <div class="opened-view card">
      <div class="opened-toolbar">
        <button class="btn back-btn" onclick={() => (openedPlaylist = null)}>
          <Icon name="back" size={16} />
          <span>{searched ? 'Arama Sonuçlarına Dön' : 'Keşfete Dön'}</span>
        </button>
      </div>

      <header class="pl-hero">
        <div class="pl-hero-art" style:background-color={openedPlaylist.color}>
          {#if openedPlaylist.thumbnail}
            <img src={openedPlaylist.thumbnail} alt="" draggable="false" />
          {:else}
            <div class="art-placeholder">
              <Icon name="music" size={36} />
            </div>
          {/if}
        </div>
        <div class="pl-hero-info">
          <span class="badge">
            {openedPlaylist.playlistId.startsWith('RD')
              ? 'KİŞİSEL MİKS'
              : openedPlaylist.playlistId.startsWith('MPREb_')
              ? 'ALBÜM'
              : 'ÇALMA LİSTESİ'}
          </span>
          <h2 class="hero-title">{openedPlaylist.title}</h2>
          <p class="hero-sub muted">{openedPlaylist.subtitle}</p>
          <div class="hero-meta">
            {#if openedTracks.length > 0}
              <span class="muted">{openedTracks.length} parça</span>
            {/if}
          </div>
          <div class="hero-actions">
            <button
              class="btn accent hero-play-btn"
              disabled={loadingOpened || openedTracks.length === 0}
              onclick={playOpenedAll}
            >
              <Icon name="play" size={16} />
              Tümünü Oynat
            </button>
            <button
              class="btn hero-play-btn"
              disabled={loadingOpened || openedTracks.length === 0}
              onclick={playOpenedShuffled}
            >
              <Icon name="shuffle" size={16} />
              Karışık Çal
            </button>
          </div>
        </div>
      </header>

      {#if openedError}
        <div class="msg error">
          <p>{openedError}</p>
        </div>
      {:else if loadingOpened}
        <div class="msg">
          <p class="muted">"{openedPlaylist.title}" parça listesi yükleniyor...</p>
        </div>
      {:else if openedTracks.length > 0}
        <div class="opened-tracklist">
          <ol class="track-ol">
            {#each openedTracks as track, idx (track.id)}
              {@const active = player.state.track?.id === track.id}
              {@const liked = library.isFavorite(track.id)}
              <li class="pl-track-row" class:active>
                <span class="t-idx muted">{idx + 1}</span>
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div
                  class="pl-track-hit"
                  onclick={() => player.playTrack(track, openedTracks, openedPlaylist?.title)}
                >
                  <Cover {track} size="40px" radius="calc(var(--radius) * 0.5)" />
                  <span class="t-meta">
                    <span class="t-name truncate">{track.title}</span>
                    <span class="t-artist truncate muted">
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
                        <span class="album-tag"> · {track.album}</span>
                      {/if}
                    </span>
                  </span>
                  <span class="t-dur muted">{track.durationMs ? formatTime(track.durationMs) : ''}</span>
                </div>
                <button
                  class="btn ghost icon pl-radio"
                  title="Şarkı Radyosu Başlat"
                  aria-label="Şarkı Radyosu Başlat"
                  onclick={() => player.startRadio(track)}
                >
                  <Icon name="radio" size={16} />
                </button>
                <button
                  class="btn ghost icon fav"
                  class:on={liked}
                  title={liked ? 'Kitaplıktan Çıkar' : 'Kitaplığa Ekle'}
                  aria-label={liked ? 'Kitaplıktan Çıkar' : 'Kitaplığa Ekle'}
                  onclick={() => library.toggle(track)}
                >
                  <Icon name={liked ? 'heart-fill' : 'heart'} size={17} />
                </button>
              </li>
            {/each}
          </ol>
        </div>
      {:else}
        <div class="msg">
          <p class="muted">Bu listede çalınabilir şarkı bulunamadı.</p>
        </div>
      {/if}
    </div>

  <!-- SEARCH RESULTS VIEW -->
  {:else if searched}
    <!-- FILTER TABS -->
    <div class="tabs-bar">
      <button
        class="tab-btn"
        class:active={activeTab === 'all'}
        onclick={() => (activeTab = 'all')}
      >
        Tümü ({songResults.length + playlistResults.length})
      </button>
      <button
        class="tab-btn"
        class:active={activeTab === 'songs'}
        onclick={() => (activeTab = 'songs')}
      >
        Şarkılar ({songResults.length})
      </button>
      <button
        class="tab-btn"
        class:active={activeTab === 'playlists'}
        onclick={() => (activeTab = 'playlists')}
      >
        Çalma Listeleri & Albümler ({playlistResults.length})
      </button>
    </div>

    {#if artistResult}
      <div class="card artist-banner">
        <div class="ab-avatar">
          {#if artistResult.thumbnail}
            <img src={artistResult.thumbnail} alt={artistResult.name} draggable="false" />
          {:else}
            <Icon name="user" size={32} />
          {/if}
        </div>
        <div class="ab-info">
          <span class="badge">SANATÇI</span>
          <h2 class="ab-name">{artistResult.name}</h2>
          {#if artistResult.subscribers}
            <span class="ab-subs muted">{artistResult.subscribers}</span>
          {/if}
        </div>
        <button
          class="btn accent ab-btn"
          onclick={() => player.openArtist(artistResult!.channelId)}
        >
          <Icon name="user" size={16} />
          Sanatçı Sayfasını Aç
        </button>
      </div>
    {/if}

    {#if error}
      <div class="card msg error">
        <p>{error}</p>
      </div>
    {:else if loading}
      <div class="card msg">
        <p class="muted">YouTube Music sonuçları getiriliyor...</p>
      </div>
    {:else if songResults.length === 0 && playlistResults.length === 0}
      <div class="card msg">
        <p class="muted">"{query}" için herhangi bir şarkı veya çalma listesi bulunamadı.</p>
      </div>
    {:else}
      <!-- SECTION: PLAYLISTS & ALBUMS (Shown on 'all' or 'playlists' tab) -->
      {#if (activeTab === 'all' || activeTab === 'playlists') && playlistResults.length > 0}
        <section class="section-block">
          <div class="section-head">
            <h2>Çalma Listeleri & Albümler</h2>
            {#if activeTab === 'all' && playlistResults.length > 8}
              <button class="btn sm" onclick={() => (activeTab = 'playlists')}>
                Tümünü Gör ({playlistResults.length})
              </button>
            {/if}
          </div>

          <div class="cards-grid">
            {#each (activeTab === 'all' ? playlistResults.slice(0, 8) : playlistResults) as pl (pl.playlistId)}
              <div class="pl-card card">
                <button class="pl-card-btn" onclick={() => openPlaylistView(pl)}>
                  <div class="pl-card-art" style:background-color={pl.color}>
                    {#if pl.thumbnail}
                      <img src={pl.thumbnail} alt="" loading="lazy" draggable="false" />
                    {:else}
                      <div class="art-placeholder">
                        <Icon name="music" size={28} />
                      </div>
                    {/if}
                    <div class="pl-card-overlay">
                      <span class="pl-play-icon"><Icon name="play" size={18} /></span>
                    </div>
                  </div>
                  <div class="pl-card-info">
                    <span class="pl-card-title truncate" title={pl.title}>{pl.title}</span>
                    <span class="pl-card-sub truncate muted" title={pl.subtitle}>{pl.subtitle}</span>
                  </div>
                </button>
                <div class="pl-card-footer">
                  <button
                    class="btn sm inspect-quick-btn"
                    title="İçeriği İncele"
                    onclick={() => openPlaylistView(pl)}
                  >
                    Listeyi Aç
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </section>
      {/if}

      <!-- SECTION: SONGS (Shown on 'all' or 'songs' tab) -->
      {#if (activeTab === 'all' || activeTab === 'songs') && songResults.length > 0}
        <section class="section-block">
          <div class="section-head">
            <h2>Şarkılar</h2>
            {#if activeTab === 'all' && songResults.length > 15}
              <button class="btn sm" onclick={() => (activeTab = 'songs')}>
                Tüm Şarkıları Gör ({songResults.length})
              </button>
            {/if}
          </div>

          <div class="card results">
            <ol>
              {#each (activeTab === 'all' ? songResults.slice(0, 15) : songResults) as track (track.id)}
                {@const active = player.state.track?.id === track.id}
                {@const liked = library.isFavorite(track.id)}
                <li class="row" class:active>
                  <!-- svelte-ignore a11y_click_events_have_key_events -->
                  <!-- svelte-ignore a11y_no_static_element_interactions -->
                  <div class="main-hit" onclick={() => player.playTrack(track, songResults, `Arama: ${query}`)}>
                    <Cover {track} size="46px" radius="var(--radius-sm)" />
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
                          <span class="album-tag"> · {track.album}</span>
                        {/if}
                      </span>
                    </span>
                    <span class="dur muted">{track.durationMs ? formatTime(track.durationMs) : ''}</span>
                  </div>
                  <button
                    class="btn ghost icon s-radio"
                    title="Şarkı Radyosu Başlat"
                    aria-label="Şarkı Radyosu Başlat"
                    onclick={() => player.startRadio(track)}
                  >
                    <Icon name="radio" size={17} />
                  </button>
                  <button
                    class="btn ghost icon fav"
                    class:on={liked}
                    title={liked ? 'Kitaplıktan Çıkar' : 'Kitaplığa Ekle'}
                    aria-label={liked ? 'Kitaplıktan Çıkar' : 'Kitaplığa Ekle'}
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

  <!-- DISCOVERY / EXPLORE VIEW (WHEN SEARCH BAR IS EMPTY) -->
  {:else}
    <section class="explore-section">
      <div class="section-head">
        <div>
          <h2>Keşfet & Öne Çıkan Listeler</h2>
          <p class="muted">Sevdiğin tarzda çalma listelerini keşfet, tek tıkla içine göz at veya oynat.</p>
        </div>
      </div>

      {#if loadingExplore}
        <div class="card msg">
          <p class="muted">Keşif listeleri yükleniyor...</p>
        </div>
      {:else if explorePlaylists.length > 0}
        <div class="cards-grid">
          {#each explorePlaylists as pl (pl.playlistId)}
            <div class="pl-card card">
              <button class="pl-card-btn" onclick={() => openPlaylistView(pl)}>
                <div class="pl-card-art" style:background-color={pl.color}>
                  {#if pl.thumbnail}
                    <img src={pl.thumbnail} alt="" loading="lazy" draggable="false" />
                  {:else}
                    <div class="art-placeholder">
                      <Icon name="music" size={28} />
                    </div>
                  {/if}
                  <div class="pl-card-overlay">
                    <span class="pl-play-icon"><Icon name="play" size={18} /></span>
                  </div>
                </div>
                <div class="pl-card-info">
                  <span class="pl-card-title truncate" title={pl.title}>{pl.title}</span>
                  <span class="pl-card-sub truncate muted" title={pl.subtitle}>{pl.subtitle}</span>
                </div>
              </button>
              <div class="pl-card-footer">
                <button
                  class="btn sm inspect-quick-btn"
                  title="İçeriği İncele"
                  onclick={() => openPlaylistView(pl)}
                >
                  Listeyi Aç
                </button>
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <div class="card msg">
          <p class="muted">Şarkı, sanatçı veya çalma listesi bulmak için yukarıdan arama yapabilirsin.</p>
        </div>
      {/if}
    </section>
  {/if}
</section>

<style>
  .view {
    display: flex;
    flex-direction: column;
    gap: 22px;
    width: 100%;
    max-width: 1300px;
  }
  .head {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }
  .head-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
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

  /* Chip scroll / suggestions */
  .chip-scroll {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    padding-bottom: 2px;
  }
  .chip-btn {
    background: var(--surface);
    color: var(--text-muted);
    border: 1.5px solid var(--border);
    border-radius: 999px;
    padding: 5px 13px;
    font-size: 12.5px;
    font-weight: 700;
    cursor: pointer;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    transition: all 0.12s ease;
  }
  .chip-btn:hover {
    color: var(--text);
    border-color: var(--accent);
    transform: translateY(-1px);
  }
  .chip-btn.active {
    background: var(--accent);
    color: #fff;
    border-color: var(--accent);
  }

  /* Tabs */
  .tabs-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    border-bottom: 1.5px solid var(--border);
    padding-bottom: 12px;
  }
  .tab-btn {
    background: var(--surface);
    color: var(--text-muted);
    border: 1.5px solid var(--border);
    border-radius: var(--radius);
    padding: 7px 14px;
    font-size: 13px;
    font-weight: 700;
    cursor: pointer;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    transition: all 0.12s ease;
  }
  .tab-btn:hover {
    color: var(--text);
    border-color: var(--accent);
  }
  .tab-btn.active {
    background: var(--accent);
    color: #fff;
    border-color: var(--accent);
  }

  /* Section */
  .section-block {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }
  .section-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }
  .section-head h2 {
    font-size: 18px;
    font-weight: 800;
  }
  .section-head p {
    font-size: 13px;
    margin-top: 2px;
  }

  /* Cards Grid */
  .cards-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 14px;
  }
  .pl-card {
    display: flex;
    flex-direction: column;
    padding: 10px;
    gap: 8px;
    transition: transform 0.12s ease, border-color 0.12s ease;
  }
  .pl-card:hover {
    border-color: var(--accent);
    transform: translateY(-2px);
  }
  .pl-card-btn {
    background: none;
    border: none;
    color: inherit;
    text-align: left;
    cursor: pointer;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 100%;
  }
  .pl-card-art {
    position: relative;
    width: 100%;
    aspect-ratio: 1;
    border-radius: calc(var(--radius) * 0.7);
    overflow: hidden;
    display: grid;
    place-items: center;
  }
  .pl-card-art img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .art-placeholder {
    color: #fff;
    opacity: 0.85;
  }
  .pl-card-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    display: grid;
    place-items: center;
    opacity: 0;
    transition: opacity 0.15s ease;
  }
  .pl-card:hover .pl-card-overlay {
    opacity: 1;
  }
  .pl-play-icon {
    width: 38px;
    height: 38px;
    border-radius: 50%;
    background: var(--accent);
    color: #fff;
    display: grid;
    place-items: center;
    box-shadow: 0 3px 6px rgba(0, 0, 0, 0.4);
    transform: scale(0.9);
    transition: transform 0.12s ease;
  }
  .pl-card:hover .pl-play-icon {
    transform: scale(1);
  }
  .pl-card-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .pl-card-title {
    font-size: 13.5px;
    font-weight: 700;
  }
  .pl-card-sub {
    font-size: 11.5px;
  }
  .pl-card-footer {
    display: flex;
    margin-top: auto;
    padding-top: 4px;
  }
  .inspect-quick-btn {
    width: 100%;
    font-size: 11.5px;
    font-weight: 700;
    padding: 5px;
    justify-content: center;
  }

  /* Songs Results */
  .results {
    padding: 6px;
  }
  ol {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 8px;
    border-radius: var(--radius-sm);
    transition: background 0.1s ease;
  }
  .row:hover {
    background: var(--surface-2);
  }
  .row.active {
    background: color-mix(in srgb, var(--accent) 18%, transparent);
  }
  .main-hit {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 12px;
    border: 0;
    background: transparent;
    color: inherit;
    font: inherit;
    text-align: left;
    cursor: pointer;
    min-width: 0;
    padding: 4px 0;
  }
  .info {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
    gap: 2px;
  }
  .t {
    font-weight: 700;
    font-size: 14px;
  }
  .a {
    font-size: 12px;
  }
  .dur {
    font-size: 12px;
    font-variant-numeric: tabular-nums;
    margin-right: 6px;
  }
  .fav {
    color: var(--text-muted);
  }
  .fav.on {
    color: var(--accent);
  }

  /* Opened Playlist View */
  .opened-view {
    display: flex;
    flex-direction: column;
    gap: 18px;
    padding: 20px;
  }
  .opened-toolbar {
    display: flex;
    align-items: center;
  }
  .back-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    font-weight: 700;
    font-size: 13px;
  }
  .pl-hero {
    display: flex;
    gap: 20px;
    align-items: center;
  }
  .pl-hero-art {
    width: 140px;
    height: 140px;
    border-radius: var(--radius);
    overflow: hidden;
    flex-shrink: 0;
    display: grid;
    place-items: center;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    border: 2px solid var(--border);
  }
  .pl-hero-art img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .pl-hero-info {
    display: flex;
    flex-direction: column;
    gap: 6px;
    min-width: 0;
  }
  .badge {
    align-self: flex-start;
    font-size: 10.5px;
    font-weight: 800;
    letter-spacing: 0.05em;
    padding: 3px 8px;
    border-radius: 4px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    color: var(--text-muted);
  }
  .hero-title {
    font-size: 22px;
    font-weight: 800;
    line-height: 1.2;
    margin: 0;
  }
  .hero-sub {
    font-size: 13px;
    margin: 0;
  }
  .hero-meta {
    font-size: 12px;
  }
  .hero-actions {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 6px;
  }
  .hero-play-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    font-weight: 700;
  }

  .opened-tracklist {
    margin-top: 6px;
  }
  .track-ol {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .pl-track-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 10px;
    border-radius: var(--radius-sm);
    transition: background 0.1s ease;
  }
  .pl-track-row:hover {
    background: var(--surface-2);
  }
  .pl-track-row.active {
    background: color-mix(in srgb, var(--accent) 18%, transparent);
  }
  .t-idx {
    font-size: 12px;
    font-variant-numeric: tabular-nums;
    width: 22px;
    text-align: center;
    flex-shrink: 0;
  }
  .pl-track-hit {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 12px;
    background: none;
    border: none;
    color: inherit;
    font: inherit;
    text-align: left;
    cursor: pointer;
    min-width: 0;
    padding: 0;
  }
  .t-meta {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .t-name {
    font-size: 13.5px;
    font-weight: 700;
  }
  .t-artist {
    font-size: 11.5px;
  }
  .t-dur {
    font-size: 12px;
    font-variant-numeric: tabular-nums;
    margin-right: 6px;
  }

  .msg {
    padding: 24px;
    text-align: center;
  }
  .msg.error {
    color: var(--accent);
  }

  .artist-banner {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 16px 20px;
    margin-bottom: 6px;
    border-color: var(--accent);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }
  .ab-avatar {
    width: 60px;
    height: 60px;
    border-radius: 50%;
    overflow: hidden;
    background: var(--surface-2);
    border: 2px solid var(--border);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }
  .ab-avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .ab-info {
    display: flex;
    flex-direction: column;
    gap: 3px;
    flex: 1;
    min-width: 0;
  }
  .ab-name {
    font-size: 18px;
    font-weight: 800;
  }
  .ab-subs {
    font-size: 12px;
  }
  .ab-btn {
    flex-shrink: 0;
  }
  .s-radio,
  .pl-radio {
    width: 34px;
    height: 34px;
    min-height: 34px;
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

  @media (max-width: 768px) {
    .pl-hero {
      flex-direction: column;
      align-items: flex-start;
    }
    .pl-hero-art {
      width: 110px;
      height: 110px;
    }
    .cards-grid {
      grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    }
  }
</style>
