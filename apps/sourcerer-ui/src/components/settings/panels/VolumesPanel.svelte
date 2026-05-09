<script lang="ts" module>
  function journalLabel(fs: string): string {
    const f = fs.toLowerCase();
    if (f === "ntfs" || f === "refs" || f === "exfat" || f === "fat32") return "Enable USN Journal (E)";
    if (f === "apfs" || f === "hfs+") return "Enable FSEvents stream";
    return "Enable inotify (or fanotify if elevated)";
  }
  function isMacFs(fs: string): boolean { const f = fs.toLowerCase(); return f === "apfs" || f === "hfs+"; }
  function isLinuxFs(fs: string): boolean { const f = fs.toLowerCase(); return ["ext4", "btrfs", "zfs", "xfs", "f2fs"].includes(f); }
</script>

<script lang="ts">
  import { volumesStore } from "../../../lib/stores/volumes.svelte";
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import type { VolumeInfo } from "../../../lib/ipc/types";

  let selectedId = $state<string | null>(null);
  $effect(() => {
    if (volumesStore.list.length > 0 && !selectedId) {
      selectedId = volumesStore.list[0].id;
    }
  });

  let selected = $derived(volumesStore.list.find((v) => v.id === selectedId) ?? null);

  /// The auto-include checkboxes live alongside per-volume overrides
  /// in the daemon's `VolumesConfig`. Their state lives in the settings
  /// store under `volumes_config`; on patch we round-trip through the
  /// daemon via `settings.apply` so the daemon's state honors the flip
  /// immediately.
  type VolumesConfigPatch = {
    auto_include_fixed?: boolean;
    auto_include_removable?: boolean;
    auto_remove_offline?: boolean;
  };
  let volsCfg = $derived(volsConfigFromStore());
  function volsConfigFromStore(): {
    auto_include_fixed: boolean;
    auto_include_removable: boolean;
    auto_remove_offline: boolean;
  } {
    const extras =
      (settingsStore.state as unknown as { extras?: Record<string, unknown> }).extras ?? {};
    const vc = (extras.volumes_config as VolumesConfigPatch | undefined) ?? {};
    return {
      auto_include_fixed: vc.auto_include_fixed ?? true,
      auto_include_removable: vc.auto_include_removable ?? false,
      auto_remove_offline: vc.auto_remove_offline ?? true
    };
  }

  async function patchTopLevel(p: VolumesConfigPatch) {
    const extras =
      (settingsStore.state as unknown as { extras?: Record<string, unknown> }).extras ?? {};
    const cur = (extras.volumes_config as VolumesConfigPatch | undefined) ?? {};
    const next = { ...cur, ...p };
    settingsStore.patch({
      extras: { ...extras, volumes_config: next }
    } as never);
    settingsDialog.markDirty("indexes.volumes");
    // Push the change to the daemon's settings.apply so VolumesConfig
    // updates immediately (rather than only on the dialog's Apply).
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("settings_apply_to_daemon", { state: settingsStore.state });
    } catch (e) {
      console.warn("[volumes] settings.apply failed:", e);
    }
  }

  let busyVol = $state<string | null>(null);

  async function setIndexed(v: VolumeInfo, on: boolean) {
    busyVol = v.id;
    try {
      await volumesStore.update({ id: v.id, indexed: on });
      settingsDialog.markDirty("indexes.volumes");
    } finally {
      busyVol = null;
    }
  }
  async function setJournal(v: VolumeInfo, on: boolean) {
    busyVol = v.id;
    try {
      await volumesStore.update({ id: v.id, journal_enabled: on });
      settingsDialog.markDirty("indexes.volumes");
    } finally {
      busyVol = null;
    }
  }
  async function setBuffer(v: VolumeInfo, n: number) {
    await volumesStore.update({ id: v.id, journal_buffer_kb: n });
    settingsDialog.markDirty("indexes.volumes");
  }
  async function setAlloc(v: VolumeInfo, n: number) {
    await volumesStore.update({ id: v.id, allocation_delta_kb: n });
    settingsDialog.markDirty("indexes.volumes");
  }
  async function setIncludeOnly(v: VolumeInfo, s: string) {
    await volumesStore.update({ id: v.id, include_only: s });
    settingsDialog.markDirty("indexes.volumes");
  }
  async function setLoadRecent(v: VolumeInfo, on: boolean) {
    await volumesStore.update({ id: v.id, load_recent_changes: on });
    settingsDialog.markDirty("indexes.volumes");
  }
  async function setMonitor(v: VolumeInfo, on: boolean) {
    await volumesStore.update({ id: v.id, monitor_changes: on });
    settingsDialog.markDirty("indexes.volumes");
  }

  function fmt(bytes: number): string {
    if (!bytes) return "—";
    const units = ["B", "KB", "MB", "GB", "TB"];
    let n = bytes;
    let i = 0;
    while (n >= 1024 && i < units.length - 1) { n /= 1024; i++; }
    return `${n.toFixed(1)} ${units[i]}`;
  }
</script>

<h1>Volumes</h1>
<p class="hint">Cross-platform analogue of voidtools-Everything's NTFS / ReFS panels. Auto-detects
NTFS / ReFS / exFAT / FAT32 (Win), APFS / HFS+ (macOS), ext4 / Btrfs / ZFS / XFS / F2FS (Linux).</p>

<Section title="Auto-include (E)">
  <Checkbox id="vols-auto-fixed" label="Automatically include new fixed volumes"
    checked={volsCfg.auto_include_fixed} onChange={(v) => patchTopLevel({ auto_include_fixed: v })} />
  <Checkbox id="vols-auto-removable" label="Automatically include new removable volumes"
    checked={volsCfg.auto_include_removable} onChange={(v) => patchTopLevel({ auto_include_removable: v })} />
  <Checkbox id="vols-auto-remove-offline" label="Automatically remove offline volumes"
    checked={volsCfg.auto_remove_offline} onChange={(v) => patchTopLevel({ auto_remove_offline: v })} />
</Section>

<div class="split">
  <div class="vlist">
    <h3>Detected volumes</h3>
    {#if volumesStore.loading}
      <p class="muted">Detecting…</p>
    {:else if volumesStore.list.length === 0}
      <p class="muted">No volumes detected.</p>
    {:else}
      <ul>
        {#each volumesStore.list as v (v.id)}
          <li>
            <button type="button" class:selected={v.id === selectedId} onclick={() => (selectedId = v.id)}>
              <span class="lbl">{v.label}</span>
              <span class="fs">{v.fs_kind}</span>
              <span class="size">{fmt(v.used_bytes)} / {fmt(v.total_bytes)}</span>
              <span class="status status-{v.status}" aria-label={v.status}>●</span>
            </button>
          </li>
        {/each}
      </ul>
    {/if}
    {#if selected}
      <button type="button" class="remove"
        onclick={() => selected && volumesStore.remove(selected.id)}>Remove</button>
    {/if}
  </div>

  <div class="vdetail">
    {#if selected}
      <Checkbox id={`vol-${selected.id}-indexed`} label="Include in index (E)"
        checked={selected.indexed} disabled={busyVol === selected.id}
        onChange={(v) => selected && setIndexed(selected, v)} />
      <TextInput id={`vol-${selected.id}-include`} label="Include only (glob/regex) (E)"
        value={selected.include_only ?? ""} onChange={(s) => selected && setIncludeOnly(selected, s)} />
      <Checkbox id={`vol-${selected.id}-journal`}
        label={journalLabel(selected.fs_kind)}
        checked={selected.journal_enabled} disabled={!selected.indexed}
        onChange={(v) => selected && setJournal(selected, v)} />
      <NumberInput id={`vol-${selected.id}-buf`} label="Journal buffer size (KB)"
        min={0} max={65536} value={selected.journal_buffer_kb}
        onChange={(n) => selected && setBuffer(selected, n)} />
      {#if selected.fs_kind.toLowerCase() === "ntfs"}
        <NumberInput id={`vol-${selected.id}-alloc`} label="Allocation delta (KB)"
          min={0} max={65536} value={selected.allocation_delta_kb ?? 0}
          onChange={(n) => selected && setAlloc(selected, n)} />
      {/if}
      <Checkbox id={`vol-${selected.id}-load-recent`} label="Load recent changes from journal on startup"
        checked={selected.load_recent_changes}
        onChange={(v) => selected && setLoadRecent(selected, v)} />
      <Checkbox id={`vol-${selected.id}-monitor`} label="Monitor changes (E)"
        checked={selected.monitor_changes}
        onChange={(v) => selected && setMonitor(selected, v)} />

      {#if selected.fs_kind.toLowerCase() === "ntfs"}
        <button type="button" onclick={() => selected && volumesStore.recreateJournal(selected.id)}>Recreate journal</button>
      {/if}
      {#if isMacFs(selected.fs_kind)}
        <button type="button" onclick={() => selected && volumesStore.resetStream(selected.id)}>Reset FSEvents stream</button>
      {/if}
      {#if isLinuxFs(selected.fs_kind)}
        <button type="button" onclick={() => volumesStore.upgradeFanotify()}>Upgrade to fanotify (polkit)</button>
      {/if}
    {:else}
      <p class="muted">Select a volume to configure it.</p>
    {/if}
  </div>
</div>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  .hint { margin: 0 0 16px; color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
  h3 { font-size: 12px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.06em; color: var(--text-secondary); margin: 0 0 6px; }
  .split { display: flex; gap: 18px; align-items: flex-start; }
  .vlist { width: 320px; flex-shrink: 0; }
  .vlist ul { list-style: none; margin: 0; padding: 0; max-height: 320px; overflow-y: auto; border: 1px solid var(--border); border-radius: 4px; }
  .vlist li { border-bottom: 1px solid var(--border); }
  .vlist li:last-child { border-bottom: 0; }
  .vlist button { width: 100%; text-align: left; background: none; border: 0; color: var(--text-primary); padding: 6px 8px; cursor: pointer; display: grid; grid-template-columns: 1fr auto auto auto; gap: 8px; align-items: center; font: inherit; }
  .vlist button:hover { background: var(--bg-surface); }
  .vlist button.selected { background: var(--accent-cyan); color: var(--bg-canvas); }
  .vlist .fs { color: var(--text-secondary); font-size: 11px; }
  .vlist .size { color: var(--text-secondary); font-size: 11px; }
  .status { font-size: 16px; line-height: 0; }
  .status-indexed, .status-indexing { color: var(--accent-cyan); }
  .status-paused { color: #888; }
  .status-offline, .status-error { color: var(--accent-orange); }
  .remove { margin-top: 8px; padding: 4px 10px; background: var(--bg-canvas); border: 1px solid var(--border); color: var(--text-primary); border-radius: 3px; cursor: pointer; }
  .vdetail { flex: 1; }
  .vdetail button { margin-top: 8px; margin-right: 6px; padding: 4px 10px; background: var(--bg-canvas); border: 1px solid var(--border); color: var(--text-primary); border-radius: 3px; cursor: pointer; font: inherit; }
  .muted { color: var(--text-secondary); }
</style>
