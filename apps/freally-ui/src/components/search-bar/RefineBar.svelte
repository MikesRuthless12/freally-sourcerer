<script lang="ts">
  // Search within results (SRC-M02). Renders only when the user has
  // opened it (Ctrl+Shift+F) or has chips in play — an always-visible
  // second search field would compete with the real one.
  import { refineStore } from "../../lib/stores/refine.svelte";
  import { resultsStore } from "../../lib/stores/results.svelte";
  import { formatCount } from "../../lib/util/format";
  import { t } from "../../lib/i18n/t";

  let input = $state<HTMLInputElement | null>(null);

  const shown = $derived(refineStore.open || refineStore.active);

  // Focus the field whenever the bar opens, including the second press
  // of the shortcut while chips are already showing — which is what the
  // nonce is read for.
  $effect(() => {
    refineStore.focusNonce;
    if (refineStore.open && input) input.focus();
  });

  function onKeydown(ev: KeyboardEvent) {
    if (ev.key === "Enter") {
      ev.preventDefault();
      refineStore.commitDraft();
      return;
    }
    if (ev.key === "Escape") {
      ev.preventDefault();
      refineStore.hide();
      return;
    }
    // Backspace on an empty field peels the last chip — the same
    // gesture every tag input has.
    if (ev.key === "Backspace" && refineStore.draft.length === 0 && refineStore.active) {
      ev.preventDefault();
      refineStore.removeAt(refineStore.chips.length - 1);
    }
  }
</script>

{#if shown}
  <div class="refine" role="search" aria-label={t("refine-label")}>
    <span class="prefix">{t("refine-label")}</span>

    <ul class="chips">
      {#each refineStore.chips as chip, i (chip)}
        <li class="chip">
          <span class="chip-text">{chip}</span>
          <button
            type="button"
            class="chip-remove"
            aria-label={t("refine-remove-chip", { term: chip })}
            onclick={() => refineStore.removeAt(i)}
          >
            ×
          </button>
        </li>
      {/each}
    </ul>

    <input
      bind:this={input}
      bind:value={refineStore.draft}
      type="text"
      aria-label={t("refine-input-label")}
      placeholder={t("refine-placeholder")}
      onkeydown={onKeydown}
    />

    <span class="count">
      {t("refine-count", {
        shown: formatCount(resultsStore.total),
        total: formatCount(resultsStore.totalUnrefined)
      })}
    </span>

    {#if refineStore.active}
      <button type="button" class="clear" onclick={() => refineStore.clear()}>
        {t("refine-clear")}
      </button>
    {/if}
    <button
      type="button"
      class="clear"
      aria-label={t("refine-close")}
      onclick={() => {
        refineStore.clear();
        refineStore.hide();
      }}
    >
      ×
    </button>
  </div>
{/if}

<style>
  .refine {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 12px;
    background: var(--bg-surface-2);
    border-bottom: 1px solid var(--border);
    font-size: 12px;
  }
  .prefix {
    color: var(--text-secondary);
    white-space: nowrap;
  }
  .chips {
    display: flex;
    align-items: center;
    gap: 6px;
    list-style: none;
    margin: 0;
    padding: 0;
    flex-wrap: wrap;
  }
  .chip {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 2px 4px 2px 8px;
    border-radius: 10px;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    color: var(--text-primary);
  }
  .chip-text {
    max-width: 220px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .chip-remove,
  .clear {
    background: transparent;
    border: 0;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 0 3px;
    line-height: 1;
  }
  .chip-remove:hover,
  .clear:hover {
    color: var(--text-primary);
  }
  input {
    flex: 1;
    min-width: 120px;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    padding: 3px 6px;
    font-size: 12px;
  }
  input:focus-visible {
    outline: 2px solid var(--accent-cyan);
    outline-offset: 1px;
  }
  .count {
    color: var(--text-secondary);
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
  }
</style>
