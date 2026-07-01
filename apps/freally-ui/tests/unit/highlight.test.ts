import { describe, it, expect } from "vitest";
import { highlight, firstError } from "../../src/lib/tokenizer/highlight";
import type { ParseReport } from "../../src/lib/ipc/types";

const emptyReport: ParseReport = {
  source: "",
  strict_everything: false,
  ast: null,
  tokens: [],
  errors: []
};

describe("highlight", () => {
  it("returns [] for empty source", () => {
    expect(highlight("", emptyReport)).toEqual([]);
  });

  it("returns a single pending segment when no report yet", () => {
    const segs = highlight("hello", null);
    expect(segs).toHaveLength(1);
    expect(segs[0]!.text).toBe("hello");
    expect(segs[0]!.className).toBe("tok-pending");
    expect(segs[0]!.isError).toBe(false);
  });

  it("emits one segment per token + whitespace gaps", () => {
    const report: ParseReport = {
      source: "size:>1mb foo*",
      strict_everything: false,
      ast: null,
      tokens: [
        {
          kind: { kind: "modifier", name: "size" },
          span: { start: 0, end: 9 },
          text: "size:>1mb"
        },
        {
          kind: { kind: "wildcard" },
          span: { start: 10, end: 14 },
          text: "foo*"
        }
      ],
      errors: []
    };
    const segs = highlight(report.source, report);
    // 2 tokens + 1 gap = 3 segments
    expect(segs.map((s) => s.text)).toEqual(["size:>1mb", " ", "foo*"]);
    expect(segs[0]!.className).toBe("tok-modifier");
    expect(segs[1]!.className).toBe("tok-whitespace");
    expect(segs[2]!.className).toBe("tok-wildcard");
  });

  it("flags tokens overlapping error ranges", () => {
    const report: ParseReport = {
      source: "abc",
      strict_everything: false,
      ast: null,
      tokens: [
        {
          kind: { kind: "literal" },
          span: { start: 0, end: 3 },
          text: "abc"
        }
      ],
      errors: [
        {
          span: { start: 1, end: 2 },
          message: "bad",
          code: "unexpected_token"
        }
      ]
    };
    const segs = highlight(report.source, report);
    expect(segs[0]!.isError).toBe(true);
  });
});

describe("firstError", () => {
  it("returns null when no errors", () => {
    expect(firstError(null)).toBeNull();
    expect(firstError(emptyReport)).toBeNull();
  });
  it("surfaces the first error", () => {
    const r: ParseReport = {
      ...emptyReport,
      source: "x",
      errors: [
        { span: { start: 0, end: 1 }, message: "first", code: "unexpected_token" },
        { span: { start: 0, end: 1 }, message: "second", code: "unexpected_token" }
      ]
    };
    expect(firstError(r)).toEqual({ message: "first", offset: 0 });
  });
});
