<script lang="ts">
  import Sidebar from '#lib/components/Sidebar.svelte';
  import NowPlaying from '#lib/components/NowPlaying.svelte';
  import Queue from '#lib/components/Queue.svelte';
  import LyricsPanel from '#lib/components/LyricsPanel.svelte';
  import PlayerBar from '#lib/components/PlayerBar.svelte';
  import ThemePanel from '#lib/components/ThemePanel.svelte';
  import SearchView from '#lib/components/SearchView.svelte';
  import HomeView from '#lib/components/HomeView.svelte';
  import LibraryView from '#lib/components/LibraryView.svelte';
  import { player } from '#lib/player.svelte.ts';
  import type { View } from '#lib/types.ts';

  let view = $state<View>('now');
  let threeCol = $state(false);

  // In-app shortcuts (global media keys are handled natively in Rust).
  function onKeydown(e: KeyboardEvent) {
    const t = e.target as HTMLElement | null;
    if (t && (t.tagName === 'INPUT' || t.tagName === 'TEXTAREA' || t.isContentEditable)) {
      if ((t as HTMLInputElement).type !== 'range') return;
    }
    if (e.code === 'Space') {
      e.preventDefault();
      player.toggle();
    } else if (e.key === 'ArrowRight' && e.ctrlKey) {
      e.preventDefault();
      player.next();
    } else if (e.key === 'ArrowLeft' && e.ctrlKey) {
      e.preventDefault();
      player.previous();
    } else if (e.key === 'ArrowRight' && !e.altKey && t?.tagName !== 'INPUT') {
      player.seekBy(5000);
    } else if (e.key === 'ArrowLeft' && !e.altKey && t?.tagName !== 'INPUT') {
      player.seekBy(-5000);
    }
  }
</script>

<svelte:window onkeydown={onKeydown} />

<div class="shell">
  <Sidebar active={view} onselect={(v) => (view = v)} />

  <main>
    {#if view === 'now'}
      <div class="now-grid" class:three-col={threeCol}>
        <NowPlaying />
        {#if threeCol}
          <LyricsPanel {threeCol} ontogglecolumns={() => (threeCol = !threeCol)} />
        {/if}
        <Queue {threeCol} ontogglecolumns={() => (threeCol = !threeCol)} />
      </div>
    {:else if view === 'home'}
      <HomeView />
    {:else if view === 'search'}
      <SearchView />
    {:else if view === 'library'}
      <LibraryView />
    {:else if view === 'theme'}
      <ThemePanel />
    {/if}
  </main>

  <PlayerBar />
</div>

<style>
  .shell {
    height: 100%;
    display: grid;
    grid-template-columns: var(--sidebar-w) minmax(0, 1fr);
    grid-template-rows: minmax(0, 1fr) auto;
    grid-template-areas:
      'side main'
      'bar bar';
  }
  .shell > :global(.sidebar) {
    grid-area: side;
  }
  main {
    grid-area: main;
    min-height: 0;
    overflow: auto;
    padding: 24px;
    display: flex;
    justify-content: center;
  }
  /* Wide screens: cover and queue side by side, using the full width. */
  .now-grid {
    width: 100%;
    max-width: 1680px;
    height: 100%;
    display: grid;
    grid-template-columns: minmax(320px, 1.1fr) minmax(320px, 1fr);
    gap: 22px;
    min-height: 0;
  }
  .now-grid.three-col {
    grid-template-columns: minmax(290px, 1fr) minmax(320px, 1.1fr) minmax(290px, 1fr);
  }

  @media (max-width: 1120px) {
    .now-grid.three-col {
      grid-template-columns: minmax(320px, 1.1fr) minmax(320px, 1fr);
    }
    .now-grid.three-col > :global(.lyrics-wrap) {
      display: none;
    }
  }

  @media (max-width: 1000px) {
    .now-grid {
      grid-template-columns: 1fr;
      grid-template-rows: auto minmax(320px, 1fr);
      height: auto;
      min-height: 100%;
    }
  }

  @media (max-width: 760px) {
    .shell {
      grid-template-columns: minmax(0, 1fr);
      grid-template-rows: auto minmax(0, 1fr) auto;
      grid-template-areas:
        'side'
        'main'
        'bar';
    }
    main {
      padding: 12px;
    }
    .now-grid {
      gap: 12px;
    }
  }
</style>
