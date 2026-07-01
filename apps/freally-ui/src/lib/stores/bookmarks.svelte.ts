// Bookmarks store — backed by IPC; loads on hydrate.

import * as ipcBookmarks from "../ipc/bookmarks";
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
