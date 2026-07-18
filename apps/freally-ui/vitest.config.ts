import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { fileURLToPath } from "node:url";

export default defineConfig({
  plugins: [svelte({ hot: false })],
  test: {
    environment: "jsdom",
    include: ["tests/unit/**/*.test.ts"],
    globals: false,
    setupFiles: ["./tests/setup.ts"]
  },
  resolve: {
    conditions: ["browser"],
    // The vendored panel lives at repo-root vendor/, outside this package, so
    // its bare imports can't walk up to our node_modules. dedupe forces them to
    // resolve from this project's single installed copy.
    dedupe: ["react", "react-dom", "@tauri-apps/api"],
    alias: {
      // The vendored React panel (More Freally apps) — same alias as the app
      // build, so the smoke test exercises the real panel source.
      "@freally/central-panel": fileURLToPath(
        new URL("../../vendor/freally-central/ui/src/panel", import.meta.url)
      )
    }
  },
  // esbuild's automatic JSX runtime transforms the vendored .tsx in tests too.
  esbuild: {
    jsx: "automatic",
    jsxImportSource: "react"
  }
});
