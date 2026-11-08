/// <reference types="@sveltejs/kit" />

declare global {
  /** Injected by vite.config.ts from package.json */
  const __APP_VERSION__: string;
  namespace App {}
}

export {};
