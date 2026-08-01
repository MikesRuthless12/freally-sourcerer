// Result-row context menu placement + target (Build 1).
//
// One menu at a time, positioned at the pointer but clamped inside the
// viewport so a right-click near the bottom-right edge doesn't open a
// menu half off-screen.

import type { QueryHit } from "../ipc/types";

/** Widest the menu gets, per its CSS `max-width`. */
const MAX_WIDTH = 320;
/** The menu's CSS caps its height at `70vh`, so reserving a flat pixel
 *  count would over-reserve on a short window and pin every menu to the
 *  top. Reserve the same fraction the stylesheet does. */
const MAX_HEIGHT_FRACTION = 0.7;
const EDGE_MARGIN = 4;

class ContextMenuStore {
  target = $state<QueryHit | null>(null);
  x = $state(0);
  y = $state(0);

  openAt(ev: MouseEvent, hit: QueryHit) {
    const vw = typeof window === "undefined" ? MAX_WIDTH * 2 : window.innerWidth;
    const vh = typeof window === "undefined" ? 800 : window.innerHeight;
    this.x = clamp(ev.clientX, vw, MAX_WIDTH);
    this.y = clamp(ev.clientY, vh, vh * MAX_HEIGHT_FRACTION);
    this.target = hit;
  }

  close() {
    this.target = null;
  }
}

/** Keep `at` inside `viewport`, leaving room for a menu of `extent`.
 *  When the menu cannot fit either way, prefer the top/left edge over a
 *  negative coordinate. */
function clamp(at: number, viewport: number, extent: number): number {
  return Math.max(EDGE_MARGIN, Math.min(at, viewport - extent - EDGE_MARGIN));
}

export const contextMenuStore = new ContextMenuStore();
