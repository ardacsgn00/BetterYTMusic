<script lang="ts">
  import Icon, { type IconName } from './Icon.svelte';
  import { player } from '#lib/player.svelte.ts';
  import { theme } from '#lib/theme.svelte.ts';
  import type { View } from '#lib/types.ts';

  let { active, onselect }: { active: View; onselect: (v: View) => void } = $props();

  const items: { id: View; label: string; icon: IconName }[] = [
    { id: 'now', label: 'Dinle', icon: 'now' },
    { id: 'home', label: 'Ana Sayfa', icon: 'home' },
    { id: 'search', label: 'Ara', icon: 'search' },
    { id: 'library', label: 'Kitaplık', icon: 'library' },
    { id: 'theme', label: 'Ayarlar', icon: 'settings' }
  ];

  const brandName = $derived(theme.appName.trim() || 'BetterYTMusic');
  let resizing = $state(false);

  function startResize(e: PointerEvent) {
    e.preventDefault();
    resizing = true;
    (e.currentTarget as HTMLElement)?.setPointerCapture(e.pointerId);
  }

  function onResizeMove(e: PointerEvent) {
    if (!resizing) return;
    theme.setSidebarWidth(e.clientX);
  }

  function stopResize() {
    resizing = false;
  }
</script>

<nav class="sidebar" class:collapsed={theme.sidebarCollapsed} class:resizing>
  <div class="brand" title={brandName}>
    <span class="logo"><Icon name="play" size={18} /></span>
    <span class="name truncate">{brandName}</span>
  </div>

  <ul>
    {#each items as item (item.id)}
      <li>
        <button
          class="nav-btn"
          class:active={active === item.id}
          aria-current={active === item.id ? 'page' : undefined}
          title={item.label}
          onclick={() => onselect(item.id)}
        >
          <Icon name={item.icon} size={20} />
          <span class="label">{item.label}</span>
        </button>
      </li>
    {/each}
  </ul>

  <div class="account">
    {#if player.auth.loggedIn}
      <div class="profile-card" title={player.auth.accountName ?? 'YouTube Music'}>
        <div class="avatar">
          <Icon name="user" size={18} />
        </div>
        <div class="profile-meta">
          <span class="acc-name truncate">{player.auth.accountName ?? 'YouTube Music'}</span>
          <span class="acc-sub truncate muted">Google Hesabı</span>
        </div>
        <button
          class="btn icon logout-btn"
          title="Hesaptan Çıkış Yap"
          aria-label="Hesaptan Çıkış Yap"
          onclick={player.logout}
        >
          <Icon name="logout" size={16} />
        </button>
      </div>
    {:else}
      <div class="profile-card logged-out">
        <div class="logged-out-top">
          <div class="avatar muted-av">
            <Icon name="user" size={17} />
          </div>
          <div class="profile-meta">
            <span class="acc-name truncate">Misafir Oturumu</span>
            <span class="acc-sub truncate muted">Kitaplık için giriş yap</span>
          </div>
        </div>
        <button class="btn accent full login-btn" title="Google ile Bağlan" onclick={player.login}>
          {#if theme.sidebarCollapsed}
            <Icon name="user" size={17} />
          {:else}
            Google ile Bağlan
          {/if}
        </button>
      </div>
    {/if}

    <div class="footer-row">
      <span class="muted">Masaüstü İstemci</span>
      <span class="ver-badge">v{__APP_VERSION__}</span>
    </div>
  </div>

  <!-- Draggable right-edge splitter for resizing or collapsing the sidebar -->
  <div
    class="side-resizer"
    class:active={resizing}
    role="separator"
    aria-orientation="vertical"
    title="Sol menü genişliğini ayarlamak için sürükle (İkon moduna geçmek için daralt veya çift tıkla)"
    onpointerdown={startResize}
    onpointermove={onResizeMove}
    onpointerup={stopResize}
    onpointercancel={stopResize}
    ondblclick={() => theme.toggleSidebarCollapsed()}
  >
    <span class="side-grip"></span>
  </div>
</nav>

<style>
  .sidebar {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 18px;
    padding: 18px 14px;
    background: var(--surface);
    border-right: 2px solid var(--border);
    min-height: 0;
  }
  .sidebar.resizing {
    user-select: none;
  }
  .side-resizer {
    position: absolute;
    top: 0;
    right: -6px;
    bottom: 0;
    width: 10px;
    cursor: col-resize;
    z-index: 25;
    display: flex;
    align-items: center;
    justify-content: center;
    touch-action: none;
  }
  .side-grip {
    width: 4px;
    height: 40px;
    border-radius: 4px;
    background: transparent;
    transition:
      background-color 120ms ease,
      height 120ms ease;
  }
  .side-resizer:hover .side-grip,
  .side-resizer.active .side-grip {
    background: var(--accent);
    height: 64px;
  }
  .sidebar.collapsed {
    padding: 18px 10px;
    align-items: center;
  }
  .sidebar.collapsed .brand {
    padding: 4px 0;
    justify-content: center;
  }
  .sidebar.collapsed .brand .name,
  .sidebar.collapsed .label,
  .sidebar.collapsed .profile-meta,
  .sidebar.collapsed .logged-out-top,
  .sidebar.collapsed .footer-row {
    display: none;
  }
  .sidebar.collapsed ul {
    width: 100%;
    align-items: center;
  }
  .sidebar.collapsed .nav-btn {
    justify-content: center;
    padding: 10px;
  }
  .sidebar.collapsed .account {
    width: 100%;
    align-items: center;
  }
  .sidebar.collapsed .profile-card {
    flex-direction: column;
    padding: 6px;
    gap: 6px;
    align-items: center;
  }
  .sidebar.collapsed .avatar {
    display: none;
  }
  .sidebar.collapsed .login-btn {
    padding: 0;
    width: 40px;
    min-width: 40px;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 4px 8px;
    font-weight: 800;
    font-size: 16px;
  }
  .logo {
    display: grid;
    place-items: center;
    width: 32px;
    height: 32px;
    border-radius: 10px;
    background: var(--accent);
    color: var(--on-accent);
    box-shadow: 0 var(--edge-depth) 0 var(--accent-edge);
    flex-shrink: 0;
  }
  ul {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .nav-btn {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    border: 2px solid transparent;
    border-radius: var(--radius);
    background: transparent;
    cursor: pointer;
    color: var(--text-muted);
    font-weight: 600;
    text-align: left;
    transition:
      background-color 120ms ease,
      transform 70ms ease;
  }
  .nav-btn:hover {
    background: var(--surface-2);
    color: var(--text);
  }
  .nav-btn.active {
    background: var(--accent-soft);
    border-color: var(--accent);
    color: var(--text);
    box-shadow: 0 var(--edge-depth) 0 var(--accent-edge);
  }
  .nav-btn:active {
    transform: translateY(2px);
  }

  .account {
    margin-top: auto;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .profile-card {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px;
    background: var(--surface-2);
    border: 2px solid var(--border);
    border-radius: var(--radius);
    box-shadow: 0 var(--edge-depth) 0 var(--edge);
  }
  .profile-card.logged-out {
    flex-direction: column;
    align-items: stretch;
    gap: 10px;
  }
  .logged-out-top {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .avatar {
    position: relative;
    width: 36px;
    height: 36px;
    border-radius: 10px;
    background: var(--surface-3);
    color: var(--text);
    border: 2px solid var(--border);
    box-shadow: 0 min(2px, var(--edge-depth)) 0 var(--edge);
    display: grid;
    place-items: center;
    flex-shrink: 0;
  }
  .avatar.muted-av {
    background: var(--surface-3);
    color: var(--text-muted);
    border-color: var(--border);
    box-shadow: 0 min(2px, var(--edge-depth)) 0 var(--edge);
  }
  .profile-meta {
    display: flex;
    flex-direction: column;
    min-width: 0;
    flex: 1;
    gap: 1px;
  }
  .acc-name {
    font-weight: 800;
    font-size: 13px;
  }
  .acc-sub {
    font-size: 11px;
  }
  .logout-btn {
    width: 34px;
    height: 34px;
    min-height: 34px;
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text-muted);
    flex-shrink: 0;
  }
  .logout-btn:hover {
    color: var(--accent);
    border-color: var(--accent);
  }
  .btn.full {
    width: 100%;
    min-height: 36px;
    font-weight: 700;
    font-size: 13px;
  }
  .footer-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 4px;
    font-size: 11px;
  }
  .ver-badge {
    padding: 2px 7px;
    border-radius: 6px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    color: var(--text-muted);
  }

  /* Small windows: sidebar turns into a horizontal top bar with icons only. */
  @media (max-width: 760px) {
    .sidebar {
      flex-direction: row;
      align-items: center;
      gap: 8px;
      padding: 8px 10px;
      border-right: 0;
      border-bottom: 2px solid var(--border);
    }
    .brand .name,
    .label,
    .profile-meta,
    .logged-out-top,
    .footer-row,
    .side-resizer {
      display: none;
    }
    ul {
      flex-direction: row;
      flex: 1;
      justify-content: center;
    }
    .nav-btn {
      padding: 8px;
    }
    .account {
      margin: 0;
      flex-direction: row;
    }
    .profile-card {
      padding: 4px;
      gap: 6px;
    }
  }
</style>
