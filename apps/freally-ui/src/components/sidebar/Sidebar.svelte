<script lang="ts">
  // SRC-M22 — one-click scoping.
  //
  // Bookmarks, quick filters, volumes and recent searches all existed;
  // each was behind a different dropdown or menu, so scoping a search
  // meant knowing which of four places to look. This puts them in one
  // column of clickable nodes.

  import { bookmarksStore } from "../../lib/stores/bookmarks.svelte";
  import { typeFilterStore, ALL_TYPE_FILTERS } from "../../lib/stores/type_filter.svelte";
  import type { TypeFilterId } from "../../lib/stores/type_filter.svelte";
  import { volumesStore } from "../../lib/stores/volumes.svelte";
  import { recentSearchesStore } from "../../lib/stores/recent_searches.svelte";
  import { settingsStore } from "../../lib/stores/settings.svelte";
  import { queryStore } from "../../lib/stores/query.svelte";
  import { resultsStore } from "../../lib/stores/results.svelte";
  import { t } from "../../lib/i18n/t";
  import type { Bookmark } from "../../lib/ipc/types";

  // Reuses the chip row's existing labels rather than minting parallel
  // keys for the same seven words. `folder` has no `quick-filter-` key
  // because the chip row does not offer it, so it borrows the menu's.
  const TYPE_LABEL: Record<TypeFilterId, string> = {
    audio: "quick-filter-audio",
    video: "quick-filter-video",
    picture: "quick-filter-image",
    document: "quick-filter-document",
    executable: "quick-filter-executable",
    compressed: "quick-filter-archive",
    folder: "menu-search-filter-folder"
  };

  /**
   * Bookmarks in the user's drag order.
   *
   * The stored order is a list of ids; anything not in it (a bookmark
   * added since the last drag) is appended in store order rather than
   * dropped, and ids for deleted bookmarks are ignored rather than
   * leaving a hole.
   */
  const orderedBookmarks = $derived.by((): Bookmark[] => {
    const order = settingsStore.state.sidebar_bookmark_order ?? [];
    const byId = new Map(bookmarksStore.items.map((b) => [b.id, b]));
    const out: Bookmark[] = [];
    for (const id of order) {
      const b = byId.get(id);
      if (b) {
        out.push(b);
        byId.delete(id);
      }
    }
    return [...out, ...byId.values()];
  });

  let dragId = $state<string | null>(null);

  // Shared with the bookmarks dropdown: restoring the chip selection
  // before the query runs is load-bearing, and duplicating the flow here
  // silently dropped the saved filters.
  const applyBookmark = (b: Bookmark) => bookmarksStore.apply(b);

  async function applyQuery(q: string) {
    await queryStore.setSource(q);
    await resultsStore.run(q);
  }

  async function applyVolume(label: string) {
    // Quoted because a volume label routinely contains spaces
    // ("Orange WD 4TB") and an unquoted one would parse as two terms.
    const q = `volume:"${label}"`;
    await applyQuery(q);
  }

  function onDragStart(id: string) {
    dragId = id;
  }

  async function onDrop(targetId: string) {
    const from = dragId;
    dragId = null;
    if (!from || from === targetId) return;
    const ids = orderedBookmarks.map((b) => b.id);
    const fromAt = ids.indexOf(from);
    const toAt = ids.indexOf(targetId);
    if (fromAt < 0 || toAt < 0) return;
    ids.splice(toAt, 0, ids.splice(fromAt, 1)[0]);
    await settingsStore.patch({ sidebar_bookmark_order: ids });
  }
</script>

<nav class="sidebar" aria-label={t("sidebar-title")} data-testid="sidebar">
  <section>
    <h2>{t("sidebar-bookmarks")}</h2>
    {#if orderedBookmarks.length === 0}
      <p class="empty">{t("sidebar-no-bookmarks")}</p>
    {:else}
      <ul>
        {#each orderedBookmarks as b (b.id)}
          <li
            draggable="true"
            class:dragging={dragId === b.id}
            ondragstart={() => onDragStart(b.id)}
            ondragover={(e) => e.preventDefault()}
            ondrop={(e) => {
              e.preventDefault();
              void onDrop(b.id);
            }}
          >
            <button type="button" onclick={() => applyBookmark(b)} title={b.query}>{b.name}</button>
          </li>
        {/each}
      </ul>
    {/if}
  </section>

  <section>
    <h2>{t("sidebar-filters")}</h2>
    <ul>
      {#each ALL_TYPE_FILTERS as id (id)}
        <li>
          <button
            type="button"
            class:active={typeFilterStore.has(id)}
            aria-pressed={typeFilterStore.has(id)}
            onclick={() => typeFilterStore.toggle(id)}>{t(TYPE_LABEL[id])}</button
          >
        </li>
      {/each}
    </ul>
  </section>

  <section>
    <h2>{t("sidebar-volumes")}</h2>
    {#if volumesStore.list.length === 0}
      <p class="empty">{t("sidebar-no-volumes")}</p>
    {:else}
      <ul>
        {#each volumesStore.list as v (v.id)}
          <li>
            <button type="button" onclick={() => applyVolume(v.label || v.id)} title={v.id}>
              <span class="pip" class:offline={v.status === "offline"}></span>
              {v.label || v.id}
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </section>

  <section>
    <h2>{t("sidebar-recent")}</h2>
    {#if recentSearchesStore.items.length === 0}
      <p class="empty">{t("sidebar-no-recent")}</p>
    {:else}
      <ul>
        {#each recentSearchesStore.items as q (q)}
          <li><button type="button" onclick={() => applyQuery(q)} title={q}>{q}</button></li>
        {/each}
      </ul>
      <button type="button" class="clear" onclick={() => recentSearchesStore.clear()}>
        {t("sidebar-clear-recent")}
      </button>
    {/if}
  </section>
</nav>

<style>
  .sidebar {
    width: 220px;
    flex-shrink: 0;
    overflow-y: auto;
    padding: 8px 0;
    background: var(--bg-surface);
    border-right: 1px solid var(--border);
  }
  section {
    padding: 4px 0 10px;
  }
  h2 {
    margin: 0;
    padding: 4px 12px;
    color: var(--text-secondary);
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }
  ul {
    list-style: none;
    margin: 2px 0 0;
    padding: 0;
  }
  li {
    display: block;
  }
  li.dragging {
    opacity: 0.4;
  }
  li button,
  .clear {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 4px 12px;
    border: 0;
    background: none;
    color: var(--text-primary);
    font: 12px/1.5 inherit;
    text-align: left;
    cursor: pointer;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  li button:hover,
  .clear:hover {
    background: color-mix(in srgb, var(--text-primary) 8%, transparent);
  }
  li button.active {
    color: var(--accent-cyan);
    font-weight: 600;
  }
  .pip {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--success);
    flex-shrink: 0;
  }
  .pip.offline {
    background: var(--text-secondary);
  }
  .empty {
    margin: 0;
    padding: 2px 12px;
    color: var(--text-secondary);
    font-size: 11px;
  }
  .clear {
    color: var(--text-secondary);
    font-size: 11px;
  }
</style>
