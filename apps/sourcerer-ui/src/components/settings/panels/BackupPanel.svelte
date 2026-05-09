<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import Section from "../controls/Section.svelte";
  import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";
  import { writeTextFile, readTextFile } from "@tauri-apps/plugin-fs";

  let toast = $state("");

  async function exportSettings() {
    const path = await saveDialog({
      defaultPath: "sourcerer-settings.toml",
      filters: [{ name: "TOML", extensions: ["toml"] }]
    });
    if (typeof path !== "string") return;
    try {
      const json = JSON.stringify(settingsStore.state, null, 2);
      await writeTextFile(path, json);
      toast = "Exported settings to " + path;
    } catch (e) {
      toast = "Export failed: " + e;
    }
    setTimeout(() => (toast = ""), 4000);
  }

  async function importSettings() {
    const path = await openDialog({ multiple: false, filters: [{ name: "TOML", extensions: ["toml", "json"] }] });
    if (typeof path !== "string") return;
    try {
      const raw = await readTextFile(path);
      const parsed = JSON.parse(raw);
      await settingsStore.patch(parsed);
      await settingsStore.flush();
      toast = "Imported settings";
    } catch (e) {
      toast = "Import failed: " + e;
    }
    setTimeout(() => (toast = ""), 4000);
  }

  async function exportBookmarks() {
    const path = await saveDialog({
      defaultPath: "sourcerer-bookmarks.srcb",
      filters: [{ name: "Sourcerer Bundle", extensions: ["srcb"] }]
    });
    if (typeof path !== "string") return;
    try {
      const bookmarks = await (await import("@tauri-apps/api/core")).invoke<unknown[]>("bookmarks_list");
      await writeTextFile(path, JSON.stringify({ kind: "srcb", bookmarks }, null, 2));
      toast = "Exported bookmarks";
    } catch (e) {
      toast = "Bookmark export failed: " + e;
    }
    setTimeout(() => (toast = ""), 4000);
  }

  async function importBookmarks() {
    const path = await openDialog({ multiple: false, filters: [{ name: "Sourcerer Bundle", extensions: ["srcb", "json"] }] });
    if (typeof path !== "string") return;
    try {
      const raw = await readTextFile(path);
      const parsed = JSON.parse(raw) as { bookmarks?: unknown[] };
      if (Array.isArray(parsed.bookmarks)) {
        const { invoke } = await import("@tauri-apps/api/core");
        for (const b of parsed.bookmarks) {
          await invoke("bookmarks_save", b as Record<string, unknown>);
        }
      }
      toast = "Imported bookmarks";
    } catch (e) {
      toast = "Bookmark import failed: " + e;
    }
    setTimeout(() => (toast = ""), 4000);
  }

  async function resetAll() {
    if (!confirm("Reset all settings to defaults? This cannot be undone (the dialog stays open).")) return;
    await settingsStore.reset();
    toast = "All settings reset";
    setTimeout(() => (toast = ""), 4000);
  }
</script>

<h1>Backup, Export, Reset</h1>

<Section title="Settings (+)">
  <button type="button" onclick={exportSettings}>Export settings</button>
  <button type="button" onclick={importSettings}>Import settings</button>
</Section>

<Section title="Bookmarks + Custom Extractors (+)">
  <button type="button" onclick={exportBookmarks}>Export bookmarks bundle</button>
  <button type="button" onclick={importBookmarks}>Import bookmarks bundle</button>
</Section>

<Section title="Reset">
  <button type="button" class="danger" onclick={resetAll}>Reset all settings to defaults</button>
</Section>

{#if toast}<p class="toast">{toast}</p>{/if}

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  button { padding: 5px 12px; background: var(--bg-canvas); color: var(--text-primary); border: 1px solid var(--border); border-radius: 3px; cursor: pointer; font: inherit; margin-right: 8px; }
  button.danger { background: var(--accent-orange, #c04020); color: #fff; border-color: transparent; }
  .toast { margin-top: 12px; color: var(--accent-cyan); font-size: 12px; }
</style>
