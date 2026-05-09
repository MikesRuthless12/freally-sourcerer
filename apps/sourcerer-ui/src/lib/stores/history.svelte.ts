import * as ipc from "../ipc/history";
import type { HistoryConfig, HistoryUpdate } from "../ipc/types";

const DEFAULT: HistoryConfig = {
  search_history_enabled: true,
  search_history_keep_days: 90,
  run_history_enabled: true,
  run_history_keep_days: 90,
  privacy_mode: false,
  per_lens: { filename: true, content: true, audio: true, similarity: true }
};

class HistoryStore {
  cfg = $state<HistoryConfig>({ ...DEFAULT });
  snapshotCfg: HistoryConfig = { ...DEFAULT };
  loaded = $state(false);

  async hydrate() {
    try {
      this.cfg = await ipc.get();
    } catch (e) {
      console.warn("[history] hydrate failed:", e);
    }
    this.loaded = true;
  }

  snapshot() {
    this.snapshotCfg = JSON.parse(JSON.stringify(this.cfg));
  }
  rollback() {
    this.cfg = JSON.parse(JSON.stringify(this.snapshotCfg));
  }

  async patch(p: HistoryUpdate) {
    this.cfg = { ...this.cfg, ...p };
  }

  async clear() {
    await ipc.clear();
  }

  async flush(): Promise<void> {
    await ipc.set(this.cfg);
    this.snapshotCfg = JSON.parse(JSON.stringify(this.cfg));
  }

  async reset(): Promise<void> {
    this.cfg = { ...DEFAULT };
    await ipc.set(this.cfg);
  }
}

export const historyStore = new HistoryStore();
