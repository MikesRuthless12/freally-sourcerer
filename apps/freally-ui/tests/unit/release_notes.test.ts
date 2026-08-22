// TASK-UP1 — the release notes rendered in the Updates panel.
//
// The notes are the GitHub release body, fetched over the network as part
// of the update manifest. Two things follow from "remote text on screen":
// it is Markdown, so rendering it raw shows literal `**` and an
// unclickable URL; and it is untrusted, so nothing derived from it may
// ever become markup.
//
// These pin both.
import { describe, expect, it } from "vitest";
import { isOpenableUrl, parseReleaseNotes } from "../../src/lib/util/release_notes";

const text = (segs: ReturnType<typeof parseReleaseNotes>) =>
  segs
    .filter((s) => s.kind === "text")
    .map((s) => (s as { value: string }).value)
    .join("");
const links = (segs: ReturnType<typeof parseReleaseNotes>) =>
  segs.filter((s) => s.kind === "link").map((s) => (s as { href: string }).href);

describe("parseReleaseNotes", () => {
  it("returns nothing for empty or blank notes", () => {
    expect(parseReleaseNotes("")).toEqual([]);
    expect(parseReleaseNotes("   \n\n  ")).toEqual([]);
  });

  it("pulls the Full Changelog URL out as a link", () => {
    const segs = parseReleaseNotes(
      "**Full Changelog**: https://github.com/MikesRuthless12/freally-sourcerer/compare/v0.23.2...v0.24.0",
    );
    expect(links(segs)).toEqual([
      "https://github.com/MikesRuthless12/freally-sourcerer/compare/v0.23.2...v0.24.0",
    ]);
    // The emphasis markers go; the words stay.
    expect(text(segs)).toContain("Full Changelog");
    expect(text(segs)).not.toContain("**");
  });

  it("keeps the dots inside a compare URL", () => {
    // `v0.23.2...v0.24.0` is the whole point of a compare link. Stripping
    // trailing punctuation must not eat the range separator.
    const segs = parseReleaseNotes("see https://example.com/compare/v0.23.2...v0.24.0");
    expect(links(segs)).toEqual(["https://example.com/compare/v0.23.2...v0.24.0"]);
  });

  it("leaves a sentence-ending period on the sentence", () => {
    const segs = parseReleaseNotes("Read https://example.com/notes.");
    expect(links(segs)).toEqual(["https://example.com/notes"]);
    expect(text(segs)).toBe("Read .");
  });

  it("drops a paragraph repeated by the release workflow", () => {
    // The release matrix regenerated the body once per build target,
    // which put four identical Full Changelog lines in one release.
    const dup = "Full Changelog: https://example.com/c\n\nFull Changelog: https://example.com/c";
    expect(links(parseReleaseNotes(dup))).toEqual(["https://example.com/c"]);
  });

  it("never emits a link segment for a non-http scheme", () => {
    // The notes are remote. Anything that is not http(s) must stay inert
    // text rather than becoming something clickable.
    const segs = parseReleaseNotes("try javascript:alert(1) or file:///etc/passwd");
    expect(links(segs)).toEqual([]);
    expect(text(segs)).toContain("javascript:alert(1)");
  });
});

describe("isOpenableUrl", () => {
  it("accepts http and https", () => {
    expect(isOpenableUrl("https://example.com")).toBe(true);
    expect(isOpenableUrl("http://example.com")).toBe(true);
  });

  it("refuses every scheme that could act on the machine", () => {
    for (const bad of [
      "javascript:alert(1)",
      "file:///etc/passwd",
      "data:text/html,<script>",
      "vbscript:msgbox",
      "not a url",
      "",
    ]) {
      expect(isOpenableUrl(bad), bad).toBe(false);
    }
  });
});
