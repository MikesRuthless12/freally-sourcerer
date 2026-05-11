// Real Windows-shell icon cache. Keyed by `(ext, is_dir)` so 1.4M rows
// produce ~few hundred IPC calls instead of one per row. The cache
// stores Promises so concurrent requesters await the same in-flight
// invoke rather than firing duplicate calls.

import { invoke } from "@tauri-apps/api/core";

class IconStore {
  // Map keys are `${ext}|${is_dir ? "d" : "f"}`. Values resolve to a
  // `data:image/png;base64,…` URL or `null` if the OS couldn't supply
  // one (we render a fallback glyph in that case).
  private cache = new Map<string, Promise<string | null>>();
  // Reactive tick — bumped each time a fetch resolves so result rows
  // that called `get()` re-render once the icon comes in.
  tick = $state(0);

  private key(ext: string, isDir: boolean): string {
    return `${ext.toLowerCase()}|${isDir ? "d" : "f"}`;
  }

  /** Returns the cached data URL synchronously when available, otherwise
   *  kicks off the fetch in the background and returns null until it
   *  resolves. Components that call this from a `$derived` will pick up
   *  the resolved value on the next reactive tick. */
  get(ext: string, isDir: boolean): string | null {
    const k = this.key(ext, isDir);
    const existing = this.cache.get(k);
    if (existing) {
      // Race: if the Promise has already settled, we want the resolved
      // value synchronously. We attach a `.value` field on resolution.
      const settled = (existing as Promise<string | null> & { value?: string | null })
        .value;
      return settled ?? null;
    }
    const promise = invoke<string | null>("icon_for_ext", {
      ext,
      isDir,
    })
      .catch(() => null)
      .then((v) => {
        (promise as Promise<string | null> & { value?: string | null }).value = v;
        this.tick++;
        return v;
      });
    this.cache.set(k, promise);
    return null;
  }
}

export const iconStore = new IconStore();
