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
  projects: [{ name: "chromium", use: { ...devices["Desktop Chrome"] } }],
  webServer: {
    // `preview` serves the built `dist` — run `pnpm build` first.
    command: "pnpm preview --port 4173 --strictPort",
    url: "http://localhost:4173",
    reuseExistingServer: !process.env.CI,
    timeout: 60_000,
  },
});
