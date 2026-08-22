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

  // The grip was focusable but had no keyboard route at all, so tabbing to it
  // reached a control that could not be operated. This is the ARIA window
  // splitter: arrows nudge, Home/End jump to the bounds.
  const KEY_STEP = 8;

  function onGripKey(ev: KeyboardEvent) {
    let next: number | null = null;
    if (ev.key === "ArrowLeft") next = width - KEY_STEP;
    else if (ev.key === "ArrowRight") next = width + KEY_STEP;
    else if (ev.key === "Home") next = MIN_COL_WIDTH;
    else if (ev.key === "End") next = MAX_COL_WIDTH;
    if (next === null) return;
    ev.preventDefault();
    void columnsStore.setWidth(id, Math.max(MIN_COL_WIDTH, Math.min(MAX_COL_WIDTH, next)));
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
  <!-- ARIA's window-splitter pattern: a `separator` that is focusable is a
       widget role and takes aria-valuenow/min/max, which is exactly what a
       column-resize grip is. Svelte's a11y rules classify `separator` as
       non-interactive unconditionally and so cannot express that, hence the
       two suppressions rather than a role that fits the linter better than it
       fits the control. -->
  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="grip"
    role="separator"
    aria-orientation="vertical"
    aria-label={t("column-resize", { name: label })}
    aria-valuenow={width}
    aria-valuemin={MIN_COL_WIDTH}
    aria-valuemax={MAX_COL_WIDTH}
    tabindex="0"
    onkeydown={onGripKey}
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
