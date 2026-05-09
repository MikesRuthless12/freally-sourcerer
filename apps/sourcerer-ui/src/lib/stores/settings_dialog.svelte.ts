// SettingsModel — the Phase 12 settings-dialog state machine.
//
// Holds:
//   * `view`  — the live, editable copy of every setting.
//   * `baseline` — the snapshot taken when the dialog opened. `dirty`
//     compares `view` against this. `Cancel` rolls `view` back to it.
//   * `selected` — the current tree node.
//   * `search`   — the search-the-options box value; filters the tree.
//   * `panelScroll` — per-panel scroll positions, persisted across
//     openings via localStorage.
//
// Apply pipeline:
//   * `apply()` writes `view` to the SettingsStore (which fans out to
//     the daemon for index-affecting fields) and refreshes the
//     `baseline`.
//   * `ok()` calls `apply()` and closes the dialog.
//   * `rollback()` reverts `view` to `baseline`.
//   * `resetPanel(id)` reverts only the fields the named panel owns.

import { settingsStore } from "./settings.svelte";
import { volumesStore } from "./volumes.svelte";
import { foldersStore } from "./folders.svelte";
import { excludesStore } from "./excludes.svelte";
import { networkStore } from "./network.svelte";
import { customExtractorsStore } from "./custom_extractors.svelte";
import { historyStore } from "./history.svelte";

export type PanelId =
  | "general.ui"
  | "general.home"
  | "general.search"
  | "general.results"
  | "general.view"
  | "general.context_menu"
  | "general.fonts_colors"
  | "general.keyboard"
  | "history"
  | "indexes.top"
  | "indexes.volumes"
  | "indexes.folders"
  | "indexes.file_lists"
  | "indexes.exclude"
  | "lenses.filename"
  | "lenses.content"
  | "lenses.audio"
  | "lenses.similarity"
  | "lenses.custom"
  | "network.https"
  | "network.api"
  | "privacy"
  | "logs"
  | "backup"
  | "locale"
  | "about";

export const PANEL_IDS: PanelId[] = [
  "general.ui",
  "general.home",
  "general.search",
  "general.results",
  "general.view",
  "general.context_menu",
  "general.fonts_colors",
  "general.keyboard",
  "history",
  "indexes.top",
  "indexes.volumes",
  "indexes.folders",
  "indexes.file_lists",
  "indexes.exclude",
  "lenses.filename",
  "lenses.content",
  "lenses.audio",
  "lenses.similarity",
  "lenses.custom",
  "network.https",
  "network.api",
  "privacy",
  "logs",
  "backup",
  "locale",
  "about"
];

const SCROLL_STORAGE_KEY = "sourcerer.settings.scroll";
const SELECTED_STORAGE_KEY = "sourcerer.settings.selected";

class SettingsDialogModel {
  selected = $state<PanelId>(loadSelected());
  search = $state("");
  open = $state(false);

  /// Panels with unsaved changes — set is keyed by PanelId. The `Apply`
  /// button enables when this is non-empty.
  dirtyPanels = $state<Set<PanelId>>(new Set());

  /// Per-panel scroll positions, persisted to localStorage.
  scroll = $state<Record<string, number>>(loadScroll());

  setSelected(p: PanelId) {
    this.selected = p;
    if (typeof window !== "undefined") {
      try {
        window.localStorage.setItem(SELECTED_STORAGE_KEY, p);
      } catch {
        // best-effort
      }
    }
  }

  setScroll(p: PanelId, n: number) {
    this.scroll = { ...this.scroll, [p]: n };
    if (typeof window !== "undefined") {
      try {
        window.localStorage.setItem(SCROLL_STORAGE_KEY, JSON.stringify(this.scroll));
      } catch {
        // best-effort
      }
    }
  }

  setSearch(s: string) {
    this.search = s;
  }

  markDirty(p: PanelId) {
    if (!this.dirtyPanels.has(p)) {
      const next = new Set(this.dirtyPanels);
      next.add(p);
      this.dirtyPanels = next;
    }
  }

  markClean(p: PanelId) {
    if (this.dirtyPanels.has(p)) {
      const next = new Set(this.dirtyPanels);
      next.delete(p);
      this.dirtyPanels = next;
    }
  }

  get dirty(): boolean {
    return this.dirtyPanels.size > 0;
  }

  /// Persist all dirty panels in one shot. Each store owns its own
  /// `flush()` that calls into IPC; we just clear the dirty set after.
  async apply(): Promise<void> {
    // Settings store handles UI-side persistence; calls flow into the
    // daemon for index-affecting fields via apply hooks.
    await settingsStore.flush();
    await volumesStore.flush();
    await foldersStore.flush();
    await excludesStore.flush();
    await networkStore.flush();
    await customExtractorsStore.flush();
    await historyStore.flush();
    this.dirtyPanels = new Set();
  }

  async ok(): Promise<void> {
    await this.apply();
    this.openDialog(false);
  }

  rollback(): void {
    settingsStore.rollback();
    volumesStore.rollback();
    foldersStore.rollback();
    excludesStore.rollback();
    networkStore.rollback();
    customExtractorsStore.rollback();
    historyStore.rollback();
    this.dirtyPanels = new Set();
  }

  cancel(): void {
    this.rollback();
    this.openDialog(false);
  }

  /// Reset just the active panel to defaults (per Restore-Defaults
  /// button). Each panel maps to one or more store-level resets.
  async resetPanel(p: PanelId): Promise<void> {
    switch (p) {
      case "general.ui":
      case "general.home":
      case "general.search":
      case "general.results":
      case "general.view":
      case "general.context_menu":
      case "general.fonts_colors":
      case "general.keyboard":
      case "lenses.filename":
      case "privacy":
      case "logs":
      case "backup":
      case "locale":
        await settingsStore.resetPanel(p);
        break;
      case "indexes.top":
      case "indexes.volumes":
        await volumesStore.reset();
        break;
      case "indexes.folders":
      case "indexes.file_lists":
        await foldersStore.reset();
        break;
      case "indexes.exclude":
        await excludesStore.reset();
        break;
      case "lenses.content":
      case "lenses.audio":
      case "lenses.similarity":
        await settingsStore.resetPanel(p);
        break;
      case "lenses.custom":
        await customExtractorsStore.reset();
        break;
      case "network.https":
      case "network.api":
        await networkStore.reset();
        break;
      case "history":
        await historyStore.reset();
        break;
      case "about":
        // About has no editable state; the button is disabled there.
        break;
    }
    this.markClean(p);
  }

  openDialog(open: boolean) {
    this.open = open;
    if (open) {
      // Snapshot every store so Cancel rolls cleanly back.
      settingsStore.snapshot();
      volumesStore.snapshot();
      foldersStore.snapshot();
      excludesStore.snapshot();
      networkStore.snapshot();
      customExtractorsStore.snapshot();
      historyStore.snapshot();
      // Hydrate daemon-backed stores so the panels show real state.
      void volumesStore.hydrate();
      void foldersStore.hydrate();
      void excludesStore.hydrate();
      void networkStore.hydrate();
      void customExtractorsStore.hydrate();
      void historyStore.hydrate();
    } else {
      // Closing without applying means the user clicked Cancel /
      // pressed Esc. The cancel() / ok() entrypoints already cleared
      // dirtyPanels.
      this.dirtyPanels = new Set();
    }
  }
}

function loadSelected(): PanelId {
  if (typeof window === "undefined") return "general.ui";
  try {
    const raw = window.localStorage.getItem(SELECTED_STORAGE_KEY);
    if (raw && (PANEL_IDS as string[]).includes(raw)) {
      return raw as PanelId;
    }
  } catch {
    // best-effort
  }
  return "general.ui";
}

function loadScroll(): Record<string, number> {
  if (typeof window === "undefined") return {};
  try {
    const raw = window.localStorage.getItem(SCROLL_STORAGE_KEY);
    if (raw) {
      const parsed = JSON.parse(raw) as unknown;
      if (parsed && typeof parsed === "object") {
        return parsed as Record<string, number>;
      }
    }
  } catch {
    // best-effort
  }
  return {};
}

export const settingsDialog = new SettingsDialogModel();
