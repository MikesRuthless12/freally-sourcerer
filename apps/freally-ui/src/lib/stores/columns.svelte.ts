// Column model — derived from settings.column_profiles[active]. Mutating a
// column width round-trips through settings IPC so the change persists.

import type { ColumnId, ColumnProfile } from "../ipc/types";
import { settingsStore } from "./settings.svelte";

export interface VisibleColumn {
  id: ColumnId;
  width_px: number;
}

// Fluent keys per column id. Consumers call `t(COLUMN_LABEL_KEYS[id])`
// so the header re-renders on locale switch.
export const COLUMN_LABEL_KEYS: Record<ColumnId, string> = {
  name: "column-name",
  path: "column-path",
  size: "column-size",
  modified: "column-modified",
  type: "column-type",
  ext: "column-ext"
};

export const MIN_COL_WIDTH = 60;
export const MAX_COL_WIDTH = 800;

class ColumnsStore {
  get profile(): ColumnProfile | null {
    const id = settingsStore.state.active_column_profile;
    return settingsStore.state.column_profiles.find((p) => p.id === id) ?? null;
  }

  get visible(): VisibleColumn[] {
    const p = this.profile;
    if (!p) return [];
    return p.columns
      .filter((c) => c.visible)
      .map((c) => ({ id: c.id, width_px: c.width_px }));
  }

  async setWidth(id: ColumnId, width_px: number) {
    const clamped = Math.max(MIN_COL_WIDTH, Math.min(MAX_COL_WIDTH, Math.round(width_px)));
    const profiles = settingsStore.state.column_profiles.map((p) => {
      if (p.id !== settingsStore.state.active_column_profile) return p;
      return {
        ...p,
        columns: p.columns.map((c) => (c.id === id ? { ...c, width_px: clamped } : c))
      };
    });
    await settingsStore.patch({ column_profiles: profiles });
  }

  async setVisible(id: ColumnId, visible: boolean) {
    const profiles = settingsStore.state.column_profiles.map((p) => {
      if (p.id !== settingsStore.state.active_column_profile) return p;
      return {
        ...p,
        columns: p.columns.map((c) => (c.id === id ? { ...c, visible } : c))
      };
    });
    await settingsStore.patch({ column_profiles: profiles });
  }
}

export const columnsStore = new ColumnsStore();
