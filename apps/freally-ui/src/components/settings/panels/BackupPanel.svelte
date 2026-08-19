<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import Section from "../controls/Section.svelte";
  import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";
  import { writeTextFile, readTextFile } from "@tauri-apps/plugin-fs";
  import { t } from "../../../lib/i18n/t";

  let toast = $state("");

  async function exportSettings() {
    const path = await saveDialog({
      defaultPath: "freally-settings.toml",
      filters: [{ name: "TOML", extensions: ["toml"] }]
    });
    if (typeof path !== "string") return;
    try {
      const json = JSON.stringify(settingsStore.state, null, 2);
      await writeTextFile(path, json);
      toast = t("backup-toast-exported", { path });
    } catch (e) {
      toast = t("backup-toast-export-failed", { error: String(e) });
    }
    setTimeout(() => (toast = ""), 4000);
  }

  async function importSettings() {
    const path = await openDialog({ multiple: false, filters: [{ name: "TOML", extensions: ["toml", "json"] }] });
    if (typeof path !== "string") return;
    try {
      const raw = await readTextFile(path);
      const parsed = JSON.parse(raw);
      // A settings file can define custom commands, and a custom command
      // is a program to run — `run_custom_command` spawns its `program`
      // with its `args`. Importing one silently would turn "here is my
      // Freally setup" into a launcher the user never wrote, sitting in
      // every result's context menu under a name its author chose. So the
      // programs get named and the user gets asked; declining imports
      // everything else. That is the whole difference between installed
      // and consented.
      // `parsed` is `any` out of `JSON.parse`, so the element type has to
      // be stated or the callback below is an implicit `any`. `unknown`
      // rather than `string`: this is an untrusted file, and the only
      // thing done with the value is naming it back to the user.
      const incoming: Array<{ program?: unknown }> = Array.isArray(parsed?.custom_commands)
        ? parsed.custom_commands
        : [];
      if (incoming.length > 0) {
        const named = incoming
          .map((c) => c?.program)
          .filter((p): p is string => typeof p === "string" && p !== "");
        const programs = [...new Set(named)].join(", ");
        if (!confirm(t("backup-confirm-import-commands", { programs }))) {
          delete parsed.custom_commands;
        }
      }
      await settingsStore.patch(parsed);
      await settingsStore.flush();
      toast = t("backup-toast-imported");
    } catch (e) {
      toast = t("backup-toast-import-failed", { error: String(e) });
    }
    setTimeout(() => (toast = ""), 4000);
  }

  async function exportBookmarks() {
    const path = await saveDialog({
      defaultPath: "freally-bookmarks.srcb",
      filters: [{ name: "Freally Bundle", extensions: ["srcb"] }]
    });
    if (typeof path !== "string") return;
    try {
      const bookmarks = await (await import("@tauri-apps/api/core")).invoke<unknown[]>("bookmarks_list");
      await writeTextFile(path, JSON.stringify({ kind: "srcb", bookmarks }, null, 2));
      toast = t("backup-toast-bookmarks-exported");
    } catch (e) {
      toast = t("backup-toast-bookmarks-export-failed", { error: String(e) });
    }
    setTimeout(() => (toast = ""), 4000);
  }

  async function importBookmarks() {
    const path = await openDialog({ multiple: false, filters: [{ name: "Freally Bundle", extensions: ["srcb", "json"] }] });
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
      toast = t("backup-toast-bookmarks-imported");
    } catch (e) {
      toast = t("backup-toast-bookmarks-import-failed", { error: String(e) });
    }
    setTimeout(() => (toast = ""), 4000);
  }

  async function resetAll() {
    if (!confirm(t("backup-confirm-reset"))) return;
    await settingsStore.reset();
    toast = t("backup-toast-reset");
    setTimeout(() => (toast = ""), 4000);
  }
</script>

<h1>{t("settings-group-backup")}</h1>

<Section title={t("backup-section-settings")}>
  <button type="button" onclick={exportSettings}>{t("settings-backup-export")}</button>
  <button type="button" onclick={importSettings}>{t("settings-backup-import")}</button>
</Section>

<Section title={t("backup-section-bookmarks")}>
  <button type="button" onclick={exportBookmarks}>{t("settings-backup-export-bookmarks")}</button>
  <button type="button" onclick={importBookmarks}>{t("settings-backup-import-bookmarks")}</button>
</Section>

<Section title={t("backup-section-reset")}>
  <button type="button" class="danger" onclick={resetAll}>{t("settings-backup-reset-all")}</button>
</Section>

{#if toast}<p class="toast">{toast}</p>{/if}

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  button { padding: 5px 12px; background: var(--bg-canvas); color: var(--text-primary); border: 1px solid var(--border); border-radius: 3px; cursor: pointer; font: inherit; margin-right: 8px; }
  button.danger { background: var(--accent-orange, #c04020); color: #fff; border-color: transparent; }
  .toast { margin-top: 12px; color: var(--accent-cyan); font-size: 12px; }
</style>
