// Build 1 (v0.21.0) frontend behaviour — SRC-M01 term extraction,
// SRC-M02 refinement chips, SRC-M03 file-list search, SRC-M06 command
// scoping.

import { describe, it, expect, beforeEach } from "vitest";
import { refineStore } from "../../src/lib/stores/refine.svelte";
import { fileListStore } from "../../src/lib/stores/file_list.svelte";
import { contextMenuStore } from "../../src/lib/stores/context_menu.svelte";
import { literalTerms } from "../../src/lib/stores/content_view.svelte";
import { appliesTo } from "../../src/lib/stores/custom_commands.svelte";
import type { CustomCommand, FileListEntry, ParseReport, QueryHit } from "../../src/lib/ipc/types";

function hit(name: string, path: string): QueryHit {
  return {
    file_id: path,
    lens: "filename",
    name,
    path,
    ext: name.split(".").pop() ?? "",
    size: 1,
    modified_ms: 0,
    type: "TXT",
    score: 1
  };
}

// ---- SRC-M02: search within results -----------------------------------

describe("refinement chips", () => {
  beforeEach(() => refineStore.clear());

  it("narrows by file name, not by path, for a plain term", () => {
    refineStore.draft = "report";
    refineStore.commitDraft();
    const hits = [hit("report.pdf", "/a/report.pdf"), hit("notes.txt", "/report/notes.txt")];
    expect(refineStore.apply(hits).map((h) => h.name)).toEqual(["report.pdf"]);
  });

  it("switches to the full path when the term contains a separator", () => {
    refineStore.draft = "/report/";
    refineStore.commitDraft();
    const hits = [hit("report.pdf", "/a/report.pdf"), hit("notes.txt", "/report/notes.txt")];
    expect(refineStore.apply(hits).map((h) => h.name)).toEqual(["notes.txt"]);
  });

  it("composes chips with AND and peels them off one at a time", () => {
    const hits = [
      hit("q1-report.pdf", "/a/q1-report.pdf"),
      hit("q2-report.pdf", "/a/q2-report.pdf"),
      hit("q1-notes.txt", "/a/q1-notes.txt")
    ];
    refineStore.draft = "report";
    refineStore.commitDraft();
    refineStore.draft = "q1";
    refineStore.commitDraft();
    expect(refineStore.apply(hits).map((h) => h.name)).toEqual(["q1-report.pdf"]);

    // Removing the middle chip must leave the other in place.
    refineStore.removeAt(0);
    expect(refineStore.chips).toEqual(["q1"]);
    expect(refineStore.apply(hits).map((h) => h.name)).toEqual(["q1-report.pdf", "q1-notes.txt"]);
  });

  it("ignores blank and duplicate terms", () => {
    refineStore.draft = "   ";
    refineStore.commitDraft();
    expect(refineStore.chips).toEqual([]);
    refineStore.draft = "Report";
    refineStore.commitDraft();
    refineStore.draft = "report";
    refineStore.commitDraft();
    expect(refineStore.chips).toEqual(["Report"]);
  });

  it("matches case-insensitively", () => {
    refineStore.draft = "REPORT";
    refineStore.commitDraft();
    expect(refineStore.apply([hit("report.pdf", "/a/report.pdf")])).toHaveLength(1);
  });

  it("passes everything through when no chip is set", () => {
    const hits = [hit("a.txt", "/a.txt")];
    expect(refineStore.apply(hits)).toBe(hits);
  });

  it("counts survivors without materialising them", () => {
    const hits = [
      hit("q1-report.pdf", "/a/q1-report.pdf"),
      hit("q2-report.pdf", "/a/q2-report.pdf"),
      hit("notes.txt", "/a/notes.txt")
    ];
    expect(refineStore.countIn(hits)).toBe(3);
    refineStore.draft = "report";
    refineStore.commitDraft();
    expect(refineStore.countIn(hits)).toBe(2);
    expect(refineStore.countIn(hits)).toBe(refineStore.apply(hits).length);
  });
});

// ---- SRC-M03: imported file lists --------------------------------------

describe("file list search", () => {
  const entries: FileListEntry[] = [
    { path: "/wd4tb/Music/track.flac", size: 900, modified_ms: 5, created_ms: 5, attrs: 0 },
    { path: "/wd4tb/Docs/report.pdf", size: 1200, modified_ms: 6, created_ms: 6, attrs: 0 },
    { path: "/wd4tb/Docs", size: 0, modified_ms: 0, created_ms: 0, attrs: 0x10 }
  ];

  beforeEach(() => {
    fileListStore.close();
    fileListStore.setEntries(entries);
    fileListStore.path = "/catalogs/orange-wd.efu";
  });

  it("reports the list's file name for the status bar", () => {
    expect(fileListStore.active).toBe(true);
    expect(fileListStore.label).toBe("orange-wd.efu");
  });

  it("matches names case-insensitively and derives the extension", () => {
    const hits = fileListStore.search("REPORT");
    expect(hits).toHaveLength(1);
    expect(hits[0].name).toBe("report.pdf");
    expect(hits[0].ext).toBe("pdf");
    expect(hits[0].size).toBe(1200);
  });

  it("matches the whole path when the query looks like one", () => {
    expect(fileListStore.search("/Docs/").map((h) => h.name)).toEqual(["report.pdf"]);
  });

  it("returns every entry for an empty query", () => {
    expect(fileListStore.search("")).toHaveLength(3);
  });

  it("carries the directory attribute through so folders render as folders", () => {
    const dir = fileListStore.search("Docs").find((h) => h.path === "/wd4tb/Docs");
    expect(dir?.attrs).toBe(0x10);
  });

  it("goes inactive once closed", () => {
    fileListStore.close();
    expect(fileListStore.active).toBe(false);
    expect(fileListStore.label).toBe("");
  });

  it("handles Windows-style separators in an imported list", () => {
    fileListStore.setEntries([
      { path: "C:\\Users\\mike\\Report.PDF", size: 1, modified_ms: 0, created_ms: 0, attrs: 0 }
    ]);
    const hits = fileListStore.search("report");
    expect(hits[0].name).toBe("Report.PDF");
    expect(hits[0].ext).toBe("pdf");
  });
});

// ---- SRC-M01: which terms get highlighted ------------------------------

describe("hit-viewer term extraction", () => {
  beforeEach(() => refineStore.clear());

  function report(tokens: ParseReport["tokens"]): ParseReport {
    return { source: "", strict_everything: false, ast: null, tokens, errors: [] };
  }

  const span = { start: 0, end: 0 };

  it("keeps literals and quoted strings, drops modifiers and wildcards", () => {
    const r = report([
      { kind: { kind: "literal" }, span, text: "budget" },
      { kind: { kind: "quoted" }, span, text: '"q1 plan"' },
      { kind: { kind: "modifier", name: "size" }, span, text: "size:>1mb" },
      { kind: { kind: "wildcard" }, span, text: "*.pdf" },
      { kind: { kind: "and" }, span, text: "AND" }
    ]);
    expect(literalTerms(r)).toEqual(["budget", "q1 plan"]);
  });

  it("includes active refinement chips", () => {
    refineStore.draft = "draft";
    refineStore.commitDraft();
    expect(literalTerms(report([{ kind: { kind: "literal" }, span, text: "budget" }]))).toEqual([
      "budget",
      "draft"
    ]);
  });

  it("de-duplicates case-insensitively", () => {
    const r = report([
      { kind: { kind: "literal" }, span, text: "Budget" },
      { kind: { kind: "literal" }, span, text: "budget" }
    ]);
    expect(literalTerms(r)).toEqual(["Budget"]);
  });

  it("returns nothing for a null report and no chips", () => {
    expect(literalTerms(null)).toEqual([]);
  });
});

// ---- SRC-M06: custom-command scoping -----------------------------------

describe("custom command extension scoping", () => {
  const cmd = (extensions: string[]): CustomCommand => ({
    id: "1",
    name: "Edit",
    program: "code",
    args: ["{path}"],
    extensions
  });

  it("applies to every file when unscoped", () => {
    expect(appliesTo(cmd([]), "/a/anything.xyz")).toBe(true);
  });

  it("matches the extension case-insensitively", () => {
    expect(appliesTo(cmd(["rs", "TOML"]), "/a/Cargo.toml")).toBe(true);
    expect(appliesTo(cmd(["rs"]), "/a/lib.rs")).toBe(true);
    expect(appliesTo(cmd(["rs"]), "/a/notes.md")).toBe(false);
  });

  it("does not treat a dotted folder name as the file's extension", () => {
    expect(appliesTo(cmd(["cache"]), "/a/.cache/lockfile")).toBe(false);
  });

  it("treats a dotfile as having no extension", () => {
    expect(appliesTo(cmd(["bashrc"]), "/home/mike/.bashrc")).toBe(false);
  });
});

// ---- context menu placement --------------------------------------------

describe("context menu placement", () => {
  it("clamps inside the viewport so a bottom-right click stays visible", () => {
    const target = hit("a.txt", "/a.txt");
    contextMenuStore.openAt(
      { clientX: window.innerWidth + 500, clientY: window.innerHeight + 500 } as MouseEvent,
      target
    );
    expect(contextMenuStore.x).toBeLessThan(window.innerWidth);
    expect(contextMenuStore.y).toBeLessThan(window.innerHeight);
    expect(contextMenuStore.x).toBeGreaterThanOrEqual(4);
    expect(contextMenuStore.y).toBeGreaterThanOrEqual(4);
    // `$state` stores the value behind a proxy, so compare by value.
    expect(contextMenuStore.target).toStrictEqual(target);
    contextMenuStore.close();
    expect(contextMenuStore.target).toBeNull();
  });
});
