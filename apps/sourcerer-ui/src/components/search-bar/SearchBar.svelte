<script lang="ts">
  import { queryStore } from "../../lib/stores/query.svelte";
  import { resultsStore } from "../../lib/stores/results.svelte";
  import { highlight, firstError } from "../../lib/tokenizer/highlight";
  import { t } from "../../lib/i18n/t";

  let inputEl: HTMLInputElement | undefined = $state();

  const segments = $derived(highlight(queryStore.source, queryStore.report));
  const err = $derived(firstError(queryStore.report));

  async function onInput(ev: Event) {
    const v = (ev.target as HTMLInputElement).value;
    await queryStore.setSource(v);
    await resultsStore.run(v);
  }

  function onKey(ev: KeyboardEvent) {
    if (ev.key === "Escape") {
      if (inputEl) inputEl.value = "";
      void queryStore.setSource("");
      void resultsStore.run("");
    }
  }
</script>

<div class="search-bar">
  <div class="input-wrap">
    <div class="mirror" aria-hidden="true">
      {#each segments as seg}
        <span class={seg.className} class:err={seg.isError}>{seg.text}</span>
      {/each}
    </div>
    <input
      bind:this={inputEl}
      type="text"
      spellcheck="false"
      autocomplete="off"
      autocapitalize="off"
      class="raw"
      data-testid="search-input"
      placeholder={t("search-placeholder")}
      aria-label={t("search-placeholder")}
      value={queryStore.source}
      oninput={onInput}
      onkeydown={onKey}
    />
  </div>
  {#if err}
    <div class="error-pill" role="status">
      <span class="dot"></span>
      <span>{err.message}</span>
    </div>
  {/if}
</div>

<style>
  .search-bar {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 8px 12px;
    background: var(--bg-surface);
    border-bottom: 1px solid var(--border);
  }

  .input-wrap {
    position: relative;
    height: 36px;
  }

  .raw,
  .mirror {
    position: absolute;
    inset: 0;
    margin: 0;
    padding: 8px 12px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg-canvas);
    font: 14px/1.4 var(--font-mono);
    white-space: pre;
    overflow: hidden;
  }

  .raw {
    color: var(--text-primary);
    caret-color: var(--text-primary);
    background: var(--bg-canvas);
    z-index: 1;
    outline: none;
  }
  .mirror {
    display: none;
  }
  .raw::placeholder {
    color: var(--text-secondary);
  }
  .raw:focus {
    border-color: var(--accent-cyan);
  }

  .mirror {
    color: var(--text-primary);
    pointer-events: none;
    z-index: 0;
  }

  /* Token classes — keys used by lib/tokenizer/highlight.ts. */
  .mirror :global(.tok-literal) { color: var(--text-primary); }
  .mirror :global(.tok-quoted) { color: var(--success); }
  .mirror :global(.tok-wildcard) { color: var(--accent-cyan); font-weight: 600; }
  .mirror :global(.tok-regex) { color: var(--accent-violet); font-style: italic; }
  .mirror :global(.tok-modifier) { color: var(--accent-orange); font-weight: 600; }
  .mirror :global(.tok-quick-filter) { color: var(--lens-similarity); font-weight: 600; }
  .mirror :global(.tok-lens-prefix) { color: var(--lens-content); font-weight: 700; }
  .mirror :global(.tok-paren) { color: var(--text-secondary); }
  .mirror :global(.tok-operator) { color: var(--warning); font-weight: 600; }
  .mirror :global(.err) {
    text-decoration: underline wavy var(--danger);
    text-underline-offset: 3px;
  }

  .error-pill {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 2px 8px;
    background: color-mix(in srgb, var(--danger) 15%, transparent);
    border: 1px solid var(--danger);
    border-radius: 4px;
    color: var(--danger);
    font-size: 12px;
    width: fit-content;
  }
  .error-pill .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--danger);
  }
</style>
