<script lang="ts">
  import { resultsStore } from "../../lib/stores/results.svelte";
  import { selectionStore } from "../../lib/stores/selection.svelte";
  import { settingsStore } from "../../lib/stores/settings.svelte";
  import * as files from "../../lib/ipc/files";
  import { mediaKind } from "../../lib/ipc/media";
  import MediaPlayer from "./MediaPlayer.svelte";
  import { t } from "../../lib/i18n/t";
  import type { PreviewPayload } from "../../lib/ipc/types";

  let payload = $state<PreviewPayload | null>(null);
  let loading = $state(false);
  let lastPath = "";

  // SRC-M18 — the selected row, when it is something we can play. Read
  // from the hit rather than from the preview payload: the payload is
  // whatever the preview host could render, and a media file's payload
  // is "unsupported", which is precisely the case worth playing.
  const playable = $derived.by(() => {
    const id = [...selectionStore.ids][0];
    if (!id) return null;
    for (const batch of resultsStore.batches) {
      const hit = batch.hits.find((h) => h.file_id === id);
      if (!hit) continue;
      const kind = mediaKind(hit.ext ?? "");
      return kind ? { path: hit.path, name: hit.name, kind } : null;
    }
    return null;
  });

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
          payload = { kind: "unsupported", message: undefined };
          loading = false;
        },
      );
  });
</script>

{#if settingsStore.state.show_preview}
  <aside class="preview" aria-label={t("preview-header")}>
    <header>{t("preview-header")}</header>
    <div class="body">
      {#if playable}
        <MediaPlayer path={playable.path} name={playable.name} kind={playable.kind} />
      {:else if loading}
        <div class="hint">{t("preview-loading")}</div>
      {:else if !payload}
        <div class="hint">{t("preview-select-file")}</div>
      {:else if payload.kind === "text" && payload.text}
        <pre class="text">{payload.text}</pre>
      {:else if payload.kind === "image" && payload.data_url}
        <img src={payload.data_url} alt={t("preview-header")} />
      {:else}
        <div class="hint">{payload.message ?? t("preview-unavailable")}</div>
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
