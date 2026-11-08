import { readFileSync } from 'node:fs';
import adapter from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

const pkg = JSON.parse(readFileSync(new URL('./package.json', import.meta.url), 'utf-8'));
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [
    // Tauri serves static files only: SPA mode, no SSR.
    sveltekit({ adapter: adapter({ fallback: 'index.html' }) })
  ],
  define: {
    __APP_VERSION__: JSON.stringify(pkg.version)
  },
  // Tauri expects a fixed port and its own error output.
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: 'ws', host, port: 1421 } : undefined,
    watch: { ignored: ['**/src-tauri/**'] }
  }
});
