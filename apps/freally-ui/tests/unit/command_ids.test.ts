import { describe, it, expect } from "vitest";
import { COMMAND_IDS, isCommandId } from "../../src/lib/commands/ids";
import { iterItems, MENU_BAR } from "../../src/lib/commands/menu_spec";

describe("COMMAND_IDS", () => {
  it("contains the seven top-level menu groups by prefix", () => {
    for (const prefix of ["file.", "edit.", "view.", "search.", "bookmarks.", "tools.", "help."]) {
      const has = COMMAND_IDS.some((id) => id.startsWith(prefix));
      expect(has, `expected at least one ${prefix}* id`).toBe(true);
    }
  });
  it("has no duplicates", () => {
    const set = new Set(COMMAND_IDS);
    expect(set.size).toBe(COMMAND_IDS.length);
  });
});

describe("isCommandId", () => {
  it("accepts known ids", () => {
    expect(isCommandId("file.exit")).toBe(true);
    expect(isCommandId("view.theme.dark")).toBe(true);
    expect(isCommandId("help.about")).toBe(true);
  });
  it("rejects unknown ids", () => {
    expect(isCommandId("not.a.real.id")).toBe(false);
    expect(isCommandId("")).toBe(false);
    expect(isCommandId("file.does_not_exist")).toBe(false);
  });
});

describe("MENU_BAR ↔ COMMAND_IDS lockstep", () => {
  it("every menu item id is a known CommandId", () => {
    for (const item of iterItems()) {
      expect(isCommandId(item.id), `menu item ${item.id} not in COMMAND_IDS`).toBe(true);
    }
  });
  it("seven top-level roots in PRD §8.28 order", () => {
    expect(MENU_BAR.map((r) => r.label)).toEqual([
      "File",
      "Edit",
      "View",
      "Search",
      "Bookmarks",
      "Tools",
      "Help"
    ]);
  });
});
