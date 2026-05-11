<script lang="ts">
  import { resultsStore } from "../../lib/stores/results.svelte";
  import { selectionStore } from "../../lib/stores/selection.svelte";
  import { settingsStore } from "../../lib/stores/settings.svelte";
  import * as files from "../../lib/ipc/files";
  import type { PreviewPayload } from "../../lib/ipc/types";

  let payload = $state<PreviewPayload | null>(null);
  let loading = $state(false);
  let lastPath = "";

  $effect(() => {
    if (!settingsStore.state.show_preview) return;
    const id = [...selectionStore.ids][0];
    if (!id) {
      payload = null;
      lastPath = "";
      return;
    }
    let path: string | undefined;
    for (const batch of resultsStore.batches) {
      const hit = batch.hits.find((h) => h.file_id === id);
      if (hit) {
        path = hit.path;
        break;
      }
    }
    if (!path || path === lastPath) return;
    lastPath = path;
    loading = true;
    // KnownPaths only gets populated by user-initiated dialogs by
    // default; query-result hits aren't registered, so we whitelist
    // the path explicitly right before calling `files.preview` — the
    // user just selected it, which is a legitimate trust signal.
    const target = path;
    const t0 = performance.now();
    console.log("[preview] requesting", target);
    files
      .whitelistUserChosen(target)
      .catch((e) => console.warn("[preview] whitelist failed:", e))
      .then(() => files.preview(target))
      .then(
        (p) => {
          console.log(
            "[preview] resolved",
            target,
            "kind:",
            p.kind,
            "ms:",
            Math.round(performance.now() - t0),
          );
          payload = p;
          loading = false;
        },
        (e) => {
          console.error("[preview] rejected", target, e);
          payload = { kind: "unsupported", message: "Preview unavailable" };
          loading = false;
        },
      );
  });
</script>

{#if settingsStore.state.show_preview}
  <aside class="preview" aria-label="Preview pane">
    <header>Preview</header>
    <div class="body">
      {#if loading}
        <div class="hint">Loading…</div>
      {:else if !payload}
        <div class="hint">Select a file to preview.</div>
      {:else if payload.kind === "text" && payload.text}
        <pre class="text">{payload.text}</pre>
      {:else if payload.kind === "image" && payload.data_url}
        <img src={payload.data_url} alt="Preview" />
      {:else}
        <div class="hint">{payload.message ?? "No preview available"}</div>
      {/if}
    </div>
  </aside>
{/if}

<style>
  .preview {
    width: 360px;
    flex-shrink: 0;
    background: var(--bg-surface);
    border-left: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  header {
    padding: 8px 12px;
    background: var(--bg-surface-2);
    border-bottom: 1px solid var(--border);
    color: var(--text-secondary);
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }
  .body {
    flex: 1;
    overflow: auto;
    padding: 12px;
  }
  .hint {
    color: var(--text-secondary);
    font-size: 12px;
    text-align: center;
    margin-top: 24px;
  }
  .text {
    margin: 0;
    font-family: var(--font-mono);
    font-size: 12px;
    color: var(--text-primary);
    white-space: pre-wrap;
    word-break: break-word;
  }
  img {
    max-width: 100%;
    height: auto;
    display: block;
    margin: 0 auto;
  }
</style>
