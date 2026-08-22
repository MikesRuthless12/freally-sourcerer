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

  #timer: ReturnType<typeof setTimeout> | null = null;

  /** Whether a refresh has ever completed. Deliberately a plain private
   *  field and not `$state`: it is the answer to "is this the first load",
   *  which nothing renders. Deriving it from `health` instead — the old
   *  `this.loading = this.health === null` — made `refresh()` *read*
   *  reactive state, and `start()` runs inside a Svelte `$effect`. That one
   *  read enrolled the effect in this store's own state, so every `health`
   *  write re-ran the effect, whose cleanup stops the poller and whose body
   *  starts it again: the panel polled as fast as the RPC answered instead
   *  of every POLL_MS, with the main thread pegged.
   *
   *  The load-bearing property is that `refresh()` reads no reactive state.
   *  Keep it that way. */
  #everLoaded = false;

  async refresh() {
    this.loading = !this.#everLoaded;
    try {
      this.health = await indexIpc.health();
      this.#everLoaded = true;
      this.error = null;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
    } finally {
      this.loading = false;
    }
  }

  #running = false;

  start() {
    if (this.#running) return;
    this.#running = true;
    void this.#tick();
  }

  stop() {
    this.#running = false;
    if (this.#timer !== null) {
      clearTimeout(this.#timer);
      this.#timer = null;
    }
  }

  /** Self-scheduling rather than `setInterval`: a slow refresh must not
   *  stack a second request on top of the one still in flight. */
  async #tick() {
    await this.refresh();
    if (!this.#running) return;
    this.#timer = setTimeout(() => void this.#tick(), POLL_MS);
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
