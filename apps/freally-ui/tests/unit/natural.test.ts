// SRC-M24 — natural sort, UI side.
//
// The `natural_matches_the_rust_twin` case is mirrored in
// `crates/freally-query/src/natural.rs`, so a change made to one
// comparator and not the other fails on one side. It is deliberately
// all-lowercase ASCII: the two differ on case and accents by design
// (this side runs non-digit runs through `localeCompare`), so a shared
// vector may only exercise the numeric behaviour they must agree on.

import { describe, it, expect } from "vitest";
import { naturalCompare } from "../../src/lib/util/natural";

const sorted = (v: string[]) => [...v].sort(naturalCompare);

describe("naturalCompare", () => {
  it("orders digit runs by value, not by first character", () => {
    expect(sorted(["file10", "file2", "file1"])).toEqual(["file1", "file2", "file10"]);
  });

  it("orders version numbers segment by segment", () => {
    expect(sorted(["v1.10", "v1.9", "v1.2"])).toEqual(["v1.2", "v1.9", "v1.10"]);
  });

  it("treats leading zeros as padding, not value", () => {
    expect(naturalCompare("file007", "file7")).toBeGreaterThan(0);
    // Padding loses to a real difference later in the string.
    expect(naturalCompare("file007a", "file7b")).toBeLessThan(0);
  });

  it("reads an all-zero run as the number zero", () => {
    expect(naturalCompare("f0", "f1")).toBeLessThan(0);
    expect(naturalCompare("f000", "f1")).toBeLessThan(0);
  });

  it("compares runs longer than a double can hold", () => {
    expect(
      naturalCompare("f99999999999999999999999999999", "f99999999999999999999999999999999")
    ).toBeLessThan(0);
  });

  it("sorts a prefix before what extends it", () => {
    expect(naturalCompare("file", "file1")).toBeLessThan(0);
    expect(naturalCompare("file1", "file")).toBeGreaterThan(0);
  });

  it("leaves digit-free strings to localeCompare", () => {
    expect(naturalCompare("apple", "banana")).toBe("apple".localeCompare("banana"));
    expect(naturalCompare("a", "a")).toBe(0);
  });

  it("is antisymmetric, so a sort cannot depend on input order", () => {
    const items = ["a1", "a01", "a001", "a2", "a10", "b1", "a"];
    // Collected rather than asserted in the loop so a failure names the
    // offending pairs. `!==` also sidesteps `Object.is(+0, -0)`, which
    // would report a tie as a broken pair.
    const broken: string[] = [];
    for (const x of items) {
      for (const y of items) {
        if (Math.sign(naturalCompare(x, y)) !== -Math.sign(naturalCompare(y, x))) {
          broken.push(`${x} vs ${y}`);
        }
      }
    }
    expect(broken).toEqual([]);
  });

  it("natural_matches_the_rust_twin", () => {
    expect(
      sorted(["img12.png", "img10.png", "img2.png", "img1.png", "img3.png", "img.png"])
    ).toEqual(["img.png", "img1.png", "img2.png", "img3.png", "img10.png", "img12.png"]);
  });
});
