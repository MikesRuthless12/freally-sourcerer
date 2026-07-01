<script lang="ts">
  import { queryStore } from "../../lib/stores/query.svelte";
  import { resultsStore } from "../../lib/stores/results.svelte";
  import { typeFilterStore, type TypeFilterId } from "../../lib/stores/type_filter.svelte";
  import { t } from "../../lib/i18n/t";

  interface FilterChip {
    id: TypeFilterId;
    labelKey: string;
    icon: string;
  }

  // PRD §8.28 Search filter set + Freally additions.
  const CHIPS: FilterChip[] = [
    { id: "audio", labelKey: "quick-filter-audio", icon: "♪" },
    { id: "video", labelKey: "quick-filter-video", icon: "▶" },
    { id: "picture", labelKey: "quick-filter-image", icon: "▣" },
    { id: "document", labelKey: "quick-filter-document", icon: "📄" },
    { id: "executable", labelKey: "quick-filter-executable", icon: "▷" },
    { id: "compressed", labelKey: "quick-filter-archive", icon: "🗜" },
    { id: "folder", labelKey: "menu-search-filter-folder", icon: "📁" }
  ];

  async function toggle(chip: FilterChip) {
    typeFilterStore.toggle(chip.id);
    await resultsStore.run(queryStore.source);
  }
</script>

<div class="palette" role="toolbar" aria-label={t("menu-view-filters")}>
  {#each CHIPS as chip (chip.id)}
    <button
      type="button"
      class="chip"
      class:active={typeFilterStore.has(chip.id)}
      aria-pressed={typeFilterStore.has(chip.id)}
      onclick={() => toggle(chip)}
    >
      <span class="icon" aria-hidden="true">{chip.icon}</span>
      <span>{t(chip.labelKey)}</span>
    </button>
  {/each}
</div>

<style>
  .palette {
    display: flex;
    gap: 6px;
    padding: 6px 12px;
    background: var(--bg-surface);
    border-bottom: 1px solid var(--border);
    overflow-x: auto;
  }
  .chip {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 3px 10px;
    background: var(--bg-surface-2);
    border: 1px solid var(--border);
    border-radius: 999px;
    color: var(--text-secondary);
    font-size: 12px;
    cursor: pointer;
    white-space: nowrap;
  }
  .chip:hover {
    color: var(--text-primary);
  }
  .chip.active {
    background: color-mix(in srgb, var(--accent-cyan) 25%, transparent);
    border-color: var(--accent-cyan);
    color: var(--text-primary);
  }
  .icon {
    font-size: 11px;
  }
</style>
