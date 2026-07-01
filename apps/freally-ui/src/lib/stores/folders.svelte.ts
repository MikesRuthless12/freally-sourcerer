import * as ipc from "../ipc/folders";
import type { WatchedFolder } from "../ipc/types";

class FoldersStore {
  list = $state<WatchedFolder[]>([]);
  loading = $state(false);
  snapshotList: WatchedFolder[] = [];

  async hydrate() {
    this.loading = true;
    try {
      this.list = await ipc.list();
    } catch (e) {
      console.warn("[folders] hydrate failed:", e);
    } finally {
      this.loading = false;
    }
  }

  snapshot() {
    this.snapshotList = JSON.parse(JSON.stringify(this.list));
  }
  rollback() {
    this.list = JSON.parse(JSON.stringify(this.snapshotList));
  }

  async add(folder: WatchedFolder) {
    await ipc.add(folder);
    this.list = [...this.list.filter((f) => f.id !== folder.id), folder];
  }

  async update(folder: WatchedFolder) {
    await ipc.update(folder);
    this.list = this.list.map((f) => (f.id === folder.id ? folder : f));
  }

  async remove(id: string) {
    await ipc.remove(id);
    this.list = this.list.filter((f) => f.id !== id);
  }

  async rescan(id: string) {
    await ipc.rescan(id);
  }

  async rescanAll() {
    await ipc.rescanAll();
  }

  async flush(): Promise<void> {
    // Add/update/remove all push immediately.
  }

  async reset(): Promise<void> {
    for (const f of [...this.list]) {
      await this.remove(f.id);
    }
    await this.hydrate();
  }
}

export const foldersStore = new FoldersStore();
