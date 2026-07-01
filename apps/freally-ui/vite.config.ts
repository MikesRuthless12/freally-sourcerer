import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";

const host = process.env.TAURI_DEV_HOST;

// The 18 Fluent .ftl translation files live at the workspace root under
// `locales/<code>/freally.ftl` — outside this app's package root. Vite
// 5/6 default `server.fs.strict` blocks reads outside the project root,
// so we extend `fs.allow` to include the workspace root for dev. Builds
// don't need this (vite resolves `import.meta.glob` at bundle time).
const workspaceRoot = resolve(fileURLToPath(new URL(".", import.meta.url)), "../../");

export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"]
    },
    fs: {
      allow: [workspaceRoot]
    }
  }
});
