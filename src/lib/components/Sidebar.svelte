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
</script>

<nav class="sidebar">
  <div class="brand">
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
      <div class="profile-card">
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
        <button class="btn accent full" onclick={player.login}>
          Google ile Bağlan
        </button>
      </div>
    {/if}

    <div class="footer-row">
      <span class="muted">Masaüstü İstemci</span>
      <span class="ver-badge">v{__APP_VERSION__}</span>
    </div>
  </div>
</nav>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    gap: 18px;
    padding: 18px 14px;
    background: var(--surface);
    border-right: 2px solid var(--border);
    min-height: 0;
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
    .footer-row {
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
