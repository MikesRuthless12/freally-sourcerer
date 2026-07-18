// The React island for "More Freally apps": mounts the vendored, view-only
// CentralPanel into a Svelte-owned container element. Svelte owns show/hide and
// teardown; React owns the panel subtree. The panel localizes through OUR own
// Fluent t() (its fcp-* catalogs are layered into our bundles in i18n/bundle.ts)
// and opens external links via the Tauri opener plugin. allowDownloads is false,
// so it is a pure showcase — no engine, no download/install controls.
import { createElement } from "react";
import { createRoot, type Root } from "react-dom/client";
import { CentralPanel, type PanelHost } from "@freally/central-panel";
import { t } from "../lib/i18n/t";
import { settingsStore } from "../lib/stores/settings.svelte";

const HOST: PanelHost = {
  openExternal: async (url: string) => {
    const opener = await import("@tauri-apps/plugin-opener");
    await opener.openUrl(url);
  }
};

function panelElement() {
  return createElement(CentralPanel, {
    t,
    locale: settingsStore.state.locale || "en",
    host: HOST,
    allowDownloads: false
  });
}

/** Create a React root on `el` and render the view-only panel into it. */
export function mountMoreApps(el: HTMLElement): Root {
  const root = createRoot(el);
  root.render(panelElement());
  return root;
}

/** Re-render so the panel picks up a new active locale (t reads it on render). */
export function refreshMoreApps(root: Root): void {
  root.render(panelElement());
}
