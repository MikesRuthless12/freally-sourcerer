// SRC-M19 — Spacebar Quick Look.
//
// A transient, keyboard-driven preview. The docked preview pane is a
// persistent panel you glance at; this is the "what *is* this file?"
// gesture: Space opens it big, the arrow keys walk the result set with
// it still open, Space closes it again. The whole point is that your
// hands never leave the keyboard and the modal never gets in the way of
// moving on to the next row.

import type { QueryHit } from "../ipc/types";
import { resultsStore } from "./results.svelte";
import { selectionStore } from "./selection.svelte";

/**
 * The hits in the order the list actually renders them.
 *
 * Re-exported from `resultsStore` rather than recomputed: this started
 * as a second implementation, which meant the arrow keys and
 * export/select-all/the status-bar count could disagree about what
 * "next row" meant the moment a column sort was active.
 */
export function visibleHits(): QueryHit[] {
  return resultsStore.visibleHits;
}

class QuickLookStore {
  open = $state(false);

  /** The row Quick Look is showing — the first selected one. */
  get current(): QueryHit | null {
    const id = selectionStore.first;
    if (!id) return null;
    return resultsStore.hitById(id);
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
    // `.values().next()` rather than spreading: a Ctrl+A selection is
    // tens of thousands of ids, and this runs per arrow key.
    const id = selectionStore.first;
    const at = id ? hits.findIndex((h) => h.file_id === id) : -1;
    const next = Math.min(hits.length - 1, Math.max(0, at + delta));
    if (next === at) return;
    selectionStore.ids = new Set([hits[next].file_id]);
  }
}

export const quickLookStore = new QuickLookStore();
