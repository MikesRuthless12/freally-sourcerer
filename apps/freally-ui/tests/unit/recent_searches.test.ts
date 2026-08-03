// SRC-M22 — the sidebar's Recent searches list.
//
// Search is live, so `run()` fires on every keystroke. Without the
// prefix rules the list would be `r`, `re`, `rep`, `repo`, `repor`,
// `report` and the one entry anybody wants would be buried.

import { describe, it, expect, beforeEach } from "vitest";
import { recentSearchesStore } from "../../src/lib/stores/recent_searches.svelte";
import { settingsStore } from "../../src/lib/stores/settings.svelte";
import { historyStore } from "../../src/lib/stores/history.svelte";

async function type(...queries: string[]) {
  for (const q of queries) await recentSearchesStore.record(q);
}

describe("recentSearchesStore", () => {
  beforeEach(() => {
    // Patch locally rather than through IPC — these tests are about the
    // collapse rules, not the settings round-trip.
    settingsStore.state.recent_searches = [];
    settingsStore.patch = async (p) => {
      Object.assign(settingsStore.state, p);
    };
    historyStore.cfg = { ...historyStore.cfg, privacy_mode: false, search_history_enabled: true };
  });

  it("collapses the keystrokes that led to a query", async () => {
    await type("r", "re", "rep", "repo", "repor", "report");
    expect(recentSearchesStore.items).toEqual(["report"]);
  });

  it("keeps distinct searches, most recent first", async () => {
    await type("report", "invoice");
    expect(recentSearchesStore.items).toEqual(["invoice", "report"]);
  });

  it("ignores a backspace back into a prefix of the newest entry", async () => {
    await type("report", "repor");
    expect(recentSearchesStore.items).toEqual(["report"]);
  });

  it("moves a repeated search back to the top instead of duplicating it", async () => {
    await type("report", "invoice", "report");
    expect(recentSearchesStore.items).toEqual(["report", "invoice"]);
  });

  it("skips the bare-star boot query and blank input", async () => {
    await type("*", "", "   ");
    expect(recentSearchesStore.items).toEqual([]);
  });

  it("records nothing while Privacy Mode is on", async () => {
    historyStore.cfg = { ...historyStore.cfg, privacy_mode: true };
    await type("report");
    expect(recentSearchesStore.items).toEqual([]);
  });

  it("records nothing while Search History is off", async () => {
    historyStore.cfg = { ...historyStore.cfg, search_history_enabled: false };
    await type("report");
    expect(recentSearchesStore.items).toEqual([]);
  });

  it("caps the list", async () => {
    // Distinct, non-prefix entries so nothing collapses.
    await type(...Array.from({ length: 30 }, (_, i) => `q${i}x`));
    expect(recentSearchesStore.items.length).toBeLessThanOrEqual(12);
    expect(recentSearchesStore.items[0]).toBe("q29x");
  });

  it("clears on request", async () => {
    await type("report");
    await recentSearchesStore.clear();
    expect(recentSearchesStore.items).toEqual([]);
  });
});
