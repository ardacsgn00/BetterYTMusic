<script lang="ts">
  import { player } from '#lib/player.svelte.ts';
  import { library } from '#lib/library.svelte.ts';
  import { formatTime } from '#lib/format.ts';
  import type { Track, PlaylistCard, ArtistData } from '#lib/types.ts';
  import Cover from './Cover.svelte';
  import Icon from './Icon.svelte';

  let {
    artist,
    onclose,
    onopenplaylist
  }: {
    artist: ArtistData;
    onclose: () => void;
    onopenplaylist?: (pl: PlaylistCard) => void;
  } = $props();

  let showFullDesc = $state(false);

  function playTopAll() {
    if (artist.topTracks.length === 0) return;
    player.playTrack(artist.topTracks[0], artist.topTracks, `${artist.name} · Popüler Şarkılar`);
  }

  function startArtistRadio() {
    if (artist.topTracks.length > 0) {
      player.startRadio(artist.topTracks[0]);
    }
  }
</script>

<div class="artist-view card">
  <div class="artist-toolbar">
    <button class="btn back-btn" onclick={onclose}>
      <Icon name="back" size={16} />
      <span>Geri Dön</span>
    </button>
  </div>

  <!-- HERO BANNER -->
  <header class="artist-hero">
    <div class="artist-avatar">
      {#if artist.thumbnail}
        <img src={artist.thumbnail} alt={artist.name} draggable="false" />
      {:else}
        <div class="avatar-fallback">
          <Icon name="user" size={48} />
        </div>
      {/if}
    </div>

    <div class="artist-meta">
      <span class="badge">SANATÇI</span>
      <h1 class="artist-name">{artist.name}</h1>
      <div class="artist-stats muted">
        {#if artist.subscribers}
          <span>{artist.subscribers} abone</span>
        {/if}
        {#if artist.monthlyListeners}
          <span>· {artist.monthlyListeners}</span>
        {/if}
      </div>

      <div class="artist-actions">
        <button
          class="btn accent hero-btn"
          disabled={artist.topTracks.length === 0}
          onclick={playTopAll}
        >
          <Icon name="play" size={16} />
          Tümünü Çal
        </button>
        <button
          class="btn hero-btn"
          disabled={artist.topTracks.length === 0}
          title="Sanatçı Radyosunu Başlat"
          onclick={startArtistRadio}
        >
          <Icon name="radio" size={16} />
          Sanatçı Radyosu
        </button>
      </div>
    </div>
  </header>

  <!-- BIO (IF AVAILABLE) -->
  {#if artist.description}
    <div class="artist-bio-card">
      <p class="bio-text" class:clamped={!showFullDesc}>
        {artist.description}
      </p>
      {#if artist.description.length > 180}
        <button class="bio-toggle muted" onclick={() => (showFullDesc = !showFullDesc)}>
          {showFullDesc ? 'Daha az göster' : 'Devamını oku...'}
        </button>
      {/if}
    </div>
  {/if}

  <!-- TOP TRACKS -->
  {#if artist.topTracks.length > 0}
    <section class="section">
      <div class="section-head">
        <h2>En Çok Dinlenen Şarkılar</h2>
      </div>

      <div class="tracklist">
        <ol class="track-ol">
          {#each artist.topTracks as track, idx (track.id)}
            {@const active = player.state.track?.id === track.id}
            {@const liked = library.isFavorite(track.id)}
            <li class="track-row" class:active>
              <span class="t-idx muted">{idx + 1}</span>
              <button
                class="track-hit"
                onclick={() => player.playTrack(track, artist.topTracks, `${artist.name} · Popüler`)}
              >
                <Cover {track} size="42px" radius="calc(var(--radius) * 0.5)" />
                <span class="t-info">
                  <span class="t-title truncate">{track.title}</span>
                  <span class="t-sub truncate muted">{track.artist}{track.album ? ` · ${track.album}` : ''}</span>
                </span>
                <span class="t-dur muted">{track.durationMs ? formatTime(track.durationMs) : ''}</span>
              </button>

              <button
                class="btn ghost icon radio-btn"
                title="Şarkı Radyosu Başlat"
                aria-label="Şarkı Radyosu Başlat"
                onclick={() => player.startRadio(track)}
              >
                <Icon name="radio" size={16} />
              </button>

              <button
                class="btn ghost icon fav-btn"
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
    </section>
  {/if}

  <!-- ALBUMS -->
  {#if artist.albums.length > 0}
    <section class="section">
      <div class="section-head">
        <h2>Albümler</h2>
      </div>
      <div class="cards-grid">
        {#each artist.albums as album (album.playlistId)}
          {@const plSaved = library.isPlaylistSaved(album.playlistId)}
          <div class="pl-card card">
            <button
              class="pl-card-btn"
              onclick={() => onopenplaylist ? onopenplaylist(album) : player.playPlaylist(album.playlistId, album.title)}
            >
              <div class="pl-card-art" style:background-color={album.color}>
                {#if album.thumbnail}
                  <img src={album.thumbnail} alt="" loading="lazy" draggable="false" />
                {:else}
                  <Icon name="disc" size={28} />
                {/if}
                <div class="pl-card-overlay">
                  <span class="pl-play-icon"><Icon name="play" size={18} /></span>
                </div>
              </div>
              <div class="pl-card-info">
                <span class="pl-card-title truncate" title={album.title}>{album.title}</span>
                <span class="pl-card-sub truncate muted" title={album.subtitle}>{album.subtitle}</span>
              </div>
            </button>
            <div class="pl-card-footer">
              <button
                type="button"
                class="btn sm pl-save-btn"
                class:accent={plSaved}
                title={plSaved ? 'Kitaplıktan Çıkar' : 'Kitaplığa Kaydet'}
                onclick={() => library.togglePlaylist(album)}
              >
                <Icon name={plSaved ? 'bookmark-fill' : 'bookmark'} size={14} />
                <span>{plSaved ? 'Kaydedildi' : 'Kaydet'}</span>
              </button>
            </div>
          </div>
        {/each}
      </div>
    </section>
  {/if}

  <!-- SINGLES & EPS -->
  {#if artist.singles.length > 0}
    <section class="section">
      <div class="section-head">
        <h2>Single'lar ve EP'ler</h2>
      </div>
      <div class="cards-grid">
        {#each artist.singles as single (single.playlistId)}
          {@const plSaved = library.isPlaylistSaved(single.playlistId)}
          <div class="pl-card card">
            <button
              class="pl-card-btn"
              onclick={() => onopenplaylist ? onopenplaylist(single) : player.playPlaylist(single.playlistId, single.title)}
            >
              <div class="pl-card-art" style:background-color={single.color}>
                {#if single.thumbnail}
                  <img src={single.thumbnail} alt="" loading="lazy" draggable="false" />
                {:else}
                  <Icon name="music" size={28} />
                {/if}
                <div class="pl-card-overlay">
                  <span class="pl-play-icon"><Icon name="play" size={18} /></span>
                </div>
              </div>
              <div class="pl-card-info">
                <span class="pl-card-title truncate" title={single.title}>{single.title}</span>
                <span class="pl-card-sub truncate muted" title={single.subtitle}>{single.subtitle}</span>
              </div>
            </button>
            <div class="pl-card-footer">
              <button
                type="button"
                class="btn sm pl-save-btn"
                class:accent={plSaved}
                title={plSaved ? 'Kitaplıktan Çıkar' : 'Kitaplığa Kaydet'}
                onclick={() => library.togglePlaylist(single)}
              >
                <Icon name={plSaved ? 'bookmark-fill' : 'bookmark'} size={14} />
                <span>{plSaved ? 'Kaydedildi' : 'Kaydet'}</span>
              </button>
            </div>
          </div>
        {/each}
      </div>
    </section>
  {/if}

  <!-- SIMILAR ARTISTS -->
  {#if artist.similarArtists.length > 0}
    <section class="section">
      <div class="section-head">
        <h2>Benzer Sanatçılar</h2>
      </div>
      <div class="artists-grid">
        {#each artist.similarArtists as sim (sim.channelId)}
          <button class="sim-card card" onclick={() => player.openArtist(sim.channelId)}>
            <div class="sim-avatar">
              {#if sim.thumbnail}
                <img src={sim.thumbnail} alt="" loading="lazy" draggable="false" />
              {:else}
                <Icon name="user" size={24} />
              {/if}
            </div>
            <span class="sim-name truncate" title={sim.name}>{sim.name}</span>
            {#if sim.subscribers}
              <span class="sim-subs truncate muted">{sim.subscribers}</span>
            {/if}
          </button>
        {/each}
      </div>
    </section>
  {/if}
</div>

<style>
  .artist-view {
    display: flex;
    flex-direction: column;
    gap: 22px;
    padding: 22px;
    width: 100%;
    max-width: 1300px;
  }
  .artist-toolbar {
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

  /* Hero */
  .artist-hero {
    display: flex;
    align-items: center;
    gap: 24px;
  }
  .artist-avatar {
    width: 140px;
    height: 140px;
    border-radius: 50%;
    overflow: hidden;
    flex-shrink: 0;
    border: 3px solid var(--border);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
    background: var(--surface-3);
    display: grid;
    place-items: center;
  }
  .artist-avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .avatar-fallback {
    color: var(--text-muted);
  }
  .artist-meta {
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
  .artist-name {
    font-size: 28px;
    font-weight: 800;
    line-height: 1.15;
    margin: 0;
  }
  .artist-stats {
    font-size: 13px;
  }
  .artist-actions {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 6px;
  }
  .hero-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    font-weight: 700;
  }

  /* Bio */
  .artist-bio-card {
    background: var(--surface-2);
    border: 1.5px solid var(--border);
    border-radius: var(--radius);
    padding: 14px 16px;
  }
  .bio-text {
    font-size: 13px;
    line-height: 1.55;
    margin: 0;
  }
  .bio-text.clamped {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  .bio-toggle {
    background: none;
    border: none;
    font-size: 12px;
    font-weight: 700;
    cursor: pointer;
    padding: 6px 0 0;
  }
  .bio-toggle:hover {
    color: var(--accent);
  }

  /* Sections */
  .section {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .section-head h2 {
    font-size: 18px;
    font-weight: 800;
    margin: 0;
  }

  /* Tracklist */
  .track-ol {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .track-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px 8px;
    border-radius: var(--radius-sm);
    transition: background 0.1s ease;
  }
  .track-row:hover {
    background: var(--surface-2);
  }
  .track-row.active {
    background: color-mix(in srgb, var(--accent) 18%, transparent);
  }
  .t-idx {
    font-size: 12px;
    font-variant-numeric: tabular-nums;
    width: 22px;
    text-align: center;
    flex-shrink: 0;
  }
  .track-hit {
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
  .t-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .t-title {
    font-size: 13.5px;
    font-weight: 700;
  }
  .t-sub {
    font-size: 11.5px;
  }
  .t-dur {
    font-size: 12px;
    font-variant-numeric: tabular-nums;
    margin-right: 6px;
  }
  .fav-btn.on {
    color: var(--accent);
  }
  .radio-btn {
    color: var(--text-muted);
  }
  .radio-btn:hover {
    color: var(--accent);
  }

  /* Grid of Albums & Singles */
  .cards-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(170px, 1fr));
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
    background: var(--surface-3);
  }
  .pl-card-art img {
    width: 100%;
    height: 100%;
    object-fit: cover;
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
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--accent);
    color: #fff;
    display: grid;
    place-items: center;
    box-shadow: 0 3px 6px rgba(0, 0, 0, 0.4);
  }
  .pl-card-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .pl-card-title {
    font-size: 13px;
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
  .pl-save-btn {
    width: 100%;
    font-size: 11.5px;
    font-weight: 700;
    min-height: 30px;
    padding: 4px 8px;
    gap: 5px;
    justify-content: center;
  }

  /* Similar Artists */
  .artists-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 12px;
  }
  .sim-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 8px;
    padding: 12px;
    background: var(--surface-2);
    border: 1.5px solid var(--border);
    cursor: pointer;
    transition: transform 0.12s ease, border-color 0.12s ease;
  }
  .sim-card:hover {
    border-color: var(--accent);
    transform: translateY(-2px);
  }
  .sim-avatar {
    width: 76px;
    height: 76px;
    border-radius: 50%;
    overflow: hidden;
    background: var(--surface-3);
    border: 2px solid var(--border);
    display: grid;
    place-items: center;
  }
  .sim-avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .sim-name {
    font-size: 13px;
    font-weight: 700;
    width: 100%;
  }
  .sim-subs {
    font-size: 11px;
    width: 100%;
  }

  @media (max-width: 768px) {
    .artist-hero {
      flex-direction: column;
      align-items: flex-start;
    }
    .artist-avatar {
      width: 100px;
      height: 100px;
    }
    .cards-grid {
      grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
    }
    .artists-grid {
      grid-template-columns: repeat(auto-fill, minmax(110px, 1fr));
    }
  }
</style>

