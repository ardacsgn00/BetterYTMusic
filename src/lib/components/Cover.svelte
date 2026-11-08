<script lang="ts">
  import type { Track } from '#lib/types.ts';

  let { track, size = '100%', radius = 'var(--radius)' }: { track: Track | null; size?: string; radius?: string } =
    $props();

  let failed = $state(false);
  $effect(() => {
    // Reset error state when the track changes.
    track?.id;
    failed = false;
  });

  const initial = $derived(track?.title.trim().charAt(0).toUpperCase() ?? '♪');
</script>

<div
  class="cover"
  style:width={size}
  style:border-radius={radius}
  style:background-color={track?.color ?? 'var(--surface-3)'}
>
  {#if track?.thumbnail && !failed}
    <img src={track.thumbnail} alt="" loading="lazy" draggable="false" onerror={() => (failed = true)} />
  {:else}
    <span class="initial">{initial}</span>
  {/if}
</div>

<style>
  .cover {
    position: relative;
    aspect-ratio: 1;
    flex-shrink: 0;
    overflow: hidden;
    display: grid;
    place-items: center;
    container-type: inline-size;
  }
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .initial {
    font-size: 46cqi;
    font-weight: 800;
    color: rgb(0 0 0 / 0.28);
    line-height: 1;
  }
</style>
