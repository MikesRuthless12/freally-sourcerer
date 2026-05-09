<script lang="ts">
  import { foldersStore } from "../../../lib/stores/folders.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import type { WatchedFolder, RescanSchedule } from "../../../lib/ipc/types";

  let selectedId = $state<string | null>(null);
  let selected = $derived(foldersStore.list.find((f) => f.id === selectedId) ?? null);

  async function pickAndAdd() {
    const picked = await openDialog({ directory: true, multiple: false });
    if (typeof picked === "string") {
      const id = `folder-${Date.now()}`;
      const folder: WatchedFolder = {
        id,
        path: picked,
        monitor_changes: true,
        buffer_kb: 0,
        rescan_on_full_buffer: true,
        rescan_schedule: { kind: "at_time", hour: 3, minute: 0 }
      };
      await foldersStore.add(folder);
      selectedId = id;
      settingsDialog.markDirty("indexes.folders");
    }
  }

  async function removeSelected() {
    if (!selected) return;
    await foldersStore.remove(selected.id);
    selectedId = null;
    settingsDialog.markDirty("indexes.folders");
  }

  async function update(p: Partial<WatchedFolder>) {
    if (!selected) return;
    await foldersStore.update({ ...selected, ...p });
    settingsDialog.markDirty("indexes.folders");
  }

  function setSchedule(kind: "at_time" | "every_hours" | "never") {
    if (!selected) return;
    let s: RescanSchedule;
    if (kind === "at_time") s = { kind: "at_time", hour: 3, minute: 0 };
    else if (kind === "every_hours") s = { kind: "every_hours", hours: 24 };
    else s = { kind: "never" };
    update({ rescan_schedule: s });
  }
</script>

<h1>Folders</h1>
<p class="hint">Additional watched folders beyond the default volumes (E).</p>

<div class="split">
  <div class="flist">
    <h3>Watched folders</h3>
    {#if foldersStore.list.length === 0}
      <p class="muted">No folders added yet.</p>
    {:else}
      <ul>
        {#each foldersStore.list as f (f.id)}
          <li>
            <button type="button" class:selected={f.id === selectedId} onclick={() => (selectedId = f.id)}>
              {f.path}
            </button>
          </li>
        {/each}
      </ul>
    {/if}
    <div class="actions">
      <button type="button" onclick={pickAndAdd}>Add…</button>
      <button type="button" onclick={removeSelected} disabled={!selected}>Remove</button>
      <button type="button" disabled={!selected} onclick={() => selected && foldersStore.rescan(selected.id)}>Rescan Now</button>
      <button type="button" onclick={() => foldersStore.rescanAll()}>Rescan All Now</button>
    </div>
  </div>

  <div class="fdetail">
    {#if selected}
      <Section title="Settings for {selected.path}">
        <Checkbox id={`fld-${selected.id}-monitor`} label="Attempt to monitor changes (E)"
          checked={selected.monitor_changes} onChange={(v) => update({ monitor_changes: v })} />
        <NumberInput id={`fld-${selected.id}-buf`} label="Buffer size"
          min={0} max={65536} suffix="KB"
          value={selected.buffer_kb} onChange={(n) => update({ buffer_kb: n })} />
        <Checkbox id={`fld-${selected.id}-on-full`} label="Rescan on full buffer (E)"
          checked={selected.rescan_on_full_buffer}
          onChange={(v) => update({ rescan_on_full_buffer: v })} />
        <Dropdown id={`fld-${selected.id}-sched`} label="Rescan schedule"
          value={selected.rescan_schedule.kind}
          options={[ { value: "at_time", label: "Every day at HH:MM" }, { value: "every_hours", label: "Every N hours" }, { value: "never", label: "Never" } ]}
          onChange={(v) => setSchedule(v)} />
        {#if selected.rescan_schedule.kind === "at_time"}
          <NumberInput id={`fld-${selected.id}-hour`} label="Hour" min={0} max={23}
            value={selected.rescan_schedule.hour} onChange={(n) => update({ rescan_schedule: { kind: "at_time", hour: n, minute: selected.rescan_schedule.kind === "at_time" ? selected.rescan_schedule.minute : 0 } })} />
          <NumberInput id={`fld-${selected.id}-min`} label="Minute" min={0} max={59}
            value={selected.rescan_schedule.minute} onChange={(n) => update({ rescan_schedule: { kind: "at_time", hour: selected.rescan_schedule.kind === "at_time" ? selected.rescan_schedule.hour : 3, minute: n } })} />
        {:else if selected.rescan_schedule.kind === "every_hours"}
          <NumberInput id={`fld-${selected.id}-hours`} label="Hours" min={1} max={720}
            value={selected.rescan_schedule.hours} onChange={(n) => update({ rescan_schedule: { kind: "every_hours", hours: n } })} />
        {/if}
        <TextInput id={`fld-${selected.id}-id`} label="Folder ID (read-only)"
          value={selected.id} onChange={() => {}} />
      </Section>
    {:else}
      <p class="muted">Select a folder to configure it.</p>
    {/if}
  </div>
</div>

<Section title="Sourcerer Extras (+)">
  <p class="muted">Rescan on resume from sleep is enabled by default in this build; the toggle joins the
  folder-level controls in Phase 13's polish pass.</p>
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  .hint { margin: 0 0 16px; color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
  h3 { font-size: 12px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.06em; color: var(--text-secondary); margin: 0 0 6px; }
  .split { display: flex; gap: 18px; align-items: flex-start; }
  .flist { width: 360px; flex-shrink: 0; }
  .flist ul { list-style: none; margin: 0; padding: 0; max-height: 280px; overflow-y: auto; border: 1px solid var(--border); border-radius: 4px; }
  .flist li button { width: 100%; text-align: left; background: none; border: 0; color: var(--text-primary); padding: 6px 8px; cursor: pointer; font: inherit; }
  .flist li button:hover { background: var(--bg-surface); }
  .flist li button.selected { background: var(--accent-cyan); color: var(--bg-canvas); }
  .actions { margin-top: 8px; display: flex; gap: 6px; flex-wrap: wrap; }
  .actions button { padding: 4px 10px; background: var(--bg-canvas); border: 1px solid var(--border); color: var(--text-primary); border-radius: 3px; cursor: pointer; font: inherit; }
  .actions button:disabled { opacity: 0.55; cursor: not-allowed; }
  .fdetail { flex: 1; }
  .muted { color: var(--text-secondary); }
</style>
