import { describe, it, expect } from "vitest";
import { BINDINGS, formatShortcut, shortcutMatches } from "../../src/lib/commands/shortcuts";

describe("BINDINGS", () => {
  it("has no duplicate (key + modifiers) shapes", () => {
    const seen = new Set<string>();
    for (const b of BINDINGS) {
      const sig = JSON.stringify(b.shortcut);
      expect(seen.has(sig), `duplicate binding signature: ${sig}`).toBe(false);
      seen.add(sig);
    }
  });
  it("every binding targets a real command id", () => {
    // The CommandId type is enforced at compile time; runtime check just
    // asserts the table is non-empty and well-shaped.
    for (const b of BINDINGS) {
      expect(b.command).toMatch(/^[a-z_]+(\.[a-z_]+)+$/);
    }
  });
});

describe("shortcutMatches", () => {
  it("matches a Ctrl+S keydown to file.export_results binding", () => {
    const ev = new KeyboardEvent("keydown", { key: "s", ctrlKey: true });
    Object.defineProperty(ev, "code", { value: "KeyS" });
    const b = BINDINGS.find((b) => b.command === "file.export_results");
    expect(b).toBeDefined();
    // On non-Mac, mod === ctrl. shortcutMatches reads navigator.platform.
    // jsdom defaults to a Linux-ish UA, so ctrl matches mod.
    expect(shortcutMatches(ev, b!.shortcut)).toBe(true);
  });
  it("rejects a key without the required modifier", () => {
    const ev = new KeyboardEvent("keydown", { key: "s" });
    Object.defineProperty(ev, "code", { value: "KeyS" });
    const b = BINDINGS.find((b) => b.command === "file.export_results");
    expect(shortcutMatches(ev, b!.shortcut)).toBe(false);
  });
});

describe("formatShortcut", () => {
  it("renders mod-prefixed shortcuts", () => {
    const s = formatShortcut({ key: "s", mod: true });
    expect(s).toMatch(/Ctrl|⌘/);
    expect(s).toMatch(/S/);
  });
});
