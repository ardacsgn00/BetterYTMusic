<script lang="ts">
  /**
   * Accessible slider: a native <input type="range"> laid transparently over a
   * custom solid track/fill. Emits `onpreview` while dragging, `oncommit` on release.
   */
  let {
    value,
    max,
    label,
    step = 1,
    onpreview,
    oncommit
  }: {
    value: number;
    max: number;
    label: string;
    step?: number;
    onpreview?: (v: number) => void;
    oncommit: (v: number) => void;
  } = $props();

  let dragging = $state(false);
  let dragValue = $state(0);

  const shown = $derived(dragging ? dragValue : value);
  const pct = $derived(max > 0 ? Math.min(100, (shown / max) * 100) : 0);

  function onInput(e: Event) {
    dragging = true;
    dragValue = Number((e.currentTarget as HTMLInputElement).value);
    onpreview?.(dragValue);
  }

  function onChange(e: Event) {
    const v = Number((e.currentTarget as HTMLInputElement).value);
    dragging = false;
    oncommit(v);
  }
</script>

<div class="slider" class:dragging style:--pct="{pct}%">
  <div class="track"><div class="fill"></div></div>
  <div class="thumb"></div>
  <input
    type="range"
    min="0"
    {max}
    {step}
    value={shown}
    aria-label={label}
    oninput={onInput}
    onchange={onChange}
    disabled={max <= 0}
  />
</div>

<style>
  .slider {
    position: relative;
    height: 20px;
    width: 100%;
    display: flex;
    align-items: center;
  }
  .track {
    width: 100%;
    height: 6px;
    border-radius: 6px;
    background: var(--surface-3);
    overflow: hidden;
  }
  .fill {
    height: 100%;
    width: var(--pct);
    background: var(--accent);
  }
  .thumb {
    position: absolute;
    left: var(--pct);
    top: 50%;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--text);
    border: 2px solid var(--accent-edge);
    transform: translate(-50%, -50%) scale(0);
    transition: transform 100ms ease;
    pointer-events: none;
  }
  .slider:hover .thumb,
  .slider.dragging .thumb,
  .slider:has(input:focus-visible) .thumb {
    transform: translate(-50%, -50%) scale(1);
  }
  input {
    position: absolute;
    inset: 0;
    width: 100%;
    margin: 0;
    opacity: 0;
    cursor: pointer;
  }
  input:disabled {
    cursor: default;
  }
</style>
