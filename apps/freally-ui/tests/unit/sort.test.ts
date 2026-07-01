import { describe, it, expect, beforeEach } from "vitest";
import { sortStore } from "../../src/lib/stores/sort.svelte";
import type { QueryHit } from "../../src/lib/ipc/types";

const hits: QueryHit[] = [
  {
    file_id: "a",
    lens: "filename",
    name: "alpha.txt",
    path: "/p/alpha.txt",
    ext: "txt",
    size: 100,
    modified_ms: 3,
    type: "TXT",
    score: 0.5
  },
  {
    file_id: "b",
    lens: "filename",
    name: "beta.txt",
    path: "/p/beta.txt",
    ext: "txt",
    size: 50,
    modified_ms: 1,
    type: "TXT",
    score: 0.9
  },
  {
    file_id: "c",
    lens: "filename",
    name: "gamma.txt",
    path: "/p/gamma.txt",
    ext: "txt",
    size: 200,
    modified_ms: 2,
    type: "TXT",
    score: 0.7
  }
];

describe("sortStore", () => {
  beforeEach(() => {
    sortStore.setField("name");
    sortStore.setOrder("asc");
  });

  it("sorts by name ascending by default", () => {
    expect(sortStore.applied(hits).map((h) => h.file_id)).toEqual(["a", "b", "c"]);
  });
  it("toggling the same field flips order", () => {
    sortStore.toggle("name");
    expect(sortStore.order).toBe("desc");
    expect(sortStore.applied(hits).map((h) => h.file_id)).toEqual(["c", "b", "a"]);
  });
  it("toggling a different field jumps to ascending", () => {
    sortStore.toggle("size");
    expect(sortStore.field).toBe("size");
    expect(sortStore.order).toBe("asc");
    expect(sortStore.applied(hits).map((h) => h.file_id)).toEqual(["b", "a", "c"]);
  });
  it("similarity sorts by score desc", () => {
    sortStore.setField("similarity");
    expect(sortStore.applied(hits).map((h) => h.file_id)).toEqual(["b", "c", "a"]);
  });
});
