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
    dedupe: ["@tauri-apps/api"]
  }
});
