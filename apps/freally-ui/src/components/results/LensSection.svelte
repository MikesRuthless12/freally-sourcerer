<script lang="ts">
  import type { HitGroup, LensId, QueryHit } from "../../lib/ipc/types";
  import LensTimingBadge from "./LensTimingBadge.svelte";
  import ResultRow from "./ResultRow.svelte";
  import ColumnHeaderRow from "./ColumnHeaderRow.svelte";
  import { settingsStore } from "../../lib/stores/settings.svelte";
  import { sortStore } from "../../lib/stores/sort.svelte";
  import { formatBytes } from "../../lib/util/format";
  import { t } from "../../lib/i18n/t";

  interface Props {
    lens: LensId;
    title: string;
    hits: QueryHit[];
    timingMs: number;
    /** SRC-M07 duplicate clusters. Non-empty switches this section to
     *  the grouped renderer. */
    groups?: HitGroup[];
  }
  let { lens, title, hits, timingMs, groups = [] }: Props = $props();

  let collapsed = $state(false);

  const visible = $derived(settingsStore.state.lens_visibility[lens] !== false);
  const grouped = $derived(groups.length > 0);
  // Column sorting would break the cluster contiguity the daemon's
  // `start`/`len` offsets depend on, so grouped sections keep the
  // daemon's order — the sort already ran *within* each cluster.
  const sortedHits = $derived(grouped ? hits : sortStore.applied(hits));
  const clusters = $derived(
    groups.map((g) => {
      const rows = hits.slice(g.start, g.start + g.len);
      return {
        ...g,
        rows,
        // The header is built here, not in the daemon: it has to go
        // through Fluent and the user's chosen size format like every
        // other string and number on screen.
        label:
          g.name ??
          (g.size === undefined ? "" : formatBytes(g.size)),
        // Recomputed from the rows actually shown, so a refinement that
        // drops members can't leave a stale total in the header.
        totalBytes: rows.reduce((n, h) => n + h.size, 0)
      };
    })
  );
</script>

{#if visible}
  <section class="lens" data-lens={lens}>
    <header>
      <button
        type="button"
        class="caret"
        aria-expanded={!collapsed}
        aria-label={collapsed ? t("lens-expand") : t("lens-collapse")}
        onclick={() => (collapsed = !collapsed)}
      >
        <span class="chevron" class:collapsed>▾</span>
      </button>
      <span class="dot {lens}" aria-hidden="true"></span>
      <h2>{title}</h2>
      <span class="count">{hits.length}</span>
      <span class="grow"></span>
      {#if settingsStore.state.show_timing_badges}
        <LensTimingBadge {lens} ms={timingMs} />
      {/if}
    </header>
    {#if !collapsed}
      <ColumnHeaderRow />
      <div class="rows" role="listbox" aria-label={title} aria-multiselectable="true">
        {#if grouped}
          {#each clusters as cluster (cluster.label + cluster.start)}
            <div class="group-header" role="presentation">
              <span class="group-label">{cluster.label}</span>
              <span class="group-meta">
                {t("dupe-group-summary", {
                  count: cluster.len,
                  bytes: formatBytes(cluster.totalBytes)
                })}
              </span>
            </div>
            {#each cluster.rows as hit (hit.file_id)}
              <ResultRow {hit} />
            {/each}
          {/each}
        {:else}
          {#each sortedHits as hit (hit.file_id)}
            <ResultRow {hit} />
          {/each}
        {/if}
        {#if hits.length === 0}
          <div class="empty">{t("lens-no-matches")}</div>
        {/if}
      </div>
    {/if}
  </section>
{/if}

<style>
  .lens {
    margin-bottom: var(--lens-section-gap);
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
  }
  header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: var(--bg-surface-2);
    border-bottom: 1px solid var(--border);
  }
  h2 {
    margin: 0;
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
  }
  .count {
    font-size: 12px;
    color: var(--text-secondary);
    font-variant-numeric: tabular-nums;
  }
  .grow {
    flex: 1;
  }
  .caret {
    background: transparent;
    border: 0;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 0 2px;
  }
  .chevron {
    display: inline-block;
    transition: transform 100ms ease;
  }
  .chevron.collapsed {
    transform: rotate(-90deg);
  }
  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }
  .dot.filename { background: var(--lens-filename); }
  .dot.content { background: var(--lens-content); }
  .dot.audio { background: var(--lens-audio); }
  .dot.similarity { background: var(--lens-similarity); }
  .rows {
    max-height: 360px;
    overflow-y: auto;
    overflow-x: auto;
  }
  .empty {
    padding: 12px;
    color: var(--text-secondary);
    font-size: 12px;
  }
  .group-header {
    display: flex;
    align-items: baseline;
    gap: 8px;
    padding: 6px 12px;
    background: var(--bg-surface-2);
    border-top: 1px solid var(--border);
    border-bottom: 1px solid var(--border);
    position: sticky;
    top: 0;
    z-index: 1;
  }
  .group-label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-primary);
  }
  .group-meta {
    font-size: 11px;
    color: var(--text-secondary);
    font-variant-numeric: tabular-nums;
  }
</style>
