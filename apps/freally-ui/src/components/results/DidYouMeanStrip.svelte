<script lang="ts">
  // SRC-M11 — the correction offered when a search matched nothing.
  //
  // Renders above the (empty) result list rather than inside it: the
  // lens sections still draw their own "no hits" rows, and this is a
  // statement about the *query*, not about any one lens.
  //
  // Accepting runs `suggestion.query`, which the daemon already built
  // by swapping the mistyped term into the original source — so
  // modifiers, booleans and quoting survive untouched.
  import { resultsStore } from "../../lib/stores/results.svelte";
  import { queryStore } from "../../lib/stores/query.svelte";
  import { t } from "../../lib/i18n/t";

  const suggestion = $derived(resultsStore.didYouMean);

  async function accept() {
    const s = resultsStore.didYouMean;
    if (!s) return;
    // Put the corrected text in the search box too, so the box and the
    // results agree and a subsequent edit starts from the correction.
    queryStore.source = s.query;
    await resultsStore.run(s.query);
  }
</script>

{#if suggestion}
  <div class="dym" role="status">
    <span class="label">{t("did-you-mean-label")}</span>
    <button type="button" class="suggest" onclick={accept}>
      {suggestion.suggested}
    </button>
  </div>
{/if}

<style>
  .dym {
    display: flex;
    align-items: baseline;
    gap: 8px;
    padding: 8px 10px;
    margin-bottom: 8px;
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    background: var(--bg-elevated);
    font-size: 13px;
  }
  .label {
    color: var(--text-secondary);
  }
  .suggest {
    background: none;
    border: none;
    padding: 0;
    font: inherit;
    font-weight: 600;
    color: var(--accent);
    cursor: pointer;
    text-decoration: underline;
  }
  .suggest:hover,
  .suggest:focus-visible {
    text-decoration: none;
  }
</style>
