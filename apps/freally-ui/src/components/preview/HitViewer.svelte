<script lang="ts">
  // Hit-in-context viewer (SRC-M01). Full extracted document, every
  // match highlighted, line numbers, F3 / Shift+F3 between matches.
  import { contentViewStore } from "../../lib/stores/content_view.svelte";
  import { t } from "../../lib/i18n/t";

  let scroller = $state<HTMLDivElement | null>(null);
  let dialog = $state<HTMLDivElement | null>(null);

  const doc = $derived(contentViewStore.doc);
  const currentHit = $derived(
    doc && contentViewStore.current >= 0 ? doc.hits[contentViewStore.current] : null
  );

  /** Hits grouped by line, so rendering a line needs one lookup instead
   *  of a scan of every hit in the document. */
  const hitsByLine = $derived.by(() => {
    const map = new Map<number, { start: number; end: number; index: number }[]>();
    doc?.hits.forEach((h, index) => {
      const list = map.get(h.line) ?? [];
      list.push({ start: h.start, end: h.end, index });
      map.set(h.line, list);
    });
    return map;
  });

  /** Split one line into alternating plain / highlighted segments. */
  function segmentsFor(lineIndex: number, text: string) {
    const hits = hitsByLine.get(lineIndex);
    if (!hits || hits.length === 0) return [{ text, hitIndex: -1 }];
    const chars = [...text];
    const out: { text: string; hitIndex: number }[] = [];
    let cursor = 0;
    // Hits arrive sorted by start; overlapping ones from different terms
    // are merged by skipping any hit that begins before the cursor.
    for (const h of hits) {
      if (h.start < cursor) continue;
      if (h.start > cursor) out.push({ text: chars.slice(cursor, h.start).join(""), hitIndex: -1 });
      out.push({ text: chars.slice(h.start, h.end).join(""), hitIndex: h.index });
      cursor = h.end;
    }
    if (cursor < chars.length) out.push({ text: chars.slice(cursor).join(""), hitIndex: -1 });
    return out;
  }

  $effect(() => {
    if (contentViewStore.open && dialog) dialog.focus();
  });

  // Scroll the current match into view whenever it changes — and
  // whenever F3 is pressed on a document with only one match, where the
  // index cannot change but the user still means "take me back there".
  $effect(() => {
    contentViewStore.scrollNonce;
    const idx = contentViewStore.current;
    if (idx < 0 || !scroller) return;
    const el = scroller.querySelector(`[data-hit="${idx}"]`);
    el?.scrollIntoView({ block: "center", behavior: "auto" });
  });

  function onKeydown(ev: KeyboardEvent) {
    if (ev.key === "Escape") {
      ev.preventDefault();
      contentViewStore.close();
    } else if (ev.key === "F3") {
      ev.preventDefault();
      contentViewStore.step(ev.shiftKey ? -1 : 1);
    }
  }
</script>

{#if contentViewStore.open}
  <div class="backdrop" role="presentation" onpointerdown={() => contentViewStore.close()}></div>
  <div
    bind:this={dialog}
    class="viewer"
    role="dialog"
    aria-modal="true"
    aria-label={t("hitviewer-title", { name: contentViewStore.name })}
    tabindex="-1"
    onkeydown={onKeydown}
  >
    <header>
      <h2>{contentViewStore.name}</h2>
      <span class="badge" aria-live="polite">
        {contentViewStore.hitCount === 0
          ? t("hitviewer-no-matches")
          : t("hitviewer-match-position", {
              current: contentViewStore.current + 1,
              total: contentViewStore.hitCount
            })}
      </span>
      <span class="grow"></span>
      <button
        type="button"
        onclick={() => contentViewStore.step(-1)}
        disabled={contentViewStore.hitCount === 0}
        aria-label={t("hitviewer-prev")}
      >
        {t("hitviewer-prev")}
      </button>
      <button
        type="button"
        onclick={() => contentViewStore.step(1)}
        disabled={contentViewStore.hitCount === 0}
        aria-label={t("hitviewer-next")}
      >
        {t("hitviewer-next")}
      </button>
      <button type="button" onclick={() => contentViewStore.close()} aria-label={t("close")}>
        ×
      </button>
    </header>

    <div bind:this={scroller} class="body">
      {#if contentViewStore.loading}
        <p class="state">{t("hitviewer-loading")}</p>
      {:else if contentViewStore.error}
        <p class="state error">{contentViewStore.error}</p>
      {:else if doc}
        <ol class="lines">
          {#each doc.lines as line, i (i)}
            <li>
              <span class="ln">{i + 1}</span>
              <span class="text"
                >{#each segmentsFor(i, line) as seg, si (si)}{#if seg.hitIndex >= 0}<mark
                      data-hit={seg.hitIndex}
                      class:current={seg.hitIndex === contentViewStore.current}>{seg.text}</mark
                    >{:else}{seg.text}{/if}{/each}</span
              >
            </li>
          {/each}
        </ol>
      {/if}
    </div>

    {#if doc}
      <footer>
        <span>{t("hitviewer-extractor", { id: doc.extractor })}</span>
        {#if doc.truncated}
          <span class="warn">{t("hitviewer-truncated")}</span>
        {/if}
        <span class="grow"></span>
        <span class="keys">{t("hitviewer-keys")}</span>
      </footer>
    {/if}
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgb(0 0 0 / 45%);
    z-index: 80;
  }
  .viewer {
    position: fixed;
    inset: 5vh 6vw;
    z-index: 81;
    display: flex;
    flex-direction: column;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 12px 40px rgb(0 0 0 / 45%);
  }
  .viewer:focus-visible {
    outline: 2px solid var(--accent-cyan);
    outline-offset: -2px;
  }
  header,
  footer {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    background: var(--bg-surface-2);
    font-size: 12px;
    color: var(--text-secondary);
  }
  header {
    border-bottom: 1px solid var(--border);
  }
  footer {
    border-top: 1px solid var(--border);
  }
  h2 {
    margin: 0;
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 40%;
  }
  .badge {
    font-variant-numeric: tabular-nums;
  }
  .grow {
    flex: 1;
  }
  .warn {
    color: var(--accent-amber, #f5a524);
  }
  header button {
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    font-size: 12px;
    padding: 3px 8px;
    cursor: pointer;
  }
  header button:disabled {
    opacity: 0.5;
    cursor: default;
  }
  .body {
    flex: 1;
    overflow: auto;
    background: var(--bg-canvas);
  }
  .state {
    padding: 16px;
    color: var(--text-secondary);
    font-size: 12px;
  }
  .state.error {
    color: var(--accent-red, #e5484d);
  }
  .lines {
    margin: 0;
    padding: 8px 0;
    list-style: none;
    font-family: var(--font-mono, ui-monospace, monospace);
    font-size: 12px;
    line-height: 1.55;
  }
  .lines li {
    display: flex;
    gap: 12px;
    padding: 0 12px;
  }
  .ln {
    flex: 0 0 auto;
    min-width: 4ch;
    text-align: right;
    color: var(--text-secondary);
    user-select: none;
  }
  .text {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    color: var(--text-primary);
  }
  mark {
    background: var(--accent-amber, #f5a524);
    color: #101010;
    border-radius: 2px;
  }
  mark.current {
    background: var(--accent-cyan, #22d3ee);
    outline: 1px solid var(--text-primary);
  }
  .keys {
    font-variant-numeric: tabular-nums;
  }
</style>
