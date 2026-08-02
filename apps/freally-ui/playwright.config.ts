import { defineConfig, devices } from "@playwright/test";

// Visual-smoke "gallery": loads the REAL built UI in Chromium with a mocked
// Tauri IPC bridge (e2e/tauri-mock.js) and screenshots every feature panel.
// UI-render coverage only — the Rust daemon/backend is covered by the
// workspace `cargo test` + tests/smoke suites.
//
// Run:  pnpm build   (preview serves ./dist)
// Then: pnpm test:e2e
export default defineConfig({
  testDir: "./e2e",
  outputDir: "./e2e/.output",
  fullyParallel: false,
  workers: 1,
  forbidOnly: !!process.env.CI,
  reporter: [["list"]],
  use: {
    baseURL: "http://localhost:4173",
    viewport: { width: 1440, height: 900 },
    deviceScaleFactor: 1,
  },
  // `PW_CHANNEL=msedge` (or `chrome`) runs against a **system-installed**
  // browser instead of Playwright's own Chromium download. Two reasons to
  // want that:
  //
  //  1. Fidelity. Tauri renders in WebView2 on Windows, which *is* Edge —
  //     so `msedge` is closer to what ships than headless Chromium.
  //  2. It does not need the download, which is a real obstacle: the
  //     browser lives in a machine-global directory behind a single lock,
  //     so an unrelated project installing browsers blocks this one.
  //
  // Unset by default, so CI — which runs `playwright install` and wants a
  // pinned, reproducible Chromium — is unaffected.
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        ...(process.env.PW_CHANNEL ? { channel: process.env.PW_CHANNEL } : {}),
      },
    },
  ],
  webServer: {
    // `preview` serves the built `dist` — run `pnpm build` first.
    command: "pnpm preview --port 4173 --strictPort",
    url: "http://localhost:4173",
    reuseExistingServer: !process.env.CI,
    timeout: 60_000,
  },
});
