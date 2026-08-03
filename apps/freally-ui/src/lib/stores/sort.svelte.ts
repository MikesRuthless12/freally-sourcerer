// Sort store — mirrors the View → Sort by submenu state. Toggling a column
// header cycles asc → desc on the same column or jumps to a new column.

import type { ColumnId, QueryHit } from "../ipc/types";
import { naturalCompare } from "../util/natural";
import { settingsStore } from "./settings.svelte";

export type SortField = ColumnId | "lufs" | "length" | "similarity";
export type SortOrder = "asc" | "desc";

class SortStore {
  field = $state<SortField>("name");
  order = $state<SortOrder>("asc");

  toggle(field: SortField) {
    if (this.field === field) {
      this.order = this.order === "asc" ? "desc" : "asc";
    } else {
      this.field = field;
      this.order = "asc";
    }
  }

  setField(field: SortField) {
    this.field = field;
  }

  setOrder(order: SortOrder) {
    this.order = order;
  }

  /** Sorted view of a lens's hits. */
  applied(hits: QueryHit[]): QueryHit[] {
    const dir = this.order === "asc" ? 1 : -1;
    const sorted = [...hits].sort((a, b) => dir * compare(a, b, this.field));
    return sorted;
  }
}

/** SRC-M24 — every string column goes through the same comparator, so
 *  "natural sort" does not quietly mean "only the name column". Off by
 *  setting, in which case this is exactly the `localeCompare` that
 *  shipped before. */
function text(a: string, b: string): number {
  return settingsStore.state.natural_sort === false
    ? a.localeCompare(b)
    : naturalCompare(a, b);
}

function compare(a: QueryHit, b: QueryHit, field: SortField): number {
  switch (field) {
    case "name": return text(a.name, b.name);
    case "path": return text(a.path, b.path);
    case "size": return a.size - b.size;
    case "modified": return a.modified_ms - b.modified_ms;
    case "type": return text(a.type, b.type);
    case "ext": return text(a.ext, b.ext);
    case "similarity": return b.score - a.score;
    case "lufs":
    case "length":
      // Mock data — fall back to name.
      return text(a.name, b.name);
  }
}

// The results store now imports *this* module (its `visibleHits` applies
// the sort), so the old `void resultsStore` touch has to go: it ran at
// module scope and would read an uninitialised binding whenever the
// results store happened to be evaluated first.
export const sortStore = new SortStore();
