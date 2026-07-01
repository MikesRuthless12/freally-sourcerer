<script lang="ts">
  import { bookmarksStore } from "../../lib/stores/bookmarks.svelte";
  import { t } from "../../lib/i18n/t";

  interface Props {
    open: boolean;
    onClose: () => void;
  }
  let { open, onClose }: Props = $props();

  let editingId = $state<string | null>(null);
  let editValue = $state("");

  function startRename(id: string, current: string) {
    editingId = id;
    editValue = current;
  }

  async function commitRename() {
    if (editingId && editValue.trim()) {
      await bookmarksStore.rename(editingId, editValue.trim());
    }
    editingId = null;
    editValue = "";
  }

  async function remove(id: string) {
    await bookmarksStore.remove(id);
  }
</script>

{#if open}
  <div
    class="backdrop"
    role="presentation"
    onclick={onClose}
    onkeydown={(e) => e.key === "Escape" && onClose()}
  >
    <div
      class="modal"
      role="dialog"
      aria-modal="true"
      aria-label={t("bookmarks-organize-title")}
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <header>
        <h2>{t("bookmarks-organize-title")}</h2>
        <button type="button" class="close" aria-label={t("bookmarks-close")} onclick={onClose}>×</button>
      </header>
      <div class="list">
        {#if bookmarksStore.items.length === 0}
          <div class="empty">{t("bookmarks-organize-empty")}</div>
        {:else}
          {#each bookmarksStore.items as bm (bm.id)}
            <div class="row">
              <div class="meta">
                {#if editingId === bm.id}
                  <!-- svelte-ignore a11y_autofocus -->
                  <input
                    type="text"
                    class="rename-input"
                    bind:value={editValue}
                    autofocus
                    onkeydown={(e) => {
                      if (e.key === "Enter") void commitRename();
                      else if (e.key === "Escape") {
                        editingId = null;
                        editValue = "";
                      }
                    }}
                    onblur={commitRename}
                  />
                {:else}
                  <span class="name">{bm.name}</span>
                {/if}
                <span class="query">{bm.query}</span>
              </div>
              <div class="actions">
                <button type="button" onclick={() => startRename(bm.id, bm.name)}>{t("bookmarks-rename")}</button>
                <button type="button" class="danger" onclick={() => remove(bm.id)}>{t("action-delete")}</button>
              </div>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    display: grid;
    place-items: center;
    z-index: 100;
  }
  .modal {
    width: 540px;
    max-height: 70vh;
    display: flex;
    flex-direction: column;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 12px 48px rgba(0, 0, 0, 0.4);
  }
  header {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border);
  }
  h2 {
    flex: 1;
    margin: 0;
    font-size: 14px;
    color: var(--text-primary);
  }
  .close {
    background: transparent;
    border: 0;
    color: var(--text-secondary);
    font-size: 20px;
    cursor: pointer;
    padding: 0 4px;
  }
  .list {
    flex: 1;
    overflow-y: auto;
    padding: 8px 0;
  }
  .empty {
    padding: 24px;
    text-align: center;
    color: var(--text-secondary);
    font-size: 13px;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 16px;
  }
  .row:hover {
    background: var(--bg-surface-2);
  }
  .meta {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .name {
    color: var(--text-primary);
    font-size: 13px;
  }
  .query {
    color: var(--text-secondary);
    font-family: var(--font-mono);
    font-size: 11px;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .rename-input {
    background: var(--bg-canvas);
    border: 1px solid var(--accent-cyan);
    border-radius: 4px;
    color: var(--text-primary);
    font-size: 13px;
    padding: 2px 6px;
  }
  .actions {
    display: flex;
    gap: 6px;
  }
  .actions button {
    padding: 3px 10px;
    background: var(--bg-surface-2);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    font-size: 12px;
    cursor: pointer;
  }
  .actions button:hover {
    background: var(--bg-surface);
  }
  .actions .danger {
    color: var(--danger);
  }
</style>
