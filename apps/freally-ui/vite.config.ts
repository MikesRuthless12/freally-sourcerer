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
  // The "More Freally apps" panel is the React `CentralPanel` vendored from the
  // freally-central submodule (view-only). Svelte owns the app; the panel is a
  // React island. esbuild's automatic JSX runtime transforms the vendored .tsx
  // at build time — no @vitejs/plugin-react (which would fight the Svelte
  // plugin's HMR). The alias is the panel's only public entry point.
  resolve: {
    // The vendored panel lives at repo-root vendor/, outside this package, so
    // its bare imports can't walk up to our node_modules. dedupe forces them to
    // resolve from this project's single installed copy.
    dedupe: ["react", "react-dom", "@tauri-apps/api"],
    alias: {
      "@freally/central-panel": fileURLToPath(
        new URL("../../vendor/freally-central/ui/src/panel", import.meta.url)
      )
    }
  },
  esbuild: {
    jsx: "automatic",
    jsxImportSource: "react"
  },
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
