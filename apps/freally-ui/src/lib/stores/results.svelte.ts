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
import { t } from "../i18n/t";
import { toastStore } from "./toast.svelte";
import type {
  DidYouMean,
  HitGroup,
  LensId,
  LensTimings,
  QueryBatch,
  QueryDone,
  QueryHit
} from "../ipc/types";
import { fileListStore } from "./file_list.svelte";
import { refineStore } from "./refine.svelte";
import { selectionStore } from "./selection.svelte";
import { sortStore } from "./sort.svelte";
import { recentSearchesStore } from "./recent_searches.svelte";
import { settingsStore } from "./settings.svelte";
import { typeFilterStore } from "./type_filter.svelte";

/** Lens render order, mirrored from `ResultList`. Held here too so
 *  `visibleHits` walks the lenses in the order they appear on screen —
 *  an export should come out in reading order. */
export const LENS_ORDER: LensId[] = ["filename", "content", "audio", "similarity"];

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
  /** SRC-M11 — set only when a query matched nothing and the daemon
   *  found a plausible correction. Cleared at the start of every run so
   *  a stale suggestion never outlives the search that produced it. */
  didYouMean = $state<DidYouMean | null>(null);
  private seq = 0;
  private batchUnlisten: UnlistenFn | null = null;
  private doneUnlisten: UnlistenFn | null = null;
  private listenersPending = false;

  async ensureListeners() {
    // Claim the slot before awaiting: two `run()` calls can interleave
    // (each awaits `cancelRunning()` first), and both would otherwise
    // see `null`, both register, and the second assignment would leak
    // the first unlisten handle — leaving every batch processed twice.
    if (this.listenersPending) return;
    if (this.batchUnlisten && this.doneUnlisten) return;
    this.listenersPending = true;
    try {
      await this.registerListeners();
    } finally {
      this.listenersPending = false;
    }
  }

  private async registerListeners() {
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
        this.didYouMean = done.did_you_mean ?? null;
        this.running = null;
      });
    }
  }

  async run(source: string) {
    const my = ++this.seq;
    // SRC-M22 — feed the sidebar's Recent list. Fire-and-forget: a
    // settings write must never delay a keystroke-rate search, and a
    // failed one is not worth failing the query over.
    void recentSearchesStore.record(source).catch(() => {});
    // Drop the previous suggestion before anything else: it belongs to
    // the query being replaced, and leaving it up while the new one
    // runs would offer a correction for a term no longer on screen.
    this.didYouMean = null;
    // SRC-M03: an imported file list catalogues a volume the daemon
    // has never indexed, so it answers its own queries. Cancel any
    // in-flight daemon work first — a stale batch arriving afterwards
    // would overwrite the list's results.
    if (fileListStore.active) {
      await this.cancelRunning();
      if (my !== this.seq) return;
      const hits = fileListStore.search(source);
      this.batches = [{ handle: "file-list", lens: "filename", hits, done: true }];
      selectionStore.clear();
      this.timings = null;
      this.lastQueryMs = 0;
      return;
    }
    // Cancel + drop the previous in-flight handle so the daemon doesn't
    // grow per-keystroke memory.
    await this.cancelRunning();
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
        per_lens_limits: settingsStore.state.default_lens_result_limits,
        // SRC-M23 — the whole toggle set, not just the phonetic one.
        search_opts: settingsStore.state.search_opts,
        // SRC-M24 — the daemon sorts too, and `freally search` reads it
        // straight from there, so the toggle has to cross the wire and
        // not just reach the client-side comparator.
        natural_sort: settingsStore.state.natural_sort
      }));
    } catch (e) {
      // A search that throws leaves the previous batches in place or an
      // empty list, both of which read as "no matches" — the one outcome
      // the user cannot tell apart from a working search that found
      // nothing. Say it failed.
      console.warn("[results] run failed:", e);
      toastStore.error(t("toast-search-failed", { error: String(e) }));
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
    // flash stale results between keystrokes. The selection goes with
    // them: ids from the old result set would keep inflating the
    // status bar's "N selected" against a set that no longer holds
    // them, while the byte total and the copy verbs — which do filter
    // against the current batches — reported something different.
    this.batches = [];
    selectionStore.clear();
    this.timings = null;
    this.running = { handle, source, startedAt: t0 };
  }

  async cancel() {
    const r = this.running;
    if (!r) return;
    await ipcQuery.cancel(r.handle);
    this.running = null;
  }

  /** Drop the in-flight handle, tolerating a daemon that has already
   *  forgotten it. Clears `running` before awaiting so a late batch
   *  for the old handle is discarded by the listener's guard. */
  private async cancelRunning() {
    const prior = this.running?.handle;
    if (!prior) return;
    this.running = null;
    try {
      await ipcQuery.cancel(prior);
    } catch (e) {
      console.warn("[results] cancel-prior failed:", e);
    }
  }

  /** Every hit currently on screen, across all lenses. The single
   *  answer to "what is the user looking at" — export, select-all, and
   *  the status-bar count all read it, so they cannot disagree.
   *
   *  Goes through `viewForLens`, which is the same function the lens
   *  sections render from: that is what makes "visible" mean visible
   *  rather than "returned by the daemon". Reading the batches directly
   *  would count hits in a lens the user has switched off, and
   *  duplicate-cluster members that grouping dropped. */
  /*
   * `$derived.by`, not a getter. Building this list copies and sorts
   * every lens's hits, and it is read from a lot of places that each
   * assumed they were the only one: the preview pane reads it twice per
   * selection change, Quick Look three times per arrow key, and the
   * status bar on every render. As a getter that was a full rebuild
   * each time; as a `$derived` it is computed once per change to the
   * batches, the sort, or the lens toggles, and every reader after that
   * is a field read.
   */
  visibleHits: QueryHit[] = $derived(
    LENS_ORDER.flatMap((lens) => {
      if (settingsStore.state.lens_visibility[lens] === false) return [];
      const view = this.viewForLens(lens);
      // Sorted the way `LensSection` renders it, so this really is the
      // order on screen — export, select-all, the status-bar count and
      // Quick Look's arrow keys all read this, and before SRC-M22 they
      // disagreed the moment a column sort was active. Grouped
      // (duplicate-cluster) batches keep the daemon's order: re-sorting
      // them would break the clusters apart.
      return view.groups.length > 0 ? view.hits : sortStore.applied(view.hits);
    })
  );

  /** `file_id` → hit, over the same list. Quick Look walks this per
   *  arrow key and the preview pane per selection change; a linear
   *  `find` over a few hundred hits each time is work neither of them
   *  needs to repeat. */
  #byId: Map<string, QueryHit> = $derived(
    new Map(this.visibleHits.map((h) => [h.file_id, h]))
  );

  /**
   * The on-screen hit with this id, or null.
   *
   * Reads the visible list rather than the raw batches: a row belonging
   * to a lens the user has switched off is not on screen, and nothing
   * should be previewing or Quick Looking it. Three call sites had
   * rolled this by hand and two of them searched the raw batches, so
   * a hidden lens could still drive the preview pane.
   */
  hitById(id: string): QueryHit | null {
    return this.#byId.get(id) ?? null;
  }

  get total(): number {
    return this.visibleHits.length;
  }

  /** Hits the daemon returned, before refinement — but still only for
   *  lenses that are on screen. The refine bar shows both so
   *  "1,204 → 12" reads as a narrowing, not a new search. */
  get totalUnrefined(): number {
    return this.batches.reduce(
      (n, b) => n + (settingsStore.state.lens_visibility[b.lens] === false ? 0 : b.hits.length),
      0
    );
  }

  /** What a lens section should render: hits narrowed by the active
   *  refinement chips, with any duplicate clusters re-offset to match.
   *  Groups are rebuilt rather than passed through because their
   *  `start`/`len` index into the *unrefined* rows — reusing them
   *  after a filter would slice the wrong rows. A cluster that loses
   *  all but one member stops being a duplicate group and is dropped. */
  viewForLens(lens: LensId): { hits: QueryHit[]; groups: HitGroup[] } {
    const batch = this.batches.find((b) => b.lens === lens);
    const hits = batch?.hits ?? [];
    const groups = batch?.groups ?? [];
    if (!refineStore.active) return { hits, groups };
    if (groups.length === 0) {
      return { hits: refineStore.apply(hits), groups: [] };
    }
    const outHits: QueryHit[] = [];
    const outGroups: HitGroup[] = [];
    for (const g of groups) {
      const kept = refineStore.apply(hits.slice(g.start, g.start + g.len));
      if (kept.length < 2) continue;
      outGroups.push({ ...g, start: outHits.length, len: kept.length });
      outHits.push(...kept);
    }
    return { hits: outHits, groups: outGroups };
  }
}

export const resultsStore = new ResultsStore();
