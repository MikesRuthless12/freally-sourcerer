// Selection store — set of selected file_ids + aggregate byte size.
//
// `bytes` is computed via `$derived` so the O(n) scan over batches+ids
// only re-runs when one of those inputs changes, not on every status-bar
// re-render (M6 fix). Keeping `count` as a plain getter is fine because
// `Set.size` is O(1).

import { resultsStore } from "./results.svelte";

class SelectionStore {
  ids = $state(new Set<string>());
  bytes = $derived.by(() => {
    let total = 0;
    for (const batch of resultsStore.batches) {
      for (const hit of batch.hits) if (this.ids.has(hit.file_id)) total += hit.size;
    }
    return total;
  });

  has(id: string): boolean {
    return this.ids.has(id);
  }

  toggle(id: string) {
    const next = new Set(this.ids);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    this.ids = next;
  }

  clear() {
    this.ids = new Set();
  }

  selectAll(allIds: string[]) {
    this.ids = new Set(allIds);
  }

  get count(): number {
    return this.ids.size;
  }
}

export const selectionStore = new SelectionStore();
