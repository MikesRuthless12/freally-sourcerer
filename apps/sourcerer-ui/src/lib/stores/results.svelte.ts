// Results store — last query handle + per-lens batches + lens timings.
//
// Phase 12 streaming model: the daemon emits `query:batch` notifications
// per lens (re-emitted as Tauri events by `daemon.rs`) plus a final
// `query:done` notification carrying the lens timings. The store listens
// for both, accumulates batches keyed by handle, and discards anything
// that doesn't match the active handle (defends against a stale
// notification arriving after a newer keystroke).
//
// Sequence-guarded: superseded `run()` calls cancel the prior handle so
// the daemon can drop in-flight work + reclaim memory.

import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import * as ipcQuery from "../ipc/query";
import type { LensId, LensTimings, QueryBatch, QueryDone, QueryHit } from "../ipc/types";
import { settingsStore } from "./settings.svelte";
import { typeFilterStore } from "./type_filter.svelte";

interface RunningQuery {
  handle: string;
  source: string;
  startedAt: number;
}

class ResultsStore {
  running = $state<RunningQuery | null>(null);
  batches = $state<QueryBatch[]>([]);
  timings = $state<LensTimings | null>(null);
  lastQueryMs = $state(0);
  private seq = 0;
  private batchUnlisten: UnlistenFn | null = null;
  private doneUnlisten: UnlistenFn | null = null;

  async ensureListeners() {
    if (this.batchUnlisten && this.doneUnlisten) return;
    if (!this.batchUnlisten) {
      this.batchUnlisten = await listen<QueryBatch>("query:batch", (e) => {
        const batch = e.payload;
        const cur = this.running;
        if (!cur || cur.handle !== batch.handle) return;
        // Replace any prior batch for the same lens (handles partial
        // streaming where the daemon emits multiple batches per lens —
        // for Phase 12 each lens emits one batch; keep the contract
        // forward-compatible).
        const next = this.batches.filter((b) => b.lens !== batch.lens);
        next.push(batch);
        this.batches = next;
      });
    }
    if (!this.doneUnlisten) {
      this.doneUnlisten = await listen<QueryDone>("query:done", (e) => {
        const done = e.payload;
        const cur = this.running;
        if (!cur || cur.handle !== done.handle) return;
        this.timings = done.timings;
        this.lastQueryMs = Math.round(performance.now() - cur.startedAt);
        this.running = null;
      });
    }
  }

  async run(source: string) {
    const my = ++this.seq;
    // Cancel + drop the previous in-flight handle so the daemon doesn't
    // grow per-keystroke memory.
    if (this.running) {
      const prior = this.running.handle;
      this.running = null;
      try {
        await ipcQuery.cancel(prior);
      } catch (e) {
        console.warn("[results] cancel-prior failed:", e);
      }
    }
    // Compose the actual query sent to the daemon based on the
    // multi-select type-filter set + the user's typed source:
    //   - No types selected → user explicitly disabled everything; show 0.
    //   - All types selected + empty source → "Everything" mode; match every
    //     file/folder with a bare `*` wildcard (voidtools-Everything parity).
    //   - Partial types selected → `(audio: OR video: …)` group prepended
    //     to whatever the user typed.
    const trimmedSource = source.trim();
    let composed: string;
    if (typeFilterStore.isNoneSelected()) {
      composed = "";
    } else if (typeFilterStore.isAllSelected() && trimmedSource.length === 0) {
      composed = "*";
    } else {
      const fragment = typeFilterStore.toQueryFragment();
      composed = [fragment, trimmedSource].filter((s) => s.length > 0).join(" ");
    }
    if (!composed.trim()) {
      if (my !== this.seq) return;
      this.batches = [];
      this.timings = null;
      this.lastQueryMs = 0;
      return;
    }
    await this.ensureListeners();
    const t0 = performance.now();
    let handle: string;
    try {
      ({ handle } = await ipcQuery.run(composed, {
        strict_everything: settingsStore.state.strict_everything_mode,
        per_lens_limits: settingsStore.state.default_lens_result_limits
      }));
    } catch (e) {
      console.warn("[results] run failed:", e);
      return;
    }
    if (my !== this.seq) {
      try {
        await ipcQuery.cancel(handle);
      } catch {
        /* best-effort */
      }
      return;
    }
    // Empty the batches as soon as a new query starts so the UI doesn't
    // flash stale results between keystrokes.
    this.batches = [];
    this.timings = null;
    this.running = { handle, source, startedAt: t0 };
  }

  async cancel() {
    const r = this.running;
    if (!r) return;
    await ipcQuery.cancel(r.handle);
    this.running = null;
  }

  get total(): number {
    return this.batches.reduce((n, b) => n + b.hits.length, 0);
  }

  hitsForLens(lens: LensId): QueryHit[] {
    const b = this.batches.find((x) => x.lens === lens);
    return b ? b.hits : [];
  }
}

export const resultsStore = new ResultsStore();
