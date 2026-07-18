<script lang="ts">
  // "More Freally apps" — the vendored Central panel (view-only) as a React
  // island inside this Svelte dialog. Svelte owns the modal chrome + lifecycle;
  // React owns the panel subtree, localized through our own t().
  import type { Root } from "react-dom/client";
  import { t } from "../../lib/i18n/t";
  import { settingsStore } from "../../lib/stores/settings.svelte";

  interface Props {
    open: boolean;
    onClose: () => void;
  }
  let { open, onClose }: Props = $props();

  let host = $state<HTMLDivElement | null>(null);
  let root: Root | null = null;
  // Lazily imported so React + the vendored panel are a separate async chunk,
  // loaded only when this dialog first opens — never in the app's main bundle.
  let island: typeof import("../../more-apps/mount") | null = null;

  async function ensureMounted() {
    if (!host || root) return;
    island ??= await import("../../more-apps/mount");
    if (host && !root) root = island.mountMoreApps(host);
  }
  function teardown() {
    if (root) {
      root.unmount();
      root = null;
    }
  }

  // Mount when open (container is bound by the time this post-DOM effect runs);
  // unmount when closed.
  $effect(() => {
    if (open && host) void ensureMounted();
    else if (!open) teardown();
  });

  // Re-render on locale change so the panel re-localizes through our t().
  $effect(() => {
    void settingsStore.state.locale;
    if (root && island) island.refreshMoreApps(root);
  });

  // Teardown if the component itself is destroyed while still open.
  $effect(() => teardown);
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
      aria-label={t("moreapps-title")}
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <header class="head">
        <h2>{t("moreapps-title")}</h2>
        <button type="button" class="close" aria-label={t("about-close")} onclick={onClose}>
          ×
        </button>
      </header>
      <div class="body" bind:this={host}></div>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    display: grid;
    place-items: center;
    z-index: 100;
  }
  .modal {
    display: flex;
    flex-direction: column;
    width: min(1000px, 92vw);
    height: min(720px, 86vh);
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 10px;
    box-shadow: 0 12px 48px rgba(0, 0, 0, 0.4);
    overflow: hidden;
  }
  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border);
  }
  .head h2 {
    margin: 0;
    font-size: 16px;
    color: var(--accent-cyan);
  }
  .close {
    background: transparent;
    border: 0;
    color: var(--text-secondary);
    font-size: 22px;
    line-height: 1;
    cursor: pointer;
    padding: 0 6px;
  }
  .body {
    flex: 1 1 auto;
    min-height: 0;
    overflow: auto;
    /* Map the panel's --fcp-* tokens onto this app's theme; the panel ships
       dark defaults, so these only refine the palette. */
    --fcp-bg: var(--bg-canvas);
    --fcp-panel: var(--bg-surface);
    --fcp-card: var(--bg-surface);
    --fcp-muted: var(--text-secondary);
    --fcp-accent: var(--accent-cyan);
    --fcp-accent-1: var(--accent-cyan);
    --fcp-accent-2: var(--accent-cyan);
    --fcp-border: var(--border);
  }
</style>
