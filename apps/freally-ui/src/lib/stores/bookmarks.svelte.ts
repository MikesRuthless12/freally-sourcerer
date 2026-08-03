// Bookmarks store — backed by IPC; loads on hydrate.

import * as ipcBookmarks from "../ipc/bookmarks";
import { typeFilterStore } from "./type_filter.svelte";
import { queryStore } from "./query.svelte";
import { resultsStore } from "./results.svelte";
import type { Bookmark } from "../ipc/types";

class BookmarksStore {
  items = $state<Bookmark[]>([]);

  async hydrate() {
    // The first hydrate often fires before the background daemon-boot
    // thread has finished — IPC throws "daemon not initialized" and we
    // end up with an empty list. Retry briefly so the dropdown +
    // Organize dialog have data on first open.
    for (let attempt = 0; attempt < 20; attempt++) {
      try {
        this.items = await ipcBookmarks.list();
        return;
      } catch (e) {
        if (attempt === 0) console.warn("[bookmarks] hydrate retrying:", e);
        await new Promise((r) => setTimeout(r, 500));
      }
    }
    console.warn("[bookmarks] hydrate gave up after 10s");
  }

  /**
   * Load a bookmark into the search bar.
   *
   * Lives here rather than in each component because the chip
   * selection has to be restored *before* the query runs — otherwise
   * `resultsStore.run()` composes a different lens prefix than the one
   * the bookmark was saved with, and the saved filters are silently
   * discarded. The sidebar (SRC-M22) got that wrong by re-implementing
   * the dropdown's flow instead of sharing it.
   */
  async apply(bm: Bookmark) {
    typeFilterStore.setFromIds(bm.filters ?? []);
    await queryStore.setSource(bm.query);
    await resultsStore.run(bm.query);
  }

  async add(name: string, query: string, filters?: string[]) {
    const bm = await ipcBookmarks.save(name, query, filters);
    this.items = [...this.items, bm];
  }

  async remove(id: string) {
    await ipcBookmarks.remove(id);
    this.items = this.items.filter((b) => b.id !== id);
  }

  async rename(id: string, name: string) {
    await ipcBookmarks.rename(id, name);
    this.items = this.items.map((b) => (b.id === id ? { ...b, name } : b));
  }
}

export const bookmarksStore = new BookmarksStore();
