// SRC-M19 — Spacebar Quick Look.
//
// A transient, keyboard-driven preview. The docked preview pane is a
// persistent panel you glance at; this is the "what *is* this file?"
// gesture: Space opens it big, the arrow keys walk the result set with
// it still open, Space closes it again. The whole point is that your
// hands never leave the keyboard and the modal never gets in the way of
// moving on to the next row.

import type { QueryHit } from "../ipc/types";
import { LENS_ORDER, resultsStore } from "./results.svelte";
import { selectionStore } from "./selection.svelte";
import { settingsStore } from "./settings.svelte";
import { sortStore } from "./sort.svelte";

/**
 * The hits in the order the list actually renders them.
 *
 * Built from the same `viewForLens` the lens sections render from, so
 * it inherits lens visibility and the refine-bar narrowing for free —
 * reading `batches` directly would walk hits from a lens the user
 * switched off, and rows a refinement had filtered out. Within a lens
 * it applies the sort store, except for a grouped (duplicate-cluster)
 * batch where the daemon already ordered the rows and re-sorting would
 * break the clusters apart. This mirrors `LensSection` exactly; walking
 * any other order makes the arrow keys appear to jump around.
 */
export function visibleHits(): QueryHit[] {
  const out: QueryHit[] = [];
  for (const lens of LENS_ORDER) {
    if (settingsStore.state.lens_visibility[lens] === false) continue;
    const view = resultsStore.viewForLens(lens);
    out.push(...(view.groups.length > 0 ? view.hits : sortStore.applied(view.hits)));
  }
  return out;
}

class QuickLookStore {
  open = $state(false);

  /** The row Quick Look is showing — the first selected one. */
  get current(): QueryHit | null {
    const id = [...selectionStore.ids][0];
    if (!id) return null;
    return visibleHits().find((h) => h.file_id === id) ?? null;
  }

  /** Space with no selection would open an empty modal; don't. */
  toggle() {
    if (this.open) {
      this.open = false;
      return;
    }
    if (selectionStore.count === 0) return;
    this.open = true;
  }

  close() {
    this.open = false;
  }

  /**
   * Move the selection by `delta` rows and keep Quick Look pointed at
   * it. Clamps rather than wrapping: wrapping from the last row to the
   * first reads as a glitch when you are holding the arrow key down.
   */
  step(delta: number) {
    const hits = visibleHits();
    if (hits.length === 0) return;
    const id = [...selectionStore.ids][0];
    const at = id ? hits.findIndex((h) => h.file_id === id) : -1;
    const next = Math.min(hits.length - 1, Math.max(0, at + delta));
    if (next === at) return;
    selectionStore.ids = new Set([hits[next].file_id]);
  }
}

export const quickLookStore = new QuickLookStore();
