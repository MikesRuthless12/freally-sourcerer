<script lang="ts">
  import { excludesStore } from "../../../lib/stores/excludes.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import { t } from "../../../lib/i18n/t";
  import type { ExcludeRules } from "../../../lib/ipc/types";

  function patch(p: Partial<ExcludeRules>) {
    excludesStore.patch(p);
    settingsDialog.markDirty("indexes.exclude");
  }

  async function addFolder() {
    const picked = await openDialog({ directory: true, multiple: false });
    if (typeof picked === "string") {
      // Don't re-add a folder that's already in the list.
      if (excludesStore.rules.folders.includes(picked)) return;
      patch({ folders: [...excludesStore.rules.folders, picked] });
    }
  }

  function removeFolder(idx: number) {
    const next = excludesStore.rules.folders.slice();
    next.splice(idx, 1);
    patch({ folders: next });
  }

  function applyOsRecommended() {
    const win = ["%TEMP%", "%LOCALAPPDATA%\\Microsoft\\Windows\\INetCache", "pagefile.sys", "hiberfil.sys", "$Recycle.Bin"];
    const mac = ["~/Library/Caches", "~/.Trash", "/private/var", "*.DS_Store"];
    const lin = ["~/.cache", "~/.thumbnails", "/proc", "/sys", "/dev"];
    const all = Array.from(new Set([...excludesStore.rules.folders, ...win, ...mac, ...lin]));
    patch({ folders: all });
  }

  type Cls = "video" | "audio" | "image" | "archive" | "executable";
  const CLASS_EXTS: Record<Cls, string[]> = {
    video: ["mp4", "mkv", "avi", "mov", "webm"],
    audio: ["mp3", "wav", "flac", "ogg", "aac", "m4a", "aiff"],
    image: ["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff"],
    archive: ["zip", "7z", "tar", "gz", "rar", "bz2"],
    executable: ["exe", "msi", "app", "dmg", "deb", "rpm"]
  };

  function currentGlobs(): Set<string> {
    const cur = excludesStore.rules.exclude_files ?? "";
    const set = new Set<string>();
    for (const part of cur.split(";")) {
      const trimmed = part.trim();
      if (trimmed.length > 0) set.add(trimmed);
    }
    return set;
  }

  /** A class is "active" iff every one of its extensions is already in
   *  the exclude-files glob list. Used to drive button highlight state. */
  function isClassActive(cls: Cls): boolean {
    const set = currentGlobs();
    return CLASS_EXTS[cls].every((e) => set.has(`*.${e}`));
  }

  function toggleClass(cls: Cls) {
    const set = currentGlobs();
    const active = CLASS_EXTS[cls].every((e) => set.has(`*.${e}`));
    if (active) {
      for (const e of CLASS_EXTS[cls]) set.delete(`*.${e}`);
    } else {
      for (const e of CLASS_EXTS[cls]) set.add(`*.${e}`);
    }
    patch({ exclude_files: Array.from(set).join(";") });
  }
</script>

<h1>{t("settings-node-exclude")}</h1>

<Section title={t("section-top-level")}>
  <Checkbox id="ex-hidden" label={t("settings-exclude-hidden")}
    checked={excludesStore.rules.exclude_hidden} onChange={(v) => patch({ exclude_hidden: v })} />
  <Checkbox id="ex-system" label={t("settings-exclude-system")}
    checked={excludesStore.rules.exclude_system} onChange={(v) => patch({ exclude_system: v })} />
  <Checkbox id="ex-list-en" label={t("settings-exclude-list-en")}
    checked={excludesStore.rules.list_enabled} onChange={(v) => patch({ list_enabled: v })} />
</Section>

<Section title={t("settings-exclude-folders")}>
  <ul>
    {#each excludesStore.rules.folders as f, i (f + i)}
      <li>
        <span>{f}</span>
        <button type="button" onclick={() => removeFolder(i)}>{t("settings-vol-remove")}</button>
      </li>
    {/each}
  </ul>
  <div class="actions">
    <button type="button" onclick={addFolder}>Add Folder…</button>
  </div>
</Section>

<Section title={t("section-file-globs")}>
  <TextInput id="ex-include-only" label={t("settings-exclude-include-only-files")}
    value={excludesStore.rules.include_only_files ?? ""}
    onChange={(v) => patch({ include_only_files: v.length === 0 ? null : v })} />
  <TextInput id="ex-exclude" label={t("settings-exclude-files")}
    value={excludesStore.rules.exclude_files ?? ""}
    onChange={(v) => patch({ exclude_files: v.length === 0 ? null : v })} />
</Section>

<Section title={t("folders-section-extras")}>
  <button type="button" onclick={applyOsRecommended}>{t("settings-exclude-os-recommended")}</button>
  <div class="cls-row">
    <span>{t("settings-exclude-by-class")}:</span>
    <button type="button" class:active={isClassActive("video")}
      aria-pressed={isClassActive("video")} onclick={() => toggleClass("video")}>{t("quick-filter-video")}</button>
    <button type="button" class:active={isClassActive("audio")}
      aria-pressed={isClassActive("audio")} onclick={() => toggleClass("audio")}>{t("quick-filter-audio")}</button>
    <button type="button" class:active={isClassActive("image")}
      aria-pressed={isClassActive("image")} onclick={() => toggleClass("image")}>{t("quick-filter-image")}</button>
    <button type="button" class:active={isClassActive("archive")}
      aria-pressed={isClassActive("archive")} onclick={() => toggleClass("archive")}>{t("quick-filter-archive")}</button>
    <button type="button" class:active={isClassActive("executable")}
      aria-pressed={isClassActive("executable")} onclick={() => toggleClass("executable")}>{t("quick-filter-executable")}</button>
  </div>
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  ul { list-style: none; margin: 0; padding: 0; max-height: 220px; overflow-y: auto; border: 1px solid var(--border); border-radius: 4px; }
  li { display: flex; justify-content: space-between; align-items: center; padding: 4px 8px; border-bottom: 1px solid var(--border); color: var(--text-primary); font-size: 13px; }
  li:last-child { border-bottom: 0; }
  button { padding: 3px 10px; background: var(--bg-canvas); border: 1px solid var(--border); color: var(--text-primary); border-radius: 3px; cursor: pointer; font: inherit; }
  button.active {
    background: color-mix(in srgb, var(--accent-cyan) 30%, transparent);
    border-color: var(--accent-cyan);
    color: var(--text-primary);
  }
  .actions { margin-top: 6px; }
  .cls-row { display: flex; gap: 6px; align-items: center; padding: 6px 0; flex-wrap: wrap; color: var(--text-primary); font-size: 13px; }
</style>
