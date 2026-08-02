// SRC-M13 — Index Health panel state.
//
// Polls `index_health` only while the panel is open. The numbers move
// continuously (they are live journal counters), so a snapshot taken when
// the dialog opened would be stale by the time anyone read it.

import * as indexIpc from "../ipc/index_api";
import type { IndexHealth } from "../ipc/types";

const POLL_MS = 2000;

class IndexHealthStore {
  health = $state<IndexHealth | null>(null);
  loading = $state(false);
  error = $state<string | null>(null);
  /** True while a one-click fix is running. */
  fixing = $state(false);

  #timer: ReturnType<typeof setInterval> | null = null;

  async refresh() {
    this.loading = this.health === null;
    try {
      this.health = await indexIpc.health();
      this.error = null;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
    } finally {
      this.loading = false;
    }
  }

  start() {
    void this.refresh();
    this.#timer ??= setInterval(() => void this.refresh(), POLL_MS);
  }

  stop() {
    if (this.#timer !== null) {
      clearInterval(this.#timer);
      this.#timer = null;
    }
  }

  /** Run an advisory's one-click fix, then re-read the numbers it fixed. */
  async applyFix(fix: "none" | "rebuild_index") {
    if (fix === "none" || this.fixing) return;
    this.fixing = true;
    try {
      await indexIpc.rebuild();
      await this.refresh();
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
    } finally {
      this.fixing = false;
    }
  }
}

export const indexHealthStore = new IndexHealthStore();
