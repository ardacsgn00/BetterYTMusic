<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { theme } from '#lib/theme.svelte.ts';
  import { player } from '#lib/player.svelte.ts';
  import { library } from '#lib/library.svelte.ts';

  let { children } = $props();

  onMount(() => {
    theme.init();
    library.init();
    player.init().catch((e) => console.error('player init failed', e));
    return () => player.destroy();
  });
</script>

{@render children()}
