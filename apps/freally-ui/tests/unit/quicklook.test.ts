// SRC-M19 — Quick Look navigation.
//
// The modal itself is a thin wrapper around the same `files.preview`
// the docked pane calls; what is worth testing is the navigation, which
// is where an off-by-one or a wrap-around is invisible until someone is
// holding an arrow key down.

import { describe, it, expect, beforeEach } from "vitest";
import { quickLookStore } from "../../src/lib/stores/quicklook.svelte";
import { resultsStore } from "../../src/lib/stores/results.svelte";
import { selectionStore } from "../../src/lib/stores/selection.svelte";
import { sortStore } from "../../src/lib/stores/sort.svelte";
import type { QueryBatch, QueryHit } from "../../src/lib/ipc/types";

function hit(id: string, name: string): QueryHit {
  return {
    file_id: id,
    lens: "filename",
    name,
    path: `/p/${name}`,
    ext: "txt",
    size: 1,
    modified_ms: 0,
    type: "TXT",
    score: 1
  };
}

const batch: QueryBatch = {
  handle: "h",
  lens: "filename",
  hits: [hit("a", "alpha.txt"), hit("b", "beta.txt"), hit("c", "gamma.txt")],
  done: true
};

describe("quickLookStore", () => {
  beforeEach(() => {
    resultsStore.batches = [batch];
    selectionStore.clear();
    quickLookStore.close();
    sortStore.setField("name");
    sortStore.setOrder("asc");
  });

  it("does not open on an empty selection", () => {
    quickLookStore.toggle();
    expect(quickLookStore.open).toBe(false);
  });

  it("opens and closes on repeated toggles once something is selected", () => {
    selectionStore.toggle("a");
    quickLookStore.toggle();
    expect(quickLookStore.open).toBe(true);
    quickLookStore.toggle();
    expect(quickLookStore.open).toBe(false);
  });

  it("steps forward and back through the visible order", () => {
    selectionStore.toggle("a");
    quickLookStore.step(1);
    expect([...selectionStore.ids]).toEqual(["b"]);
    quickLookStore.step(1);
    expect([...selectionStore.ids]).toEqual(["c"]);
    quickLookStore.step(-1);
    expect([...selectionStore.ids]).toEqual(["b"]);
  });

  it("clamps at both ends instead of wrapping", () => {
    // Wrapping from the last row back to the first reads as a glitch
    // when the key is held down.
    selectionStore.toggle("c");
    quickLookStore.step(1);
    expect([...selectionStore.ids]).toEqual(["c"]);
    selectionStore.ids = new Set(["a"]);
    quickLookStore.step(-1);
    expect([...selectionStore.ids]).toEqual(["a"]);
  });

  it("replaces the selection rather than extending it", () => {
    selectionStore.toggle("a");
    selectionStore.toggle("b");
    quickLookStore.step(1);
    expect(selectionStore.count).toBe(1);
  });

  it("walks the sorted order, not the order the daemon sent", () => {
    sortStore.setOrder("desc");
    selectionStore.ids = new Set(["c"]);
    // Descending by name: gamma, beta, alpha — so "next" after gamma
    // is beta, not the end of the list.
    quickLookStore.step(1);
    expect([...selectionStore.ids]).toEqual(["b"]);
  });

  it("reports the current hit for the modal header", () => {
    selectionStore.toggle("b");
    expect(quickLookStore.current?.name).toBe("beta.txt");
  });
});
