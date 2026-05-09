<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { IndexCoreSettings } from "../../../lib/ipc/types";

  function patch(p: Partial<IndexCoreSettings>) {
    settingsStore.patch({ index_core: { ...settingsStore.state.index_core, ...p } });
    settingsDialog.markDirty("indexes.top");
  }

  let busy = $state<{ verify: boolean; compact: boolean; rebuild: boolean }>({
    verify: false,
    compact: false,
    rebuild: false
  });
  let toast = $state("");

  async function fire(method: string, key: keyof typeof busy) {
    if (busy[key]) return;
    busy = { ...busy, [key]: true };
    toast = "";
    try {
      await invoke(method);
      toast = `${method.replace("index_", "Index ")} succeeded`;
    } catch (e) {
      toast = `Failed: ${e}`;
    } finally {
      busy = { ...busy, [key]: false };
      setTimeout(() => (toast = ""), 3000);
    }
  }
</script>

<h1>Indexes</h1>

<Section title="Storage (E)">
  <TextInput id="ix-db-loc" label="Database location" value={settingsStore.state.index_core.database_location}
    onChange={(v) => patch({ database_location: v })} />
  <Checkbox id="ix-multiuser" label="Multi-user database filename"
    checked={settingsStore.state.index_core.multi_user_database_filename}
    onChange={(v) => patch({ multi_user_database_filename: v })} />
  <Checkbox id="ix-compress" label="Compress database"
    checked={settingsStore.state.index_core.compress_database}
    onChange={(v) => patch({ compress_database: v })} />
</Section>

<Section title="Index Fields (E)">
  <Checkbox id="ix-recent" label="Index recent changes"
    checked={settingsStore.state.index_core.index_recent_changes}
    onChange={(v) => patch({ index_recent_changes: v })} />
  <Checkbox id="ix-fsize" label="Index file size"
    checked={settingsStore.state.index_core.index_file_size}
    onChange={(v) => patch({ index_file_size: v })} />
  <Checkbox id="ix-fsize-fast" label="Fast size sort"
    checked={settingsStore.state.index_core.fast_size_sort}
    onChange={(v) => patch({ fast_size_sort: v })} />
  <Checkbox id="ix-fold-size" label="Index folder size"
    checked={settingsStore.state.index_core.index_folder_size}
    onChange={(v) => patch({ index_folder_size: v })} />
  <Checkbox id="ix-fold-size-fast" label="Fast folder size sort"
    checked={settingsStore.state.index_core.fast_folder_size_sort}
    onChange={(v) => patch({ fast_folder_size_sort: v })} />
  <Checkbox id="ix-dc" label="Index date created"
    checked={settingsStore.state.index_core.index_date_created}
    onChange={(v) => patch({ index_date_created: v })} />
  <Checkbox id="ix-dc-fast" label="Fast date created sort"
    checked={settingsStore.state.index_core.fast_date_created_sort}
    onChange={(v) => patch({ fast_date_created_sort: v })} />
  <Checkbox id="ix-dm" label="Index date modified"
    checked={settingsStore.state.index_core.index_date_modified}
    onChange={(v) => patch({ index_date_modified: v })} />
  <Checkbox id="ix-dm-fast" label="Fast date modified sort"
    checked={settingsStore.state.index_core.fast_date_modified_sort}
    onChange={(v) => patch({ fast_date_modified_sort: v })} />
  <Checkbox id="ix-da" label="Index date accessed"
    checked={settingsStore.state.index_core.index_date_accessed}
    onChange={(v) => patch({ index_date_accessed: v })} />
  <Checkbox id="ix-da-fast" label="Fast date accessed sort"
    checked={settingsStore.state.index_core.fast_date_accessed_sort}
    onChange={(v) => patch({ fast_date_accessed_sort: v })} />
  <Checkbox id="ix-attr" label="Index attributes"
    checked={settingsStore.state.index_core.index_attributes}
    onChange={(v) => patch({ index_attributes: v })} />
  <Checkbox id="ix-attr-fast" label="Fast attributes sort"
    checked={settingsStore.state.index_core.fast_attributes_sort}
    onChange={(v) => patch({ fast_attributes_sort: v })} />
  <Checkbox id="ix-path-fast" label="Fast path sort"
    checked={settingsStore.state.index_core.fast_path_sort}
    onChange={(v) => patch({ fast_path_sort: v })} />
  <Checkbox id="ix-ext-fast" label="Fast extension sort"
    checked={settingsStore.state.index_core.fast_extension_sort}
    onChange={(v) => patch({ fast_extension_sort: v })} />
</Section>

<Section title="Maintenance">
  <button type="button" onclick={() => fire("index_rebuild", "rebuild")} disabled={busy.rebuild}>
    {busy.rebuild ? "Rebuilding…" : "Force Rebuild"}
  </button>
  <button type="button" onclick={() => fire("index_compact", "compact")} disabled={busy.compact}>
    {busy.compact ? "Compacting…" : "Compact Index (+)"}
  </button>
  <button type="button" onclick={() => fire("index_verify", "verify")} disabled={busy.verify}>
    {busy.verify ? "Verifying…" : "Verify Index (+)"}
  </button>
  {#if toast}<p class="toast">{toast}</p>{/if}
</Section>

<Section title="Sourcerer Extras (+)">
  <Dropdown id="ix-policy" label="Index integrity policy"
    value={settingsStore.state.index_core.integrity_policy}
    options={[ { value: "strict", label: "Strict (refuse queries on corruption)" }, { value: "lenient", label: "Lenient (warn but query)" } ]}
    onChange={(v) => patch({ integrity_policy: v })} />
  <NumberInput id="ix-mem" label="Memory budget for indexer" min={64} max={32768}
    suffix="MB" value={settingsStore.state.index_core.memory_budget_mb}
    onChange={(n) => patch({ memory_budget_mb: n })} />
  <Dropdown id="ix-throttle" label="Background indexing throttle"
    value={settingsStore.state.index_core.background_throttle}
    options={[ { value: "off", label: "Off" }, { value: "on_battery", label: "When on battery" }, { value: "always", label: "Always" } ]}
    onChange={(v) => patch({ background_throttle: v })} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  button { padding: 6px 14px; background: var(--accent-cyan); color: var(--bg-canvas); border: 0; border-radius: 4px; cursor: pointer; font: inherit; margin-right: 8px; }
  button:disabled { opacity: 0.55; cursor: not-allowed; }
  .toast { margin-top: 8px; color: var(--accent-cyan); font-size: 12px; }
</style>
