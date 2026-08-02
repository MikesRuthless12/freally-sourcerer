// SRC-M15 bulk rename dialog state + SRC-M16 undo/redo state.

import * as renameIpc from "../ipc/rename";
import type {
  OperationEntry,
  OperationListing,
  RenamePreview,
  RenameRule
} from "../ipc/types";

/** Keystroke-rate previews would hit the disk on every character. */
const PREVIEW_DEBOUNCE_MS = 150;

export function defaultRule(): RenameRule {
  return {
    find: "",
    replace: "",
    use_regex: false,
    ignore_case: false,
    case: "none",
    part: "stem",
    counter_start: 1,
    counter_step: 1
  };
}

class RenameStore {
  /** Paths the dialog opened on, in the order the user sees them —
   *  `{n}` counts in this order. */
  paths = $state<string[]>([]);
  rule = $state<RenameRule>(defaultRule());
  preview = $state<RenamePreview | null>(null);
  error = $state<string | null>(null);
  applying = $state(false);

  #timer: ReturnType<typeof setTimeout> | null = null;

  open(paths: string[]) {
    this.paths = paths;
    this.rule = defaultRule();
    this.preview = null;
    this.error = null;
    void this.refresh();
  }

  reset() {
    if (this.#timer !== null) {
      clearTimeout(this.#timer);
      this.#timer = null;
    }
    this.paths = [];
    this.preview = null;
    this.error = null;
  }

  /** Re-plan after a rule edit, debounced. */
  schedule() {
    if (this.#timer !== null) clearTimeout(this.#timer);
    this.#timer = setTimeout(() => void this.refresh(), PREVIEW_DEBOUNCE_MS);
  }

  async refresh() {
    if (this.paths.length === 0) return;
    try {
      this.preview = await renameIpc.preview(this.paths, this.rule);
      this.error = null;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
      this.preview = null;
    }
  }

  get canApply(): boolean {
    return (
      !this.applying &&
      this.preview !== null &&
      !this.preview.blocked &&
      this.preview.will_apply > 0
    );
  }

  /** Returns how many files were renamed, or null on failure. */
  async apply(): Promise<number | null> {
    if (!this.canApply) return null;
    this.applying = true;
    try {
      const out = await renameIpc.apply(this.paths, this.rule);
      await opsStore.refresh();
      return out.renamed;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
      return null;
    } finally {
      this.applying = false;
    }
  }
}

class OpsStore {
  entries = $state<OperationEntry[]>([]);
  undoId = $state<string | null>(null);
  redoId = $state<string | null>(null);
  error = $state<string | null>(null);
  busy = $state(false);

  async refresh() {
    try {
      const listing: OperationListing = await renameIpc.opsList();
      this.entries = listing.entries;
      this.undoId = listing.undo_id ?? null;
      this.redoId = listing.redo_id ?? null;
    } catch (e) {
      // The daemon may still be booting; an empty history is the right
      // thing to show, not an error banner.
      this.entries = [];
      this.undoId = null;
      this.redoId = null;
    }
  }

  get canUndo(): boolean {
    return !this.busy && this.undoId !== null;
  }
  get canRedo(): boolean {
    return !this.busy && this.redoId !== null;
  }

  async undo(): Promise<number | null> {
    return this.#run(this.undoId, (id) => renameIpc.undo(id));
  }

  async redo(): Promise<number | null> {
    return this.#run(this.redoId, (id) => renameIpc.redo(id));
  }

  async #run(
    id: string | null,
    fn: (id: string) => Promise<{ moved: number }>
  ): Promise<number | null> {
    if (id === null || this.busy) return null;
    this.busy = true;
    try {
      const out = await fn(id);
      this.error = null;
      return out.moved;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
      return null;
    } finally {
      this.busy = false;
      // Re-read rather than assuming: the backend only marks an entry
      // undone once the disk actually changed.
      await this.refresh();
    }
  }
}

export const renameStore = new RenameStore();
export const opsStore = new OpsStore();
