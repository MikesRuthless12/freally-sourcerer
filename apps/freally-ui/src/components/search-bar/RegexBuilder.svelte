<script lang="ts">
  // SRC-M20 — compose a regex against the results you already have.
  //
  // The parse-error pill and the AST hover validate a query you have
  // already written. Neither helps you *write* the pattern, which is
  // where the time actually goes: you type something, run it, get
  // nothing, and cannot tell whether the pattern is wrong or the files
  // are absent. This shows both at once — the engine's own error, and
  // which of the names on screen the pattern currently hits.

  import { resultsStore } from "../../lib/stores/results.svelte";
  import { queryStore } from "../../lib/stores/query.svelte";
  import { searchOptsStore } from "../../lib/stores/search_opts.svelte";
  import * as regexIpc from "../../lib/ipc/regex_test";
  import type { RegexTestResult, MatchSpan } from "../../lib/ipc/regex_test";
  import { t } from "../../lib/i18n/t";

  interface Props {
    onclose: () => void;
  }
  let { onclose }: Props = $props();

  /** The feature line asks for the current top 50 result names. */
  const SAMPLE_LIMIT = 50;

  let pattern = $state("");
  let result = $state<RegexTestResult | null>(null);
  let inputEl: HTMLInputElement | undefined = $state();

  // `visibleHits`, not `batches`: the latter includes lenses the user
  // switched off and rows the refine bar filtered out, so the builder
  // would test the pattern against names that are not on screen and the
  // "N of 50" count would disagree with the list behind it.
  const subjects = $derived(
    resultsStore.visibleHits.slice(0, SAMPLE_LIMIT).map((h) => h.name)
  );

  const matchedCount = $derived(result?.matches.length ?? 0);

  // Spans keyed by subject index, so the render below is a lookup
  // rather than a scan per row.
  const spansByIndex = $derived(
    new Map<number, MatchSpan[]>((result?.matches ?? []).map((m) => [m.index, m.spans]))
  );

  $effect(() => {
    // Re-run whenever the pattern, the sample, or Match Case changes.
    const p = pattern;
    const s = subjects;
    const c = searchOptsStore.get("match_case");
    let cancelled = false;
    void (async () => {
      try {
        const r = await regexIpc.test(p, s, c);
        if (!cancelled) result = r;
      } catch {
        if (!cancelled) result = null;
      }
    })();
    return () => {
      cancelled = true;
    };
  });

  $effect(() => {
    inputEl?.focus();
  });

  /** Split a name into alternating unmatched / matched runs. */
  function segments(name: string, spans: MatchSpan[] | undefined) {
    const chars = [...name];
    if (!spans || spans.length === 0) return [{ text: name, hit: false }];
    const out: { text: string; hit: boolean }[] = [];
    let cursor = 0;
    for (const s of spans) {
      if (s.start > cursor) out.push({ text: chars.slice(cursor, s.start).join(""), hit: false });
      out.push({ text: chars.slice(s.start, s.end).join(""), hit: true });
      cursor = s.end;
    }
    if (cursor < chars.length) out.push({ text: chars.slice(cursor).join(""), hit: false });
    return out;
  }

  async function usePattern() {
    if (!result?.valid) return;
    // Committing the pattern turns Enable Regex on: a `regex:` term
    // works either way, but leaving the toggle off would make the menu
    // disagree with what the query is doing.
    if (!searchOptsStore.get("enable_regex")) await searchOptsStore.toggle("enable_regex");
    await queryStore.setSource(`regex:${pattern}`);
    await resultsStore.run(`regex:${pattern}`);
    onclose();
  }

  function onKey(ev: KeyboardEvent) {
    if (ev.key === "Escape") {
      ev.stopPropagation();
      onclose();
    } else if (ev.key === "Enter") {
      void usePattern();
    }
  }

  /** Rust-regex flavour, not JavaScript's — the differences that bite. */
  const CHEATSHEET: { code: string; l10n: string }[] = [
    { code: ".", l10n: "regex-cheat-any" },
    { code: "\\d  \\w  \\s", l10n: "regex-cheat-classes" },
    { code: "[a-z]  [^a-z]", l10n: "regex-cheat-set" },
    { code: "*  +  ?", l10n: "regex-cheat-repeat" },
    { code: "{2}  {2,}  {2,5}", l10n: "regex-cheat-count" },
    { code: "^  $", l10n: "regex-cheat-anchor" },
    { code: "(a|b)", l10n: "regex-cheat-alt" },
    { code: "(?i)", l10n: "regex-cheat-inline-flag" },
    { code: "(?:…)", l10n: "regex-cheat-noncapturing" },
    { code: "\\.  \\(", l10n: "regex-cheat-escape" }
  ];
</script>

<!-- `tabindex="-1"` because an element with role="dialog" must be
     focusable. Focus goes to the pattern input on open; this only makes
     the container a valid focus target so the role is well-formed. -->
<div
  class="popover"
  role="dialog"
  tabindex="-1"
  aria-label={t("regex-builder-title")}
  onkeydown={onKey}
>
  <div class="head">
    <strong>{t("regex-builder-title")}</strong>
    <button type="button" class="close" onclick={onclose} aria-label={t("about-close")}>×</button>
  </div>

  <input
    bind:this={inputEl}
    bind:value={pattern}
    type="text"
    class="pattern"
    class:invalid={pattern !== "" && result !== null && !result.valid}
    spellcheck="false"
    autocomplete="off"
    data-testid="regex-pattern"
    placeholder={t("regex-builder-placeholder")}
    aria-label={t("regex-builder-title")}
  />

  {#if result?.error}
    <p class="error" role="status">{result.error}</p>
  {:else if pattern !== ""}
    <p class="count" role="status">
      {t("regex-builder-match-count", { matched: matchedCount, total: subjects.length })}
    </p>
  {/if}

  <div class="sample">
    {#if subjects.length === 0}
      <p class="muted">{t("regex-builder-no-results")}</p>
    {:else}
      {#each subjects as name, i}
        <div class="row" class:dim={pattern !== "" && !spansByIndex.has(i)}>
          {#each segments(name, spansByIndex.get(i)) as seg}
            <span class:hit={seg.hit}>{seg.text}</span>
          {/each}
        </div>
      {/each}
    {/if}
  </div>

  <details class="cheat">
    <summary>{t("regex-builder-cheatsheet")}</summary>
    <p class="muted">{t("regex-builder-flavor-note")}</p>
    <dl>
      {#each CHEATSHEET as c}
        <div class="cheat-row">
          <dt>{c.code}</dt>
          <dd>{t(c.l10n)}</dd>
        </div>
      {/each}
    </dl>
  </details>

  <div class="foot">
    <button
      type="button"
      class="primary"
      disabled={!result?.valid}
      onclick={usePattern}
      data-testid="regex-use">{t("regex-builder-use")}</button
    >
  </div>
</div>

<style>
  .popover {
    position: absolute;
    top: calc(100% + 6px);
    right: 12px;
    z-index: 40;
    width: min(460px, calc(100vw - 24px));
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 8px 24px rgb(0 0 0 / 35%);
  }
  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: var(--text-primary);
    font-size: 13px;
  }
  .close {
    background: none;
    border: 0;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 18px;
    line-height: 1;
    padding: 0 4px;
  }
  .pattern {
    padding: 6px 8px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg-canvas);
    color: var(--accent-violet);
    font: 13px/1.4 var(--font-mono);
  }
  .pattern:focus {
    outline: none;
    border-color: var(--accent-cyan);
  }
  .pattern.invalid {
    border-color: var(--danger);
  }
  .error {
    margin: 0;
    color: var(--danger);
    font: 11px/1.4 var(--font-mono);
    white-space: pre-wrap;
    max-height: 84px;
    overflow: auto;
  }
  .count {
    margin: 0;
    color: var(--text-secondary);
    font-size: 12px;
  }
  .sample {
    max-height: 200px;
    overflow: auto;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg-canvas);
    padding: 4px 6px;
  }
  .row {
    font: 12px/1.6 var(--font-mono);
    color: var(--text-primary);
    white-space: pre;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .row.dim {
    opacity: 0.45;
  }
  .row .hit {
    background: color-mix(in srgb, var(--accent-cyan) 35%, transparent);
    border-radius: 2px;
  }
  .muted {
    margin: 0;
    color: var(--text-secondary);
    font-size: 12px;
  }
  .cheat summary {
    cursor: pointer;
    color: var(--text-secondary);
    font-size: 12px;
  }
  .cheat dl {
    margin: 6px 0 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .cheat-row {
    display: grid;
    grid-template-columns: 130px 1fr;
    gap: 8px;
  }
  .cheat dt {
    color: var(--accent-violet);
    font: 11px/1.5 var(--font-mono);
  }
  .cheat dd {
    margin: 0;
    color: var(--text-secondary);
    font-size: 11px;
    line-height: 1.5;
  }
  .foot {
    display: flex;
    justify-content: flex-end;
  }
  .primary {
    padding: 5px 12px;
    border: 1px solid var(--accent-cyan);
    border-radius: 4px;
    background: color-mix(in srgb, var(--accent-cyan) 20%, transparent);
    color: var(--text-primary);
    font-size: 12px;
    cursor: pointer;
  }
  .primary:disabled {
    opacity: 0.5;
    cursor: default;
  }
</style>
