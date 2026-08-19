// SRC-M22 — the "Recent searches" section of the sidebar.
//
// Search here is live: `resultsStore.run` fires on every keystroke, so
// recording each run verbatim would fill the list with `r`, `re`, `rep`,
// `repo`… and bury the one entry anybody wants. Two rules fix that
// without needing an explicit "commit" gesture the UI does not have:
//
//   1. When a new query arrives, drop any existing entry that is a
//      prefix of it — those were the keystrokes on the way here.
//   2. Don't record a query that is itself a prefix of the most recent
//      entry, which is what backspacing produces.
//
// The result is that typing "report" then "invoice" leaves exactly
// ["invoice", "report"].

import { settingsStore } from "./settings.svelte";
import { historyStore } from "./history.svelte";

/** Enough to be useful in a sidebar section, few enough to scan. */
const MAX_ITEMS = 12;

/** Queries longer than this are not something anyone re-picks. */
const MAX_LENGTH = 256;

class RecentSearchesStore {
  get items(): string[] {
    return settingsStore.state.recent_searches ?? [];
  }

  /**
   * Whether recording is allowed at all.
   *
   * Privacy Mode and the Search History toggle both already mean "do
   * not keep a record of what I searched for". A sidebar list is
   * exactly such a record, so it honours them rather than quietly
   * keeping its own copy.
   */
  private get enabled(): boolean {
    const cfg = historyStore.cfg;
    return !cfg.privacy_mode && cfg.search_history_enabled;
  }

  async record(query: string) {
    const q = query.trim();
    if (!this.enabled) return;
    if (q === "" || q.length > MAX_LENGTH) return;
    // A bare `*` is the boot query, not something the user searched for.
    if (q === "*") return;

    const current = this.items;
    if (current[0] !== undefined && current[0].startsWith(q) && current[0] !== q) return;

    const next = [q, ...current.filter((e) => e !== q && !q.startsWith(e))].slice(0, MAX_ITEMS);
    if (next.length === current.length && next.every((e, i) => e === current[i])) return;
    this.write(next);
  }

  async clear() {
    this.write([]);
  }

  /**
   * Assign the one key, rather than going through `settingsStore.patch`.
   *
   * `patch` spreads the whole settings object — ~180 keys — and
   * reassigns the `$state` root, which invalidates every component
   * reading any setting. This runs on every forward keystroke, so that
   * was a full settings-wide invalidation per character typed. The
   * state is deeply reactive, so assigning the property reaches exactly
   * the sidebar section that renders the list, and it stays in
   * `SettingsState` so `flush()` still persists it.
   */
  private write(next: string[]) {
    settingsStore.state.recent_searches = next;
  }
}

export const recentSearchesStore = new RecentSearchesStore();
