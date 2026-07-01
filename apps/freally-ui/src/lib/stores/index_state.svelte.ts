// Index state store — mirrors the daemon's indexing pip. Polls every 1s
// while phase != "indexed".

import * as ipcIndex from "../ipc/index_api";
import type { IndexState } from "../ipc/types";

const INITIAL: IndexState = {
  phase: "indexing",
  files_indexed: 0,
  files_total: 0,
  message: "Connecting to indexer…"
};

class IndexStateStore {
  state = $state<IndexState>(INITIAL);
  private timer: ReturnType<typeof setTimeout> | null = null;

  async hydrate() {
    try {
      this.state = await ipcIndex.state();
    } catch (e) {
      console.warn("[index_state] poll failed:", e);
    }
    this.scheduleNext();
  }

  private scheduleNext() {
    if (this.timer) clearTimeout(this.timer);
    if (this.stopped) return;
    // 1s while indexing/error, 5s once settled. Keeps pip animation smooth
    // without burning IPC roundtrips on the steady-state path.
    const interval = this.state.phase === "indexed" ? 5000 : 1000;
    this.timer = setTimeout(() => void this.hydrate(), interval);
  }

  private stopped = false;

  stop() {
    this.stopped = true;
    if (this.timer) clearTimeout(this.timer);
    this.timer = null;
  }
}

export const indexStateStore = new IndexStateStore();
