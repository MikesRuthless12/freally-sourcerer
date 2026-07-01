<script lang="ts">
  import type { ColumnId } from "../../lib/ipc/types";
  import { columnsStore, COLUMN_LABEL_KEYS, MIN_COL_WIDTH, MAX_COL_WIDTH } from "../../lib/stores/columns.svelte";
  import { sortStore } from "../../lib/stores/sort.svelte";
  import { t } from "../../lib/i18n/t";

  interface Props {
    id: ColumnId;
    width: number;
  }
  let { id, width }: Props = $props();

  let dragging = $state(false);
  let startX = 0;
  let startW = 0;

  function onPointerDown(ev: PointerEvent) {
    dragging = true;
    startX = ev.clientX;
    startW = width;
    (ev.target as HTMLElement).setPointerCapture(ev.pointerId);
    ev.preventDefault();
  }

  function onPointerMove(ev: PointerEvent) {
    if (!dragging) return;
    const next = Math.max(MIN_COL_WIDTH, Math.min(MAX_COL_WIDTH, startW + (ev.clientX - startX)));
    void columnsStore.setWidth(id, next);
  }

  function onPointerUp(ev: PointerEvent) {
    if (!dragging) return;
    dragging = false;
    (ev.target as HTMLElement).releasePointerCapture(ev.pointerId);
  }

  function onSortClick() {
    sortStore.toggle(id);
  }

  const isSorted = $derived(sortStore.field === id);
  const arrow = $derived(isSorted ? (sortStore.order === "asc" ? "▲" : "▼") : "");
  const label = $derived(t(COLUMN_LABEL_KEYS[id]));
</script>

<div class="col-header" style="width: {width}px;" role="columnheader" aria-sort={isSorted ? (sortStore.order === "asc" ? "ascending" : "descending") : "none"}>
  <button
    type="button"
    class="title"
    onclick={onSortClick}
    aria-label={t("column-sort-by", { name: label })}
  >
    {label}
    {#if arrow}<span class="arrow" aria-hidden="true">{arrow}</span>{/if}
  </button>
  <div
    class="grip"
    role="separator"
    aria-orientation="vertical"
    aria-label={t("column-resize", { name: label })}
    tabindex="0"
    onpointerdown={onPointerDown}
    onpointermove={onPointerMove}
    onpointerup={onPointerUp}
    onpointercancel={onPointerUp}
  ></div>
</div>

<style>
  .col-header {
    display: inline-flex;
    align-items: center;
    height: 28px;
    border-right: 1px solid var(--border);
    background: var(--bg-surface-2);
    color: var(--text-secondary);
    font-size: 12px;
    user-select: none;
    box-sizing: border-box;
    flex-shrink: 0;
  }
  .title {
    flex: 1;
    background: transparent;
    border: 0;
    color: inherit;
    font-size: 12px;
    text-align: left;
    padding: 0 8px;
    cursor: pointer;
    height: 100%;
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .title:hover {
    background: var(--bg-surface);
  }
  .arrow {
    color: var(--accent-cyan);
  }
  .grip {
    width: 4px;
    height: 100%;
    cursor: col-resize;
    background: transparent;
  }
  .grip:hover,
  .grip:focus-visible {
    background: var(--accent-cyan);
  }
</style>
