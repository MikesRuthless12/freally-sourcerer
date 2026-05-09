<script lang="ts">
  import { excludesStore } from "../../../lib/stores/excludes.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import type { ExcludeRules } from "../../../lib/ipc/types";

  function patch(p: Partial<ExcludeRules>) {
    excludesStore.patch(p);
    settingsDialog.markDirty("indexes.exclude");
  }

  async function addFolder() {
    const picked = await openDialog({ directory: true, multiple: false });
    if (typeof picked === "string") {
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

  function excludeByClass(cls: "video" | "audio" | "image" | "archive" | "executable") {
    const exts: Record<typeof cls, string[]> = {
      video: ["mp4", "mkv", "avi", "mov", "webm"],
      audio: ["mp3", "wav", "flac", "ogg", "aac", "m4a", "aiff"],
      image: ["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff"],
      archive: ["zip", "7z", "tar", "gz", "rar", "bz2"],
      executable: ["exe", "msi", "app", "dmg", "deb", "rpm"]
    };
    const list = exts[cls].map((e) => `*.${e}`).join(";");
    const cur = excludesStore.rules.exclude_files ?? "";
    const next = cur.length > 0 ? `${cur};${list}` : list;
    patch({ exclude_files: next });
  }
</script>

<h1>Exclude</h1>

<Section title="Top-level (E)">
  <Checkbox id="ex-hidden" label="Exclude hidden files and folders"
    checked={excludesStore.rules.exclude_hidden} onChange={(v) => patch({ exclude_hidden: v })} />
  <Checkbox id="ex-system" label="Exclude system files and folders"
    checked={excludesStore.rules.exclude_system} onChange={(v) => patch({ exclude_system: v })} />
  <Checkbox id="ex-list-en" label="Enable exclude list"
    checked={excludesStore.rules.list_enabled} onChange={(v) => patch({ list_enabled: v })} />
</Section>

<Section title="Exclude folders (E)">
  <ul>
    {#each excludesStore.rules.folders as f, i (f + i)}
      <li>
        <span>{f}</span>
        <button type="button" onclick={() => removeFolder(i)}>Remove</button>
      </li>
    {/each}
  </ul>
  <div class="actions">
    <button type="button" onclick={addFolder}>Add Folder…</button>
  </div>
</Section>

<Section title="File globs (E)">
  <TextInput id="ex-include-only" label="Include only files (glob)"
    value={excludesStore.rules.include_only_files ?? ""}
    onChange={(v) => patch({ include_only_files: v.length === 0 ? null : v })} />
  <TextInput id="ex-exclude" label="Exclude files (glob)"
    value={excludesStore.rules.exclude_files ?? ""}
    onChange={(v) => patch({ exclude_files: v.length === 0 ? null : v })} />
</Section>

<Section title="Sourcerer Extras (+)">
  <button type="button" onclick={applyOsRecommended}>Apply OS-recommended excludes</button>
  <div class="cls-row">
    <span>Exclude by extension class:</span>
    <button type="button" onclick={() => excludeByClass("video")}>Video</button>
    <button type="button" onclick={() => excludeByClass("audio")}>Audio</button>
    <button type="button" onclick={() => excludeByClass("image")}>Image</button>
    <button type="button" onclick={() => excludeByClass("archive")}>Archive</button>
    <button type="button" onclick={() => excludeByClass("executable")}>Executable</button>
  </div>
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  ul { list-style: none; margin: 0; padding: 0; max-height: 220px; overflow-y: auto; border: 1px solid var(--border); border-radius: 4px; }
  li { display: flex; justify-content: space-between; align-items: center; padding: 4px 8px; border-bottom: 1px solid var(--border); color: var(--text-primary); font-size: 13px; }
  li:last-child { border-bottom: 0; }
  button { padding: 3px 10px; background: var(--bg-canvas); border: 1px solid var(--border); color: var(--text-primary); border-radius: 3px; cursor: pointer; font: inherit; }
  .actions { margin-top: 6px; }
  .cls-row { display: flex; gap: 6px; align-items: center; padding: 6px 0; flex-wrap: wrap; color: var(--text-primary); font-size: 13px; }
</style>
