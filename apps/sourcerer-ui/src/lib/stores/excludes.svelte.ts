import * as ipc from "../ipc/excludes";
import type { ExcludeRules } from "../ipc/types";

const DEFAULT: ExcludeRules = {
  exclude_hidden: false,
  exclude_system: false,
  list_enabled: true,
  folders: [],
  include_only_files: null,
  exclude_files: null
};

class ExcludesStore {
  rules = $state<ExcludeRules>({ ...DEFAULT });
  snapshotRules: ExcludeRules = { ...DEFAULT };
  loaded = $state(false);

  async hydrate() {
    try {
      this.rules = await ipc.get();
    } catch (e) {
      console.warn("[excludes] hydrate failed:", e);
    }
    this.loaded = true;
  }

  snapshot() {
    this.snapshotRules = JSON.parse(JSON.stringify(this.rules));
  }
  rollback() {
    this.rules = JSON.parse(JSON.stringify(this.snapshotRules));
  }

  async patch(p: Partial<ExcludeRules>) {
    this.rules = { ...this.rules, ...p };
  }

  async flush(): Promise<void> {
    await ipc.set(this.rules);
    this.snapshotRules = JSON.parse(JSON.stringify(this.rules));
  }

  async reset(): Promise<void> {
    this.rules = { ...DEFAULT };
    await ipc.set(this.rules);
  }
}

export const excludesStore = new ExcludesStore();
