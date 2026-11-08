import { invoke } from '@tauri-apps/api/core';
import { theme, type MiniLayout } from './theme.svelte';

type MiniPos = { x: number; y: number };

/**
 * Mini player mode. The main window itself turns into a small borderless,
 * always-on-top widget (handled in Rust, `miniwin.rs`), so no extra WebView
 * or state sync is needed.
 */
class MiniStore {
  active = $state(false);
  onTop = $state(true);
  busy = $state(false);

  enter = async () => {
    if (this.active || this.busy) return;
    this.busy = true;
    try {
      // Switch the UI first so the shrinking window never shows the full layout squeezed.
      this.active = true;
      this.onTop = true;
      await invoke('window_enter_mini', { layout: theme.miniLayout, pos: theme.miniPos });
    } catch (e) {
      console.error('Mini oynatıcıya geçilemedi:', e);
      this.active = false;
    } finally {
      this.busy = false;
    }
  };

  exit = async () => {
    if (!this.active || this.busy) return;
    this.busy = true;
    try {
      const last = await invoke<MiniPos | null>('window_exit_mini');
      if (last) theme.setMiniPos(last);
    } catch (e) {
      console.error('Normal pencereye dönülemedi:', e);
    } finally {
      this.active = false;
      this.busy = false;
    }
  };

  setLayout = async (layout: MiniLayout) => {
    theme.setMiniLayout(layout);
    if (this.active) {
      try {
        await invoke('window_set_mini_layout', { layout });
      } catch {
        /* ignore */
      }
    }
  };

  toggleOnTop = async () => {
    const next = !this.onTop;
    this.onTop = next;
    try {
      await invoke('window_set_always_on_top', { on: next });
    } catch {
      this.onTop = !next;
    }
  };

  startDrag = () => invoke('window_start_drag').catch(() => {});
}

export const mini = new MiniStore();

