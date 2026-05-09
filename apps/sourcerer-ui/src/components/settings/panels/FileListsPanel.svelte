<script lang="ts">
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";

  let lists = $state<{ id: string; path: string; monitor_changes: boolean }[]>([]);
  let selectedId = $state<string | null>(null);
  let selected = $derived(lists.find((l) => l.id === selectedId) ?? null);

  let format = $state<"text" | "json" | "srcb">("text");
  let autoExport = $state(false);

  async function addList() {
    const picked = await openDialog({ multiple: false, filters: [{ name: "File lists", extensions: ["txt", "json", "srcb"] }] });
    if (typeof picked === "string") {
      const id = `flist-${Date.now()}`;
      lists = [...lists, { id, path: picked, monitor_changes: false }];
      selectedId = id;
      settingsDialog.markDirty("indexes.file_lists");
    }
  }

  function removeSelected() {
    if (!selected) return;
    lists = lists.filter((l) => l.id !== selected!.id);
    selectedId = null;
    settingsDialog.markDirty("indexes.file_lists");
  }

  async function openEditor() {
    const picked = await saveDialog({
      defaultPath: "sourcerer-file-list.txt",
      filters: [{ name: "Text", extensions: ["txt"] }]
    });
    if (typeof picked === "string") {
      // The File List Editor proper is a Phase 13 enhancement; the dialog
      // here saves an empty file list at the chosen path so the user has
      // a starting file to populate.
      try {
        await (await import("@tauri-apps/plugin-fs")).writeTextFile(picked, "");
      } catch (e) {
        console.warn("file list create failed", e);
      }
    }
  }
</script>

<h1>File Lists</h1>

<Section title="File Lists (E)">
  <ul>
    {#each lists as l (l.id)}
      <li>
        <button type="button" class:selected={l.id === selectedId} onclick={() => (selectedId = l.id)}>
          {l.path}
        </button>
      </li>
    {/each}
  </ul>
  <div class="actions">
    <button type="button" onclick={addList}>Add…</button>
    <button type="button" onclick={removeSelected} disabled={!selected}>Remove</button>
  </div>
</Section>

{#if selected}
  <Section title="Settings for selected file list">
    <Checkbox id="fl-monitor" label="Monitor changes (E)"
      checked={selected.monitor_changes}
      onChange={(v) => {
        lists = lists.map((l) => (l.id === selected!.id ? { ...l, monitor_changes: v } : l));
        settingsDialog.markDirty("indexes.file_lists");
      }} />
  </Section>
{/if}

<Section title="Editor + Format (E + +)">
  <button type="button" onclick={openEditor}>File List Editor…</button>
  <Dropdown id="fl-format" label="File list format"
    value={format}
    options={[ { value: "text", label: "Text (one path per line, default)" }, { value: "json", label: "JSON (with per-entry metadata) (+)" }, { value: "srcb", label: "Sourcerer Bundle .srcb (+)" } ]}
    onChange={(v) => { format = v; settingsDialog.markDirty("indexes.file_lists"); }} />
  <Checkbox id="fl-auto-export" label="Auto-export saved searches as file lists (+)"
    checked={autoExport}
    onChange={(v) => { autoExport = v; settingsDialog.markDirty("indexes.file_lists"); }} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  ul { list-style: none; margin: 0; padding: 0; max-height: 200px; overflow-y: auto; border: 1px solid var(--border); border-radius: 4px; }
  li button { width: 100%; text-align: left; background: none; border: 0; color: var(--text-primary); padding: 6px 8px; cursor: pointer; font: inherit; }
  li button:hover { background: var(--bg-surface); }
  li button.selected { background: var(--accent-cyan); color: var(--bg-canvas); }
  .actions { margin-top: 8px; display: flex; gap: 6px; }
  .actions button, button { padding: 4px 10px; background: var(--bg-canvas); border: 1px solid var(--border); color: var(--text-primary); border-radius: 3px; cursor: pointer; font: inherit; }
  .actions button:disabled { opacity: 0.55; cursor: not-allowed; }
</style>
