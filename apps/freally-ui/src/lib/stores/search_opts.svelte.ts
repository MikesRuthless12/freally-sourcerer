// Search-options store — five toggles surfaced via `Search → Match …` /
// `Search → Enable Regex` menu items. Round-trip through settings IPC so
// the value persists across launches. Phase 12 plumbs each through the
// daemon's query.opts so the parser/executor honors them.

import { settingsStore } from "./settings.svelte";

export type SearchOptKey =
  | "match_case"
  | "match_whole_word"
  | "match_path"
  | "match_diacritics"
  | "enable_regex";

class SearchOptsStore {
  get(key: SearchOptKey): boolean {
    return settingsStore.state.search_opts?.[key] ?? false;
  }

  async toggle(key: SearchOptKey) {
    const cur = settingsStore.state.search_opts ?? {
      match_case: false,
      match_whole_word: false,
      match_path: false,
      match_diacritics: false,
      enable_regex: false
    };
    await settingsStore.patch({
      search_opts: { ...cur, [key]: !cur[key] }
    });
  }
}

export const searchOptsStore = new SearchOptsStore();
