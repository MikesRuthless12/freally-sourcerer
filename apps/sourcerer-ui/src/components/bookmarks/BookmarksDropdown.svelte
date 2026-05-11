<script lang="ts">
  import { bookmarksStore } from "../../lib/stores/bookmarks.svelte";
  import { queryStore } from "../../lib/stores/query.svelte";
  import { resultsStore } from "../../lib/stores/results.svelte";
  import { typeFilterStore } from "../../lib/stores/type_filter.svelte";
  import type { Bookmark } from "../../lib/ipc/types";

  let open = $state(false);

  async function load(bm: Bookmark) {
    open = false;
    // Restore the chip selection first so resultsStore.run() composes
    // the same lens-prefix the bookmark was saved with.
    typeFilterStore.setFromIds(bm.filters ?? []);
    await queryStore.setSource(bm.query);
    await resultsStore.run(bm.query);
  }
</script>

<svelte:window onclick={() => (open = false)} />

<div class="bm-wrap">
  <button
    type="button"
    class="bm-trigger"
    aria-haspopup="menu"
    aria-expanded={open}
    onclick={(e) => {
      e.stopPropagation();
      open = !open;
    }}
  >
    ★ Bookmarks
  </button>
  {#if open}
    <div class="bm-list" role="menu">
      {#if bookmarksStore.items.length === 0}
        <div class="empty">No bookmarks yet. Press Ctrl+D to save the current query.</div>
      {:else}
        {#each bookmarksStore.items as bm (bm.id)}
          <button
            type="button"
            class="bm-item"
            role="menuitem"
            onclick={(e) => {
              e.stopPropagation();
              void load(bm);
            }}
          >
            <span class="name">{bm.name}</span>
            <span class="query">{bm.query}</span>
          </button>
        {/each}
      {/if}
    </div>
  {/if}
</div>

<style>
  .bm-wrap {
    position: relative;
  }
  .bm-trigger {
    height: 100%;
    padding: 0 12px;
    background: transparent;
    border: 0;
    color: var(--text-primary);
    font-size: 13px;
    cursor: pointer;
  }
  .bm-trigger:hover {
    background: var(--bg-surface-2);
  }
  .bm-list {
    position: absolute;
    top: 100%;
    right: 0;
    min-width: 280px;
    max-height: 360px;
    overflow-y: auto;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    box-shadow: 0 6px 24px rgba(0, 0, 0, 0.18);
    padding: 4px 0;
    z-index: 50;
  }
  .empty {
    padding: 12px;
    color: var(--text-secondary);
    font-size: 12px;
  }
  .bm-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
    width: 100%;
    padding: 6px 12px;
    background: transparent;
    border: 0;
    text-align: left;
    cursor: pointer;
  }
  .bm-item:hover {
    background: var(--bg-surface-2);
  }
  .name {
    color: var(--text-primary);
    font-size: 13px;
  }
  .query {
    color: var(--text-secondary);
    font-family: var(--font-mono);
    font-size: 11px;
  }
</style>
