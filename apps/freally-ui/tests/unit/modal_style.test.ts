// Modal's `style` prop is inline, so it silently beats the shell's `.panel`
// class on any property it names. Four dialogs had reached past metrics into
// chrome — two set `border-radius: 8px` against the shell's 10px, two set
// their own `box-shadow` — so the app carried two dialog corner radii and
// three shadows with nothing to notice it.
//
// The guard is what does the noticing now, so it is tested directly: the
// component only logs what this returns.

import { describe, it, expect } from "vitest";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { chromeOverrides } from "../../src/lib/util/modal_style";

describe("chromeOverrides", () => {
  it("passes metrics through", () => {
    expect(
      chromeOverrides("width: min(960px, 95vw); height: 80vh; display: flex; overflow: hidden;")
    ).toEqual([]);
  });

  it("catches the two overrides that were actually in the tree", () => {
    expect(chromeOverrides("width: 540px; border-radius: 8px;")).toEqual(["border-radius"]);
    expect(chromeOverrides("width: 540px; box-shadow: 0 12px 48px rgba(0,0,0,0.5);")).toEqual([
      "box-shadow"
    ]);
  });

  it("catches longhands, which reach the same surface", () => {
    expect(chromeOverrides("border-color: red; background-image: none;")).toEqual([
      "border-color",
      "background-image"
    ]);
  });

  it("does not fire on metrics whose names merely contain a chrome word", () => {
    // `min-width` is not `width`-prefixed chrome, and `padding` is not `border`.
    expect(chromeOverrides("min-width: 800px; padding: 12px; place-items: center;")).toEqual([]);
  });

  it("tolerates empty and ragged strings", () => {
    expect(chromeOverrides("")).toEqual([]);
    expect(chromeOverrides(";;  ;")).toEqual([]);
    expect(chromeOverrides("width: 10px")).toEqual([]);
  });

  it("reports each property once, in source order", () => {
    expect(chromeOverrides("color: red; width: 1px; color: blue; border: 0;")).toEqual([
      "color",
      "border"
    ]);
  });
});

/**
 * The runtime guard in `Modal.svelte` only fires in dev, only when that
 * particular dialog is actually opened, and only if somebody is watching the
 * console. Its coverage is therefore weakest exactly where the drift happened:
 * two of the four dialogs that had reached into chrome were the first-run
 * wizard and the permission-health report, which a dev session does not open —
 * which is why they drifted for two builds without anyone noticing.
 *
 * This checks every call site at once, from source, whether or not it renders.
 */
describe("no <Modal> call site reaches past metrics into chrome", () => {
  const SRC = join(__dirname, "..", "..", "src");

  function svelteFiles(dir: string): string[] {
    return readdirSync(dir, { withFileTypes: true }).flatMap((e) => {
      const full = join(dir, e.name);
      if (e.isDirectory()) return svelteFiles(full);
      return e.isFile() && e.name.endsWith(".svelte") ? [full] : [];
    });
  }

  it("finds the Modal call sites and none of them sets chrome", () => {
    const offenders: string[] = [];
    let sites = 0;

    for (const file of svelteFiles(SRC)) {
      const text = readFileSync(file, "utf8");
      // Only files that actually mount the shared shell. `Modal.svelte`
      // itself defines the prop rather than passing it.
      if (!text.includes("<Modal") || file.endsWith("Modal.svelte")) continue;
      for (const m of text.matchAll(/<Modal\b[\s\S]*?>/g)) {
        const style = /\bstyle="([^"]*)"/.exec(m[0]);
        if (!style) continue;
        sites++;
        const stolen = chromeOverrides(style[1]!);
        if (stolen.length > 0) {
          offenders.push(`${file.slice(SRC.length + 1)} sets ${stolen.join(", ")}`);
        }
      }
    }

    // Guard the guard: a regex that silently stops matching would make this
    // test pass by checking nothing at all.
    expect(sites).toBeGreaterThan(0);
    expect(offenders).toEqual([]);
  });
});
