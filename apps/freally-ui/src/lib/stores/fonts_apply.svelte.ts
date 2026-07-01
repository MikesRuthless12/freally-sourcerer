// Applies the persisted `fonts_and_colors` settings to the live DOM by
// writing CSS custom properties on <html>. The corresponding CSS rules
// in `app.css` then pick them up. Re-runs on every settings change so
// "Apply" in the Fonts & Colors panel feels instant.

import { settingsStore } from "./settings.svelte";
import type { RgbColor } from "../ipc/types";

function rgb(c: RgbColor | null): string | null {
  if (!c) return null;
  return `rgb(${c.r}, ${c.g}, ${c.b})`;
}

const STATE_KEYS = [
  "normal",
  "highlighted",
  "current_sort",
  "current_sort_highlighted",
  "selected",
  "selected_highlighted",
  "inactive_selected",
  "inactive_selected_highlighted",
] as const;

export function applyFontsAndColors() {
  if (typeof document === "undefined") return;
  const fc = settingsStore.state.fonts_and_colors;
  if (!fc) return;

  const root = document.documentElement;

  // --- Global font family + size --------------------------------------
  // The user types any font name into the panel; we wrap it in quotes
  // (so multi-word family names like "Cascadia Code" work) and add a
  // sensible cross-OS fallback chain. The literal value "default" or an
  // empty string leaves the original token-stack alone.
  const f = (fc.font ?? "").trim();
  if (f && f.toLowerCase() !== "default") {
    const quoted = /[",]/.test(f) ? f : `"${f}"`;
    root.style.setProperty(
      "--font-ui",
      `${quoted}, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif`,
    );
  } else {
    root.style.removeProperty("--font-ui");
  }

  if (typeof fc.size_px === "number" && fc.size_px > 0) {
    root.style.setProperty("--app-font-size", `${fc.size_px}px`);
  } else {
    root.style.removeProperty("--app-font-size");
  }

  // --- Per-state colors + bold/italic --------------------------------
  for (const key of STATE_KEYS) {
    const s = fc.states?.[key];
    if (!s) continue;
    const fg = rgb(s.fg);
    const bg = rgb(s.bg);
    setOrRemove(root, `--row-${key}-fg`, fg);
    setOrRemove(root, `--row-${key}-bg`, bg);
    setOrRemove(root, `--row-${key}-weight`, s.bold ? "700" : null);
    setOrRemove(root, `--row-${key}-style`, s.italic ? "italic" : null);
  }

  // --- Per-lens accent colors (override the defaults from tokens) ----
  for (const lens of ["filename", "content", "audio", "similarity"] as const) {
    const c = fc.per_lens_accent?.[lens];
    setOrRemove(root, `--lens-${lens}`, rgb(c ?? null));
  }
}

function setOrRemove(el: HTMLElement, name: string, value: string | null) {
  if (value === null || value === "") el.style.removeProperty(name);
  else el.style.setProperty(name, value);
}
