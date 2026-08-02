// Build 2 (v0.22.0) frontend behaviour — SRC-M13 Index Health panel.

import { describe, it, expect, beforeEach } from "vitest";
import { FluentBundle, FluentResource } from "@fluent/bundle";
import { COMMAND_IDS } from "../../src/lib/commands/ids";
import { iterItems } from "../../src/lib/commands/menu_spec";
import { BINDINGS } from "../../src/lib/commands/shortcuts";
import { dialogsStore } from "../../src/lib/stores/dialogs.svelte";
import type { AdvisoryId } from "../../src/lib/ipc/types";

const EN = import.meta.glob<string>("../../../../locales/en/freally.ftl", {
  query: "?raw",
  import: "default",
  eager: true
});

function enBundle(): FluentBundle {
  const source = Object.values(EN)[0];
  const bundle = new FluentBundle("en");
  const errs = bundle.addResource(new FluentResource(source));
  if (errs.length > 0) throw new Error(`Fluent parse errors: ${errs.map(String).join(" | ")}`);
  return bundle;
}

// ---- SRC-M13: the panel is reachable ----------------------------------

describe("index health panel wiring", () => {
  beforeEach(() => dialogsStore.close());

  it("registers the command id the menu item points at", () => {
    expect(COMMAND_IDS).toContain("tools.index_health");
  });

  it("surfaces Index Health as a real menu item", () => {
    const ids = [...iterItems()].map((i) => i.id);
    expect(ids).toContain("tools.index_health");
  });

  it("is a modal the dialog store can hold open", () => {
    dialogsStore.open("index_health");
    expect(dialogsStore.active).toBe("index_health");
    dialogsStore.close();
    expect(dialogsStore.active).toBeNull();
  });
});

// ---- SRC-M13: every advisory the daemon can send renders --------------

describe("advisory messages", () => {
  // The panel builds its key as `health-advice-${id.replace(/_/g, "-")}`.
  // A daemon-side AdvisoryId with no matching Fluent key would render as
  // the raw key string in the UI, so pin the mapping here.
  const IDS: AdvisoryId[] = [
    "journal_stream_reset",
    "events_dropped",
    "not_monitoring",
    "high_lag",
    "queue_saturated"
  ];

  it("has an English message for every advisory id", () => {
    const bundle = enBundle();
    for (const id of IDS) {
      const key = `health-advice-${id.replace(/_/g, "-")}`;
      const msg = bundle.getMessage(key);
      expect(msg?.value, `missing Fluent key ${key}`).toBeTruthy();
    }
  });

  it("interpolates the root and the count the rules send", () => {
    const bundle = enBundle();
    const msg = bundle.getMessage("health-advice-events-dropped");
    const out = bundle.formatPattern(msg!.value!, { root: "C:\\", count: 3412 }, []);
    // Fluent inserts directional isolate marks around placeables.
    expect(out.replace(/[\u2068\u2069]/g, "")).toContain("3,412");
    expect(out.replace(/[\u2068\u2069]/g, "")).toContain("C:\\");
  });

  it("has the offline-catalog badge strings SRC-M14 renders", () => {
    const bundle = enBundle();
    const msg = bundle.getMessage("results-offline-badge");
    expect(msg?.value).toBeTruthy();
    const out = bundle.formatPattern(msg!.value!, { name: "Orange WD 4TB" }, []);
    expect(out.replace(/[⁨⁩]/g, "")).toContain("Orange WD 4TB");
    // The badge has to explain itself on hover — an unexplained
    // "offline" on a result row reads like an error.
    expect(bundle.getMessage("results-offline-badge-title")?.value).toBeTruthy();
  });

  it("has a message for every rename status and invalid reason SRC-M15 can send", () => {
    const bundle = enBundle();
    // The dialog builds its key from the backend enum, so a variant with
    // no Fluent key renders as the raw key string in the table.
    const statuses = ["ok", "unchanged", "invalid", "collision", "exists"];
    for (const s of statuses) {
      expect(bundle.getMessage(`rename-status-${s}`)?.value, `rename-status-${s}`).toBeTruthy();
    }
    const reasons = [
      "empty",
      "path_separator",
      "dot_name",
      "forbidden_character",
      "reserved_name",
      "bad_pattern"
    ];
    for (const r of reasons) {
      const key = `rename-invalid-${r.replace(/_/g, "-")}`;
      expect(bundle.getMessage(key)?.value, key).toBeTruthy();
    }
  });

  it("wires the SRC-M15/M16 commands, menu items and shortcuts", () => {
    for (const id of ["edit.bulk_rename", "edit.undo", "edit.redo"]) {
      expect(COMMAND_IDS).toContain(id);
    }
    const menuIds = [...iterItems()].map((i) => i.id);
    for (const id of ["edit.bulk_rename", "edit.undo", "edit.redo"]) {
      expect(menuIds, `${id} missing from the menu`).toContain(id);
    }
    const bound = BINDINGS.map((b) => b.command);
    expect(bound).toContain("edit.undo");
    expect(bound).toContain("edit.redo");
    expect(bound).toContain("edit.bulk_rename");
  });

  it("distinguishes undo from redo by the shift modifier", () => {
    const undo = BINDINGS.find((b) => b.command === "edit.undo")!;
    const redo = BINDINGS.find((b) => b.command === "edit.redo")!;
    expect(undo.shortcut.key).toBe("z");
    expect(redo.shortcut.key).toBe("z");
    expect(undo.shortcut.shift ?? false).toBe(false);
    expect(redo.shortcut.shift).toBe(true);
  });

  it("has the panel's own chrome strings", () => {
    const bundle = enBundle();
    for (const key of [
      "health-title",
      "health-all-good",
      "health-no-watched-roots",
      "health-status-live",
      "health-status-scan-only",
      "health-fix-rebuild",
      "health-extraction-untracked",
      "menu-tools-index-health"
    ]) {
      expect(bundle.getMessage(key)?.value, `missing Fluent key ${key}`).toBeTruthy();
    }
  });
});
