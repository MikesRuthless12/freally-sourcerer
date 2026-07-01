import * as ipc from "../ipc/volumes";
import type { VolumeInfo, VolumeUpdate } from "../ipc/types";

class VolumesStore {
  list = $state<VolumeInfo[]>([]);
  loading = $state(false);
  /// Snapshot taken on dialog-open for rollback.
  snapshotList: VolumeInfo[] = [];

  async hydrate() {
    this.loading = true;
    try {
      this.list = await ipc.list();
    } catch (e) {
      console.warn("[volumes] hydrate failed:", e);
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

  /// Patch one volume in the list and immediately push to the daemon.
  /// Other panels' Apply pipelines do this lazily; volumes is a
  /// special case because the journal-toggle effect is observable
  /// (events stop flowing) and users expect it to take effect now.
  async update(update: VolumeUpdate): Promise<void> {
    const idx = this.list.findIndex((v) => v.id === update.id);
    if (idx >= 0) {
      const merged: VolumeInfo = { ...this.list[idx] };
      if (update.indexed !== undefined) merged.indexed = update.indexed;
      if (update.journal_enabled !== undefined) merged.journal_enabled = update.journal_enabled;
      if (update.journal_buffer_kb !== undefined) merged.journal_buffer_kb = update.journal_buffer_kb;
      if (update.allocation_delta_kb !== undefined) merged.allocation_delta_kb = update.allocation_delta_kb;
      if (update.include_only !== undefined) merged.include_only = update.include_only;
      if (update.load_recent_changes !== undefined) merged.load_recent_changes = update.load_recent_changes;
      if (update.monitor_changes !== undefined) merged.monitor_changes = update.monitor_changes;
      const next = this.list.slice();
      next[idx] = merged;
      this.list = next;
    }
    await ipc.update(update);
  }

  async recreateJournal(id: string) {
    await ipc.recreateJournal(id);
  }
  async resetStream(id: string) {
    await ipc.resetStream(id);
  }
  async upgradeFanotify() {
    await ipc.upgradeFanotify();
  }
  async remove(id: string) {
    await ipc.remove(id);
    this.list = this.list.filter((v) => v.id !== id);
  }

  async flush(): Promise<void> {
    // No batch apply — `update` already pushed on each call.
  }

  async reset(): Promise<void> {
    // Wipe per-volume overrides on the daemon. The per-row checkboxes
    // re-derive from auto-detection on next hydrate.
    for (const v of this.list) {
      await ipc.update({
        id: v.id,
        indexed: false,
        journal_enabled: false,
        load_recent_changes: false,
        monitor_changes: true
      });
    }
    await this.hydrate();
  }
}

export const volumesStore = new VolumesStore();
