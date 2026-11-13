<script lang="ts">
  import { onMount } from 'svelte';
  import { player } from '#lib/player.svelte.ts';
  import { library } from '#lib/library.svelte.ts';
  import { theme } from '#lib/theme.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import type { MoodCategory, PlaylistCard, Track, ArtistCard } from '#lib/types.ts';
  import Cover from './Cover.svelte';
  import Icon from './Icon.svelte';

  let tracks = $state<Track[]>([]);
  let playlists = $state<PlaylistCard[]>([]);
  let moodCategories = $state<MoodCategory[]>([]);
  let selectedMood = $state<MoodCategory | null>(null);
  let moodPlaylists = $state<PlaylistCard[]>([]);
  let loadingMood = $state(false);

  // Home search state
  let homeSearchQuery = $state('');
  let homeActiveQuery = $state('');
  let homeSongResults = $state<Track[]>([]);
  let homePlaylistResults = $state<PlaylistCard[]>([]);
  let homeArtistResult = $state<ArtistCard | null>(null);
  let homeSearching = $state(false);
  let homeSearched = $state(false);
  let homeSearchError = $state<string | null>(null);

  async function doHomeSearch(q = homeSearchQuery) {
    const trimmed = q.trim();
    if (!trimmed) return;
    homeSearchQuery = trimmed;
    homeActiveQuery = trimmed;
    homeSearching = true;
    homeSearchError = null;
    homeSearched = true;

    try {
      const [songsRes, listsRes, artistRes] = await Promise.allSettled([
        player.search(trimmed),
        player.searchPlaylists(trimmed),
        player.artistSearch(trimmed)
      ]);

      homeSongResults = songsRes.status === 'fulfilled' ? songsRes.value : [];
      homePlaylistResults = listsRes.status === 'fulfilled' ? listsRes.value : [];
      homeArtistResult = artistRes.status === 'fulfilled' ? artistRes.value : null;

      if (songsRes.status === 'rejected' && listsRes.status === 'rejected') {
        homeSearchError = 'Arama sonuçları alınamadı. Lütfen internet bağlantınızı kontrol edin.';
      }
    } catch (e) {
      homeSearchError = String(e);
    } finally {
      homeSearching = false;
    }
  }

  function clearHomeSearch() {
    homeSearchQuery = '';
    homeActiveQuery = '';
    homeSongResults = [];
    homePlaylistResults = [];
    homeArtistResult = null;
    homeSearched = false;
    homeSearchError = null;
  }

  function onHomeSearchSubmit(e: SubmitEvent) {
    e.preventDefault();
    doHomeSearch();
  }

  let openedPlaylist = $state<PlaylistCard | null>(null);
  let openedTracks = $state<Track[]>([]);
  let openingPlaylist = $state(false);
  let playingPlaylistId = $state<string | null>(null);
  let zoomOpen = $state(false);
  let loading = $state(true);
  let error = $state<string | null>(null);

  const quickPicks = $derived(tracks.slice(0, 8));
  const shelfTracks = $derived(tracks.length > 8 ? tracks.slice(8) : tracks);

  async function loadHome() {
    loading = true;
    error = null;
    try {
      const [homeTracks, homeLists, moods] = await Promise.all([
        player.home(),
        player.homePlaylists().catch(() => [] as PlaylistCard[]),
        player.moodCategories().catch(() => [] as MoodCategory[])
      ]);
      tracks = homeTracks;
      playlists = homeLists;
      moodCategories = moods;
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function selectMoodCategory(cat: MoodCategory) {
    if (selectedMood?.params === cat.params) {
      selectedMood = null;
      moodPlaylists = [];
      return;
    }
    selectedMood = cat;
    loadingMood = true;
    try {
      moodPlaylists = await player.moodPlaylists(cat.params);
    } catch {
      moodPlaylists = [];
    } finally {
      loadingMood = false;
    }
  }

  function clearMoodCategory() {
    selectedMood = null;
    moodPlaylists = [];
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
    {@const plSaved = library.isPlaylistSaved(openedPlaylist.playlistId)}
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
          <button
            type="button"
            class="btn"
            class:accent={plSaved}
            title={plSaved ? 'Kitaplıktan kaldır' : 'Listeyi kitaplığa kaydet'}
            onclick={() => openedPlaylist && library.togglePlaylist(openedPlaylist)}
          >
            <Icon name={plSaved ? 'bookmark-fill' : 'bookmark'} size={17} />
            <span>{plSaved ? 'Kaydedildi' : 'Kitaplığa Kaydet'}</span>
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
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div
                class="pl-hit"
                onclick={() => player.playTrack(track, openedTracks, openedPlaylist?.title)}
              >
                <span class="pl-idx muted">{idx + 1}</span>
                <Cover {track} size="46px" radius="var(--radius-sm)" />
                <span class="q-info">
                  <span class="q-title truncate">{track.title}</span>
                  <span class="q-artist-line truncate muted">
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
                <span class="q-dur muted">
                  {track.durationMs ? formatTime(track.durationMs) : ''}
                </span>
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

    {#if theme.searchPlacement === 'home' || theme.searchPlacement === 'both'}
      <div class="home-search-wrap">
        <form class="home-search-form" onsubmit={onHomeSearchSubmit}>
          <span class="home-search-icon"><Icon name="search" size={18} /></span>
          <input
            type="text"
            class="home-search-input"
            placeholder="Şarkı, sanatçı, albüm veya çalma listesi ara..."
            bind:value={homeSearchQuery}
          />
          {#if homeSearchQuery.length > 0}
            <button
              type="button"
              class="btn ghost icon home-search-clear-btn"
              title="Aramayı Temizle"
              aria-label="Aramayı Temizle"
              onclick={clearHomeSearch}
            >
              <Icon name="close" size={16} />
            </button>
          {/if}
          <button
            type="submit"
            class="btn accent home-search-btn"
            disabled={homeSearching || !homeSearchQuery.trim()}
          >
            {homeSearching ? 'Aranıyor...' : 'Ara'}
          </button>
        </form>
      </div>

      {#if homeSearched}
        <section class="home-search-results card">
          <div class="hs-head">
            <div class="hs-head-left">
              <span class="hs-title">"{homeActiveQuery}" Arama Sonuçları</span>
              <span class="hs-count muted">
                {homeSongResults.length} şarkı, {homePlaylistResults.length} liste
              </span>
            </div>
            <button
              type="button"
              class="btn sm hs-close-btn"
              onclick={clearHomeSearch}
            >
              <Icon name="close" size={15} />
              Sonuçları Kapat
            </button>
          </div>

          {#if homeSearching}
            <div class="msg hs-loading">
              <p class="muted">Arama sonuçları getiriliyor...</p>
            </div>
          {:else if homeSearchError}
            <div class="msg error">
              <p>{homeSearchError}</p>
            </div>
          {:else if homeSongResults.length === 0 && homePlaylistResults.length === 0 && !homeArtistResult}
            <div class="msg hs-empty">
              <p class="muted">"{homeActiveQuery}" için herhangi bir sonuç bulunamadı.</p>
            </div>
          {:else}
            <!-- Artist match -->
            {#if homeArtistResult}
              <div class="hs-artist-strip">
                <button
                  type="button"
                  class="hs-artist-card"
                  onclick={() => homeArtistResult && player.openArtist(homeArtistResult.name)}
                >
                  <div class="hs-artist-avatar">
                    {#if homeArtistResult.thumbnail}
                      <img src={homeArtistResult.thumbnail} alt="" draggable="false" />
                    {:else}
                      <Icon name="user" size={24} />
                    {/if}
                  </div>
                  <div class="hs-artist-info">
                    <span class="hs-artist-tag muted">SANATÇI PROFİLİ</span>
                    <span class="hs-artist-name">{homeArtistResult.name}</span>
                    {#if homeArtistResult.subscribers}
                      <span class="hs-artist-subs muted">{homeArtistResult.subscribers} abone</span>
                    {/if}
                  </div>
                  <span class="hs-artist-go">
                    <Icon name="next" size={16} />
                  </span>
                </button>
              </div>
            {/if}

            <!-- Songs -->
            {#if homeSongResults.length > 0}
              <div class="hs-section">
                <h3 class="hs-sec-title">Şarkılar</h3>
                <div class="hs-song-list">
                  {#each homeSongResults.slice(0, 10) as track (track.id)}
                    {@const isCurrent = player.state.track?.id === track.id}
                    {@const isFav = library.isFavorite(track.id)}
                    <div class="hs-song-row" class:active={isCurrent}>
                      <!-- svelte-ignore a11y_click_events_have_key_events -->
                      <!-- svelte-ignore a11y_no_static_element_interactions -->
                      <div
                        class="hs-song-hit"
                        onclick={() => player.playTrack(track, homeSongResults, `Arama: ${homeActiveQuery}`)}
                      >
                        <Cover {track} size="42px" radius="calc(var(--radius) * 0.5)" />
                        <div class="hs-song-meta">
                          <span class="hs-song-title truncate">{track.title}</span>
                          <span class="hs-song-artist truncate muted">
                            <button
                              type="button"
                              class="artist-inline-btn"
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
                        </div>
                        <span class="hs-song-dur muted">{track.durationMs ? formatTime(track.durationMs) : ''}</span>
                      </div>
                      <div class="hs-song-actions">
                        <button
                          type="button"
                          class="btn ghost icon sm"
                          title="Şarkı Radyosu Başlat"
                          aria-label="Şarkı Radyosu Başlat"
                          onclick={() => player.startRadio(track)}
                        >
                          <Icon name="radio" size={15} />
                        </button>
                        <button
                          type="button"
                          class="btn ghost icon sm fav"
                          class:on={isFav}
                          title={isFav ? 'Kitaplıktan Çıkar' : 'Kitaplığa Ekle'}
                          aria-label={isFav ? 'Kitaplıktan Çıkar' : 'Kitaplığa Ekle'}
                          onclick={() => library.toggle(track)}
                        >
                          <Icon name={isFav ? 'heart-fill' : 'heart'} size={15} />
                        </button>
                      </div>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}

            <!-- Playlists & Albums -->
            {#if homePlaylistResults.length > 0}
              <div class="hs-section">
                <h3 class="hs-sec-title">Çalma Listeleri & Albümler</h3>
                <div class="hs-pl-grid">
                  {#each homePlaylistResults.slice(0, 8) as pl (pl.playlistId)}
                    {@const plSaved = library.isPlaylistSaved(pl.playlistId)}
                    <div class="hs-pl-card">
                      <button
                        type="button"
                        class="hs-pl-hit"
                        onclick={() => openPlaylistView(pl)}
                      >
                        <div class="hs-pl-art" style:background-color={pl.color}>
                          {#if pl.thumbnail}
                            <img src={pl.thumbnail} alt="" draggable="false" />
                          {:else}
                            <Icon name="music" size={24} />
                          {/if}
                        </div>
                        <div class="hs-pl-info">
                          <span class="hs-pl-title truncate">{pl.title}</span>
                          <span class="hs-pl-sub truncate muted">{pl.subtitle}</span>
                        </div>
                      </button>
                      <button
                        type="button"
                        class="btn ghost icon hs-pl-save"
                        class:on={plSaved}
                        title={plSaved ? 'Kitaplıktan kaldır' : 'Kitaplığa kaydet'}
                        aria-label={plSaved ? 'Kitaplıktan kaldır' : 'Kitaplığa kaydet'}
                        onclick={() => library.togglePlaylist(pl)}
                      >
                        <Icon name={plSaved ? 'bookmark-fill' : 'bookmark'} size={16} />
                      </button>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}
          {/if}
        </section>
      {/if}
    {/if}

    {#if moodCategories.length > 0}
      <div class="mood-bar">
        <div class="mood-bar-title muted">
          <Icon name="sparkles" size={14} />
          <span>Ruh Hali & Türler</span>
        </div>
        <div
          class="mood-scroll"
          onwheel={(e) => {
            if (e.deltaY !== 0) {
              e.preventDefault();
              e.currentTarget.scrollLeft += e.deltaY;
            }
          }}
        >
          {#each moodCategories as cat (cat.params)}
            {@const isActive = selectedMood?.params === cat.params}
            <button
              type="button"
              class="mood-chip"
              class:active={isActive}
              onclick={() => selectMoodCategory(cat)}
            >
              {cat.title}
            </button>
          {/each}
        </div>
      </div>
    {/if}

    {#if selectedMood}
      <div class="section mood-section card">
        <div class="sec-head mood-sec-head">
          <div class="mood-head-title">
            <span class="badge">RUH HALİ & TÜR</span>
            <h2>{selectedMood.title}</h2>
            <span class="muted sec-sub">Özenle derlenmiş çalma listeleri ve miksler</span>
          </div>
          <button class="btn sm" onclick={clearMoodCategory}>
            <Icon name="close" size={14} />
            Filtreyi Temizle
          </button>
        </div>

        {#if loadingMood}
          <div class="card msg">
            <p class="muted">"{selectedMood.title}" çalma listeleri yükleniyor...</p>
          </div>
        {:else if moodPlaylists.length > 0}
          <div class="mix-grid">
            {#each moodPlaylists as pl (pl.playlistId)}
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
                    <span class="mix-tag">{selectedMood.title.toUpperCase()}</span>
                    <button
                      type="button"
                      class="mix-save-btn"
                      class:on={plSaved}
                      title={plSaved ? 'Kitaplıktan kaldır' : 'Kitaplığa kaydet'}
                      aria-label={plSaved ? 'Kitaplıktan kaldır' : 'Kitaplığa kaydet'}
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
        {:else}
          <div class="card msg">
            <p class="muted">Bu kategori için parça listesi bulunamadı.</p>
          </div>
        {/if}
      </div>
    {/if}

    {#if error}
      <div class="card msg">
        <p>{error}</p>
      </div>
    {:else if loading}
      <div class="card msg">
        <p class="muted">YouTube Music akışı hazırlanıyor...</p>
      </div>
    {:else if tracks.length > 0}
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
                    <span class="mix-tag">
                      {pl.playlistId.startsWith('RD') ? 'MİKS' : 'LİSTE'}
                    </span>
                    <button
                      type="button"
                      class="mix-save-btn"
                      class:on={plSaved}
                      title={plSaved ? 'Kitaplıktan kaldır' : 'Kitaplığa kaydet'}
                      aria-label={plSaved ? 'Kitaplıktan kaldır' : 'Kitaplığa kaydet'}
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
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div
                  class="quick-hit"
                  onclick={() => player.playTrack(track, tracks, 'Hızlı Seçimler')}
                >
                  <Cover {track} size="52px" radius="var(--radius-sm)" />
                  <span class="q-info">
                    <span class="q-title truncate">{track.title}</span>
                    <button
                      type="button"
                      class="artist-inline-btn truncate muted"
                      title="{track.artist} Sanatçı Sayfasını Aç"
                      onclick={(e) => {
                        e.stopPropagation();
                        player.openArtist(track.artist);
                      }}
                    >
                      {track.artist}
                    </button>
                  </span>
                  <span class="q-dur muted">
                    {track.durationMs ? formatTime(track.durationMs) : ''}
                  </span>
                </div>
                <button
                  class="btn ghost icon q-radio"
                  title="Şarkı Radyosu Başlat"
                  aria-label="Şarkı Radyosu Başlat"
                  onclick={() => player.startRadio(track)}
                >
                  <Icon name="radio" size={16} />
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
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div
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
                  <button
                    type="button"
                    class="artist-inline-btn truncate muted"
                    title="{track.artist} Sanatçı Sayfasını Aç"
                    onclick={(e) => {
                      e.stopPropagation();
                      player.openArtist(track.artist);
                    }}
                  >
                    {track.artist}
                  </button>
                </div>
                <button
                  class="btn ghost icon shelf-radio-btn"
                  title="Şarkı Radyosu Başlat"
                  aria-label="Şarkı Radyosu Başlat"
                  onclick={() => player.startRadio(track)}
                >
                  <Icon name="radio" size={15} />
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

  /* Mood Section Badge */
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

  /* Playlist Detail View */
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
  .hero-body {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-width: 0;
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
  .shelf-radio-btn {
    position: absolute;
    top: 16px;
    right: 54px;
    width: 32px;
    height: 32px;
    min-height: 32px;
    border-radius: 50%;
    background: var(--surface);
    border: 2px solid var(--border);
    opacity: 0;
    transition: opacity 120ms ease;
  }
  .item:hover .shelf-radio-btn,
  .shelf-radio-btn:focus-visible {
    opacity: 1;
  }
  .q-radio,
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
  .mood-bar {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px 14px;
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }
  .mood-bar-title {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  .mood-scroll {
    display: flex;
    gap: 8px;
    overflow-x: auto;
    padding-bottom: 2px;
    scrollbar-width: none;
    -ms-overflow-style: none;
  }
  .mood-scroll::-webkit-scrollbar {
    display: none;
  }
  .mood-chip {
    flex-shrink: 0;
    display: inline-flex;
    align-items: center;
    height: 32px;
    padding: 0 12px;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    border: 2px solid var(--border);
    color: var(--text);
    font-size: 12.5px;
    font-weight: 700;
    cursor: pointer;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    transition: transform 70ms ease, background-color 100ms ease, border-color 100ms ease;
  }
  .mood-chip:hover {
    background: var(--surface-3);
    border-color: var(--accent);
  }
  .mood-chip:active {
    transform: translateY(var(--edge-depth));
    box-shadow: none;
  }
  .mood-chip.active {
    background: var(--accent);
    color: var(--on-accent);
    border-color: var(--accent-edge);
    box-shadow: 0 var(--edge-depth) 0 var(--accent-edge);
  }
  .mood-section {
    padding: 16px;
    border-color: var(--accent);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }
  .mood-sec-head {
    margin-bottom: 8px;
  }
  .mood-head-title {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  /* Home search bar */
  .home-search-wrap {
    width: 100%;
  }
  .home-search-form {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 10px 6px 14px;
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius-lg);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    transition: border-color 0.15s ease;
  }
  .home-search-form:focus-within {
    border-color: var(--accent);
  }
  .home-search-icon {
    color: var(--text-muted);
    display: flex;
    flex-shrink: 0;
  }
  .home-search-input {
    flex: 1;
    border: 0;
    background: transparent;
    color: var(--text);
    font: inherit;
    font-size: 14.5px;
    outline: none;
    min-width: 0;
  }
  .home-search-clear-btn {
    padding: 6px;
    color: var(--text-muted);
  }
  .home-search-btn {
    padding: 6px 18px;
    font-size: 13.5px;
    font-weight: 700;
    flex-shrink: 0;
  }

  /* Home search results panel */
  .home-search-results {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 16px;
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius-lg);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }
  .hs-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    border-bottom: 1px solid var(--border);
    padding-bottom: 10px;
  }
  .hs-head-left {
    display: flex;
    align-items: baseline;
    gap: 10px;
    flex-wrap: wrap;
  }
  .hs-title {
    font-size: 16px;
    font-weight: 800;
  }
  .hs-count {
    font-size: 12px;
  }
  .hs-close-btn {
    gap: 6px;
    font-weight: 700;
    cursor: pointer;
  }
  .hs-loading,
  .hs-empty {
    padding: 16px 0;
  }
  .hs-artist-strip {
    width: 100%;
  }
  .hs-artist-card {
    display: flex;
    align-items: center;
    gap: 14px;
    width: 100%;
    padding: 10px 14px;
    background: var(--bg-hover);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    text-align: left;
    cursor: pointer;
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    transition: all 0.15s ease;
  }
  .hs-artist-card:hover {
    border-color: var(--accent);
    transform: translateY(-1px);
  }
  .hs-artist-avatar {
    width: 44px;
    height: 44px;
    border-radius: 50%;
    overflow: hidden;
    background: var(--surface);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }
  .hs-artist-avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .hs-artist-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
    min-width: 0;
  }
  .hs-artist-tag {
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 0.05em;
  }
  .hs-artist-name {
    font-size: 15px;
    font-weight: 800;
  }
  .hs-artist-subs {
    font-size: 11px;
  }
  .hs-artist-go {
    color: var(--text-muted);
  }
  .hs-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .hs-sec-title {
    font-size: 13px;
    font-weight: 800;
    letter-spacing: 0.04em;
    color: var(--text-muted);
    text-transform: uppercase;
  }
  .hs-song-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .hs-song-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding: 6px 10px;
    border-radius: var(--radius-sm);
    background: var(--bg);
    border: 1px solid var(--border);
    transition: all 0.12s ease;
  }
  .hs-song-row:hover,
  .hs-song-row.active {
    background: var(--bg-hover);
    border-color: var(--accent);
  }
  .hs-song-hit {
    display: flex;
    align-items: center;
    gap: 12px;
    flex: 1;
    min-width: 0;
    cursor: pointer;
  }
  .hs-song-meta {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
    min-width: 0;
  }
  .hs-song-title {
    font-size: 13.5px;
    font-weight: 700;
  }
  .hs-song-artist {
    font-size: 12px;
    display: flex;
    align-items: center;
    gap: 4px;
    flex-wrap: wrap;
  }
  .hs-song-dur {
    font-size: 12px;
    font-variant-numeric: tabular-nums;
    margin-right: 4px;
  }
  .hs-song-actions {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
  }
  .hs-pl-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 10px;
  }
  .hs-pl-card {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    transition: all 0.12s ease;
    min-width: 0;
  }
  .hs-pl-card:hover {
    border-color: var(--accent);
    transform: translateY(-1px);
    background: var(--bg-hover);
  }
  .hs-pl-hit {
    flex: 1;
    min-width: 0;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 2px;
    border: 0;
    background: transparent;
    color: inherit;
    font: inherit;
    text-align: left;
    cursor: pointer;
  }
  .hs-pl-save {
    width: 32px;
    height: 32px;
    min-height: 32px;
    flex-shrink: 0;
  }
  .hs-pl-art {
    width: 44px;
    height: 44px;
    border-radius: calc(var(--radius) * 0.4);
    overflow: hidden;
    background: var(--surface);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }
  .hs-pl-art img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .hs-pl-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1;
  }
  .hs-pl-title {
    font-size: 13px;
    font-weight: 700;
  }
  .hs-pl-sub {
    font-size: 11px;
  }

  @media (max-width: 768px) {
    .view {
      gap: 16px;
    }
    .head {
      flex-direction: column;
      align-items: stretch;
      gap: 12px;
    }
    .head-actions {
      justify-content: flex-start;
      width: 100%;
    }
    .mix-grid {
      grid-template-columns: 1fr;
    }
    .quick-grid {
      grid-template-columns: 1fr;
    }
    .grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
      gap: 10px;
    }
    .item {
      padding: 8px;
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
    .hs-head {
      flex-direction: column;
      align-items: flex-start;
      gap: 8px;
    }
    .hs-close-btn {
      align-self: flex-end;
    }
    .hs-pl-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
