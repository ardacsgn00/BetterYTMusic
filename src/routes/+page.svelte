<script lang="ts">
  import { onMount } from 'svelte';
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
  import { theme } from '#lib/theme.svelte.ts';
  import type { View } from '#lib/types.ts';

  let view = $state<View>('now');
  let threeCol = $state(false);
  let nowGridEl = $state<HTMLElement | null>(null);
  let draggingHandle = $state<'2col' | '3left' | '3right' | null>(null);

  function selectView(v: View) {
    view = v;
    if (v !== 'theme') {
      theme.setLastView(v);
    }
  }

  onMount(() => {
    theme.init();
    view = theme.initialView;
  });

  // In-app shortcuts (global media keys are handled natively in Rust).
  function onKeydown(e: KeyboardEvent) {
    // Chrome-style zoom shortcuts: Ctrl + / Ctrl - / Ctrl 0
    if (e.ctrlKey || e.metaKey) {
      if (e.key === '+' || e.key === '=') {
        e.preventDefault();
        theme.stepZoom(5);
        return;
      } else if (e.key === '-' || e.key === '_') {
        e.preventDefault();
        theme.stepZoom(-5);
        return;
      } else if (e.key === '0') {
        e.preventDefault();
        theme.setZoom(100);
        return;
      }
    }

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

  function onWheel(e: WheelEvent) {
    if (e.ctrlKey) {
      e.preventDefault();
      theme.stepZoom(e.deltaY < 0 ? 5 : -5);
    }
  }

  function startDrag(which: '2col' | '3left' | '3right', e: PointerEvent) {
    e.preventDefault();
    draggingHandle = which;
    (e.currentTarget as HTMLElement)?.setPointerCapture(e.pointerId);
  }

  function onPointerMove(e: PointerEvent) {
    if (!draggingHandle || !nowGridEl) return;
    const rect = nowGridEl.getBoundingClientRect();
    if (rect.width <= 0) return;
    const relPct = ((e.clientX - rect.left) / rect.width) * 100;

    if (draggingHandle === '2col') {
      theme.setSplit2Left(relPct);
    } else if (draggingHandle === '3left') {
      theme.setSplit3Left(relPct);
    } else if (draggingHandle === '3right') {
      theme.setSplit3Right(100 - relPct);
    }
  }

  function stopDrag() {
    draggingHandle = null;
  }

  const gridColsStyle = $derived.by(() => {
    if (threeCol) {
      const l = theme.split3Left;
      const r = theme.split3Right;
      const m = Math.max(24, 100 - l - r);
      return `minmax(250px, ${l}fr) 12px minmax(260px, ${m}fr) 12px minmax(250px, ${r}fr)`;
    }
    const l = theme.split2Left;
    const r = 100 - l;
    return `minmax(270px, ${l}fr) 12px minmax(270px, ${r}fr)`;
  });
</script>

<svelte:window onkeydown={onKeydown} onwheel={{ handleEvent: onWheel, passive: false } as any} />

<div class="shell" class:is-resizing={draggingHandle !== null}>
  <Sidebar active={view} onselect={selectView} />

  <main style:zoom={`${theme.zoom}%`}>
    {#if view === 'now'}
      <div
        class="now-grid"
        class:three-col={threeCol}
        bind:this={nowGridEl}
        style:grid-template-columns={gridColsStyle}
      >
        <NowPlaying />

        {#if threeCol}
          <!-- Left splitter: between Cover and Lyrics -->
          <div
            class="splitter"
            class:active={draggingHandle === '3left'}
            role="separator"
            aria-orientation="vertical"
            title="Genişliği ayarlamak için sürükle (Çift tıkla: Sıfırla)"
            onpointerdown={(e) => startDrag('3left', e)}
            onpointermove={onPointerMove}
            onpointerup={stopDrag}
            onpointercancel={stopDrag}
            ondblclick={() => theme.resetSplits()}
          >
            <span class="grip"></span>
          </div>

          <LyricsPanel {threeCol} ontogglecolumns={() => (threeCol = !threeCol)} />

          <!-- Right splitter: between Lyrics and Queue -->
          <div
            class="splitter"
            class:active={draggingHandle === '3right'}
            role="separator"
            aria-orientation="vertical"
            title="Genişliği ayarlamak için sürükle (Çift tıkla: Sıfırla)"
            onpointerdown={(e) => startDrag('3right', e)}
            onpointermove={onPointerMove}
            onpointerup={stopDrag}
            onpointercancel={stopDrag}
            ondblclick={() => theme.resetSplits()}
          >
            <span class="grip"></span>
          </div>

          <Queue {threeCol} ontogglecolumns={() => (threeCol = !threeCol)} />
        {:else}
          <!-- 2-col splitter: between Cover and Queue/Lyrics -->
          <div
            class="splitter"
            class:active={draggingHandle === '2col'}
            role="separator"
            aria-orientation="vertical"
            title="Genişliği ayarlamak için sürükle (Çift tıkla: Sıfırla)"
            onpointerdown={(e) => startDrag('2col', e)}
            onpointermove={onPointerMove}
            onpointerup={stopDrag}
            onpointercancel={stopDrag}
            ondblclick={() => theme.resetSplits()}
          >
            <span class="grip"></span>
          </div>

          <Queue {threeCol} ontogglecolumns={() => (threeCol = !threeCol)} />
        {/if}
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
  .shell.is-resizing {
    cursor: col-resize;
    user-select: none;
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
  /* Wide screens: cover, lyrics, and queue side by side with draggable splitters. */
  .now-grid {
    width: 100%;
    max-width: 1720px;
    height: 100%;
    display: grid;
    gap: 6px;
    min-height: 0;
  }

  .splitter {
    position: relative;
    width: 12px;
    height: 100%;
    cursor: col-resize;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 6px;
    touch-action: none;
    transition: background-color 120ms ease;
  }
  .splitter:hover,
  .splitter.active {
    background: var(--accent-soft);
  }
  .grip {
    width: 4px;
    height: 44px;
    border-radius: 4px;
    background: var(--border);
    transition:
      background-color 120ms ease,
      height 120ms ease;
  }
  .splitter:hover .grip,
  .splitter.active .grip {
    background: var(--accent);
    height: 64px;
  }

  @media (max-width: 1120px) {
    .now-grid.three-col {
      grid-template-columns: minmax(280px, 1.1fr) 12px minmax(280px, 1fr) !important;
    }
    .now-grid.three-col > :global(.lyrics-wrap),
    .now-grid.three-col > .splitter:first-of-type {
      display: none;
    }
  }

  @media (max-width: 1000px) {
    .now-grid {
      grid-template-columns: 1fr !important;
      grid-template-rows: auto minmax(320px, 1fr);
      gap: 16px;
      height: auto;
      min-height: 100%;
    }
    .splitter {
      display: none !important;
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
