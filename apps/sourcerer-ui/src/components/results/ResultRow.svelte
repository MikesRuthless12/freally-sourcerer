<script lang="ts">
  import type { ColumnId, QueryHit } from "../../lib/ipc/types";
  import { selectionStore } from "../../lib/stores/selection.svelte";
  import { columnsStore } from "../../lib/stores/columns.svelte";
  import { settingsStore } from "../../lib/stores/settings.svelte";
  import { iconStore } from "../../lib/stores/icon_store.svelte";
  import { registry } from "../../lib/commands/registry.svelte";
  import * as files from "../../lib/ipc/files";
  import { formatBytes, formatDateMs } from "../../lib/util/format";

  interface Props {
    hit: QueryHit;
  }
  let { hit }: Props = $props();

  const selected = $derived(selectionStore.has(hit.file_id));
  const cols = $derived(columnsStore.visible);
  const heightVar = $derived(
    settingsStore.state.row_density === "comfortable"
      ? "var(--row-height-comfortable)"
      : "var(--row-height-compact)"
  );
  const isDir = $derived(((hit.attrs ?? 0) & 0x10) !== 0);
  // Touch iconStore.tick so this $derived re-runs when icons resolve.
  const iconUrl = $derived(
    (iconStore.tick, iconStore.get(hit.ext ?? "", isDir))
  );
  const iconFallback = $derived(isDir ? "📁" : "📄");

  function cellText(id: ColumnId): string {
    switch (id) {
      case "name": return hit.name;
      case "path": return hit.path;
      case "size": {
        // Honor View → Display format → Size format. Stored under the
        // catch-all `extras`, so a runtime cast keeps the typed surface
        // narrow without growing SettingsState.
        const fmt = (settingsStore.state as unknown as { size_format?: string }).size_format;
        return formatBytes(hit.size, (fmt ?? "auto_binary") as Parameters<typeof formatBytes>[1]);
      }
      case "modified": return formatDateMs(hit.modified_ms);
      case "type": return hit.type;
      case "ext": return hit.ext;
    }
  }
  // View → Rendering → Show tooltips: when enabled, hovering a row
  // surfaces the full path (the same string the Path column truncates).
  const showTooltips = $derived(
    (settingsStore.state as unknown as { show_tooltips?: boolean }).show_tooltips !== false,
  );

  function onClick(ev: MouseEvent) {
    console.log("[row] click", { name: hit.name, ext: hit.ext, size: hit.size, path: hit.path });
    if (ev.metaKey || ev.ctrlKey) {
      selectionStore.toggle(hit.file_id);
    } else {
      selectionStore.clear();
      selectionStore.toggle(hit.file_id);
    }
  }

  async function onDoubleClick() {
    await files.open(hit.path);
  }

  async function onKey(ev: KeyboardEvent) {
    if (ev.key === "Enter") {
      if (ev.metaKey || ev.ctrlKey) await files.reveal(hit.path);
      else await files.open(hit.path);
    } else if (ev.key === "c" && (ev.metaKey || ev.ctrlKey)) {
      ev.preventDefault();
      // Route through the registry so the per-control wiring test covers
      // this path the same way as the Edit menu.
      await registry.run("edit.advanced.copy_filename");
    }
  }
</script>

<button
  type="button"
  class="row {hit.lens}"
  class:selected
  aria-selected={selected}
  title={showTooltips ? hit.path : undefined}
  style="height: {heightVar};"
  onclick={onClick}
  ondblclick={onDoubleClick}
  onkeydown={onKey}
>
  {#each cols as col (col.id)}
    <span class="cell {col.id}" style="width: {col.width_px}px;">
      {#if col.id === "name"}
        {#if iconUrl}
          <img class="row-icon" src={iconUrl} alt="" aria-hidden="true" />
        {:else}
          <span class="row-icon fallback" aria-hidden="true">{iconFallback}</span>
        {/if}
      {/if}
      {cellText(col.id)}
    </span>
  {/each}
</button>

<style>
  .row {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 0;
    /* Per-state Fonts & Colors overrides fall back to the theme tokens
     * so users only deviate where they set a custom value. */
    background: var(--row-normal-bg, transparent);
    border: 0;
    border-bottom: 1px solid var(--border);
    color: var(--row-normal-fg, var(--text-primary));
    font-weight: var(--row-normal-weight, normal);
    font-style: var(--row-normal-style, normal);
    text-align: left;
    cursor: default;
    font-size: 13px;
    border-radius: 0;
  }
  .row:hover {
    background: var(--row-highlighted-bg, var(--bg-surface-2));
    color: var(--row-highlighted-fg, var(--text-primary));
    font-weight: var(--row-highlighted-weight, normal);
    font-style: var(--row-highlighted-style, normal);
  }
  .row.selected {
    background: var(
      --row-selected-bg,
      color-mix(in srgb, var(--accent-cyan) 18%, transparent)
    );
    color: var(--row-selected-fg, var(--text-primary));
    font-weight: var(--row-selected-weight, normal);
    font-style: var(--row-selected-style, normal);
  }
  .row.selected:hover {
    background: var(
      --row-selected-highlighted-bg,
      color-mix(in srgb, var(--accent-cyan) 26%, transparent)
    );
    color: var(--row-selected-highlighted-fg, var(--text-primary));
    font-weight: var(--row-selected-highlighted-weight, normal);
    font-style: var(--row-selected-highlighted-style, normal);
  }
  .cell {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    padding: 0 8px;
    flex-shrink: 0;
    box-sizing: border-box;
  }
  .row-icon {
    display: inline-block;
    width: 16px;
    height: 16px;
    margin-right: 6px;
    vertical-align: middle;
    object-fit: contain;
  }
  .row-icon.fallback {
    text-align: center;
    line-height: 16px;
    font-size: 12px;
  }
  .cell.path {
    color: var(--text-secondary);
    font-family: var(--font-mono);
    font-size: 12px;
  }
  .cell.size,
  .cell.modified {
    color: var(--text-secondary);
    font-variant-numeric: tabular-nums;
  }
</style>
