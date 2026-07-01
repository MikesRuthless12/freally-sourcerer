import * as ipc from "../ipc/custom_extractors";
import type { CustomExtractorEntry } from "../ipc/types";

class CustomExtractorsStore {
  list = $state<CustomExtractorEntry[]>([]);
  loading = $state(false);
  snapshotList: CustomExtractorEntry[] = [];

  async hydrate() {
    this.loading = true;
    try {
      this.list = await ipc.list();
    } catch (e) {
      console.warn("[custom_extractors] hydrate failed:", e);
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

  async setTrusted(id: string, trusted: boolean) {
    await ipc.setTrusted(id, trusted);
    this.list = this.list.map((e) =>
      e.id === id ? { ...e, trusted } : e
    );
  }

  async refreshHashes() {
    await ipc.refreshHashes();
    await this.hydrate();
  }

  async flush(): Promise<void> {
    // Trust changes push on click; nothing to flush.
  }

  async reset(): Promise<void> {
    for (const e of this.list) {
      if (e.trusted) await this.setTrusted(e.id, false);
    }
    await this.hydrate();
  }
}

export const customExtractorsStore = new CustomExtractorsStore();
