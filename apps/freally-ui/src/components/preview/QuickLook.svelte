<script lang="ts">
  // SRC-M19 — the Space-bar preview modal.
  //
  // Deliberately not a second preview *implementation*: it calls the
  // same `files.preview` command the docked pane does, so a format that
  // renders in one renders in the other. What it adds is size and a
  // keyboard flow — arrows walk the results underneath while it stays
  // open, and the header tells you where you are in the set.

  import { quickLookStore, visibleHits } from "../../lib/stores/quicklook.svelte";
  import * as files from "../../lib/ipc/files";
  import { t } from "../../lib/i18n/t";
  import type { PreviewPayload } from "../../lib/ipc/types";

  let payload = $state<PreviewPayload | null>(null);
  let loading = $state(false);
  let nativeError = $state<string | null>(null);

  const hit = $derived(quickLookStore.current);
  const position = $derived.by(() => {
    const h = hit;
    if (!h) return null;
    const hits = visibleHits();
    const at = hits.findIndex((x) => x.file_id === h.file_id);
    return at < 0 ? null : { at: at + 1, total: hits.length };
  });

  $effect(() => {
    const h = hit;
    if (!h) {
      payload = null;
      return;
    }
    const target = h.path;
    let cancelled = false;
    loading = true;
    nativeError = null;
    void (async () => {
      try {
        // Same trust signal the docked pane uses: the user selected
        // this row, so registering it before previewing is legitimate.
        await files.whitelistUserChosen(target).catch(() => {});
        const p = await files.preview(target);
        if (!cancelled) payload = p;
      } catch {
        if (!cancelled) payload = { kind: "unsupported", message: undefined };
      } finally {
        if (!cancelled) loading = false;
      }
    })();
    return () => {
      cancelled = true;
    };
  });

  async function openNative() {
    const h = hit;
    if (!h) return;
    try {
      await files.quickLookNative(h.path);
    } catch (e) {
      nativeError = String(e);
    }
  }

  const isMac =
    typeof navigator !== "undefined" && /mac/i.test(navigator.platform || navigator.userAgent);
</script>

<!-- The backdrop closes on click; the panel stops propagation so a
     click inside it does not dismiss. Keyboard handling lives on the
     window listener in bootstrap.ts, which owns Space / arrows / Escape
     for the whole app and must stay the single owner of those keys. -->
<div
  class="backdrop"
  role="presentation"
  data-testid="quicklook"
  onclick={() => quickLookStore.close()}
>
  <div
    class="panel"
    role="dialog"
    tabindex="-1"
    aria-label={t("quicklook-title")}
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
  >
    <header>
      <div class="name" title={hit?.path}>{hit?.name ?? ""}</div>
      <div class="right">
        {#if position}
          <span class="pos">{t("quicklook-position", position)}</span>
        {/if}
        {#if isMac}
          <button type="button" class="native" onclick={openNative}>
            {t("quicklook-open-native")}
          </button>
        {/if}
        <button
          type="button"
          class="close"
          onclick={() => quickLookStore.close()}
          aria-label={t("about-close")}>×</button
        >
      </div>
    </header>

    <div class="body">
      {#if loading}
        <div class="hint">{t("preview-loading")}</div>
      {:else if nativeError}
        <div class="hint">{nativeError}</div>
      {:else if !payload}
        <div class="hint">{t("preview-select-file")}</div>
      {:else if payload.kind === "text" && payload.text}
        <pre class="text">{payload.text}</pre>
      {:else if payload.kind === "image" && payload.data_url}
        <img src={payload.data_url} alt={hit?.name ?? t("quicklook-title")} />
      {:else}
        <div class="hint">{payload.message ?? t("preview-unavailable")}</div>
      {/if}
    </div>

    <footer>{t("quicklook-hint")}</footer>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 60;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgb(0 0 0 / 55%);
  }
  .panel {
    display: flex;
    flex-direction: column;
    width: min(900px, 90vw);
    height: min(720px, 86vh);
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 10px;
    box-shadow: 0 16px 48px rgb(0 0 0 / 45%);
    overflow: hidden;
  }
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 10px 14px;
    background: var(--bg-surface-2);
    border-bottom: 1px solid var(--border);
  }
  .name {
    color: var(--text-primary);
    font-size: 13px;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .right {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-shrink: 0;
  }
  .pos {
    color: var(--text-secondary);
    font-size: 12px;
  }
  .native {
    padding: 3px 8px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: none;
    color: var(--text-secondary);
    font-size: 11px;
    cursor: pointer;
  }
  .close {
    background: none;
    border: 0;
    color: var(--text-secondary);
    font-size: 20px;
    line-height: 1;
    cursor: pointer;
    padding: 0 2px;
  }
  .body {
    flex: 1;
    overflow: auto;
    padding: 16px;
    display: flex;
    flex-direction: column;
  }
  .hint {
    margin: auto;
    color: var(--text-secondary);
    font-size: 13px;
  }
  .text {
    margin: 0;
    font-family: var(--font-mono);
    font-size: 13px;
    line-height: 1.5;
    color: var(--text-primary);
    white-space: pre-wrap;
    word-break: break-word;
  }
  img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
    margin: auto;
    display: block;
  }
  footer {
    padding: 8px 14px;
    border-top: 1px solid var(--border);
    background: var(--bg-surface-2);
    color: var(--text-secondary);
    font-size: 11px;
  }
</style>
