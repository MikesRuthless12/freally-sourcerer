<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { t } from "../../../lib/i18n/t";
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

<h1>{t("settings-group-indexes")}</h1>

<Section title={t("section-storage")}>
  <TextInput id="ix-db-loc" label={t("settings-ix-database-location")} value={settingsStore.state.index_core.database_location}
    onChange={(v) => patch({ database_location: v })} />
  <Checkbox id="ix-multiuser" label={t("settings-ix-multiuser")}
    checked={settingsStore.state.index_core.multi_user_database_filename}
    onChange={(v) => patch({ multi_user_database_filename: v })} />
  <Checkbox id="ix-compress" label={t("settings-ix-compress")}
    checked={settingsStore.state.index_core.compress_database}
    onChange={(v) => patch({ compress_database: v })} />
</Section>

<Section title={t("section-index-fields")}>
  <Checkbox id="ix-recent" label={t("settings-ix-recent-changes")}
    checked={settingsStore.state.index_core.index_recent_changes}
    onChange={(v) => patch({ index_recent_changes: v })} />
  <Checkbox id="ix-fsize" label={t("settings-ix-file-size")}
    checked={settingsStore.state.index_core.index_file_size}
    onChange={(v) => patch({ index_file_size: v })} />
  <Checkbox id="ix-fsize-fast" label={t("settings-ix-fast-size-sort")}
    checked={settingsStore.state.index_core.fast_size_sort}
    onChange={(v) => patch({ fast_size_sort: v })} />
  <Checkbox id="ix-fold-size" label={t("settings-ix-folder-size")}
    checked={settingsStore.state.index_core.index_folder_size}
    onChange={(v) => patch({ index_folder_size: v })} />
  <Checkbox id="ix-fold-size-fast" label={t("settings-ix-fast-folder-size-sort")}
    checked={settingsStore.state.index_core.fast_folder_size_sort}
    onChange={(v) => patch({ fast_folder_size_sort: v })} />
  <Checkbox id="ix-dc" label={t("settings-ix-date-created")}
    checked={settingsStore.state.index_core.index_date_created}
    onChange={(v) => patch({ index_date_created: v })} />
  <Checkbox id="ix-dc-fast" label={t("settings-ix-fast-date-created")}
    checked={settingsStore.state.index_core.fast_date_created_sort}
    onChange={(v) => patch({ fast_date_created_sort: v })} />
  <Checkbox id="ix-dm" label={t("settings-ix-date-modified")}
    checked={settingsStore.state.index_core.index_date_modified}
    onChange={(v) => patch({ index_date_modified: v })} />
  <Checkbox id="ix-dm-fast" label={t("settings-ix-fast-date-modified")}
    checked={settingsStore.state.index_core.fast_date_modified_sort}
    onChange={(v) => patch({ fast_date_modified_sort: v })} />
  <Checkbox id="ix-da" label={t("settings-ix-date-accessed")}
    checked={settingsStore.state.index_core.index_date_accessed}
    onChange={(v) => patch({ index_date_accessed: v })} />
  <Checkbox id="ix-da-fast" label={t("settings-ix-fast-date-accessed")}
    checked={settingsStore.state.index_core.fast_date_accessed_sort}
    onChange={(v) => patch({ fast_date_accessed_sort: v })} />
  <Checkbox id="ix-attr" label={t("settings-ix-attributes")}
    checked={settingsStore.state.index_core.index_attributes}
    onChange={(v) => patch({ index_attributes: v })} />
  <Checkbox id="ix-attr-fast" label={t("settings-ix-fast-attributes")}
    checked={settingsStore.state.index_core.fast_attributes_sort}
    onChange={(v) => patch({ fast_attributes_sort: v })} />
  <Checkbox id="ix-path-fast" label={t("settings-ix-fast-path-sort")}
    checked={settingsStore.state.index_core.fast_path_sort}
    onChange={(v) => patch({ fast_path_sort: v })} />
  <Checkbox id="ix-ext-fast" label={t("settings-ix-fast-extension-sort")}
    checked={settingsStore.state.index_core.fast_extension_sort}
    onChange={(v) => patch({ fast_extension_sort: v })} />
</Section>

<Section title={t("section-maintenance")}>
  <button type="button" onclick={() => fire("index_rebuild", "rebuild")} disabled={busy.rebuild}>
    {busy.rebuild ? "Rebuilding…" : t("settings-ix-force-rebuild")}
  </button>
  <button type="button" onclick={() => fire("index_compact", "compact")} disabled={busy.compact}>
    {busy.compact ? "Compacting…" : t("settings-ix-compact")}
  </button>
  <button type="button" onclick={() => fire("index_verify", "verify")} disabled={busy.verify}>
    {busy.verify ? "Verifying…" : t("settings-ix-verify")}
  </button>
  {#if toast}<p class="toast">{toast}</p>{/if}
</Section>

<Section title={t("folders-section-extras")}>
  <Dropdown id="ix-policy" label={t("settings-ix-integrity-policy")}
    value={settingsStore.state.index_core.integrity_policy}
    options={[ { value: "strict", label: t("opt-strict-refuse") }, { value: "lenient", label: t("opt-lenient-warn") } ]}
    onChange={(v) => patch({ integrity_policy: v })} />
  <NumberInput id="ix-mem" label={t("settings-ix-memory-budget")} min={64} max={32768}
    suffix={t("unit-mb")} value={settingsStore.state.index_core.memory_budget_mb}
    onChange={(n) => patch({ memory_budget_mb: n })} />
  <Dropdown id="ix-throttle" label={t("settings-ix-throttle")}
    value={settingsStore.state.index_core.background_throttle}
    options={[ { value: "off", label: t("opt-off") }, { value: "on_battery", label: t("opt-on-battery") }, { value: "always", label: t("opt-always") } ]}
    onChange={(v) => patch({ background_throttle: v })} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  button { padding: 6px 14px; background: var(--accent-cyan); color: var(--bg-canvas); border: 0; border-radius: 4px; cursor: pointer; font: inherit; margin-right: 8px; }
  button:disabled { opacity: 0.55; cursor: not-allowed; }
  .toast { margin-top: 8px; color: var(--accent-cyan); font-size: 12px; }
</style>
