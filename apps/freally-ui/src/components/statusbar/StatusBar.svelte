<script lang="ts">
  import { indexStateStore } from "../../lib/stores/index_state.svelte";
  import { resultsStore } from "../../lib/stores/results.svelte";
  import { selectionStore } from "../../lib/stores/selection.svelte";
  import { settingsStore } from "../../lib/stores/settings.svelte";
  import { menuHoverStore } from "../../lib/stores/menu_hover.svelte";
  import { themeStore } from "../../lib/stores/theme.svelte";
  import { formatBytes, formatCount } from "../../lib/util/format";
  import { t } from "../../lib/i18n/t";

  const phaseLabel = $derived.by(() => {
    const s = indexStateStore.state;
    switch (s.phase) {
      case "indexed": return t("status-indexed", { count: formatCount(s.files_total) });
      case "indexing": return t("status-indexing", { done: formatCount(s.files_indexed), total: formatCount(s.files_total) });
      case "paused": return t("status-paused");
      case "error": return t("status-error");
    }
  });

  const idleHint = $derived(`${t("status-ready")} · ${formatCount(indexStateStore.state.files_total)} ${t("statusbar-indexed-suffix")}`);
</script>

{#if settingsStore.state.show_status_bar}
  <footer class="statusbar" role="status" aria-live="polite">
    <span class="seg pip {indexStateStore.state.phase}" title={t("statusbar-hotkey-hint", { hotkey: settingsStore.state.hotkey })}>
      <span class="dot"></span>
      <span>{phaseLabel}</span>
    </span>

    <span class="seg count">
      {resultsStore.total === 1
        ? t("status-result-count-one", { count: formatCount(resultsStore.total) })
        : t("status-result-count-many", { count: formatCount(resultsStore.total) })}
      {#if selectionStore.count > 0}
        {t("status-selection", { count: formatCount(selectionStore.count) })}
      {/if}
    </span>

    {#if settingsStore.state.show_size_in_status_bar && selectionStore.count > 0}
      <span class="seg size">{t("status-selection-size", { size: formatBytes(selectionStore.bytes) })}</span>
    {/if}

    {#if resultsStore.lastQueryMs > 0}
      <span class="seg timing">{t("status-query-timing", { ms: resultsStore.lastQueryMs })}</span>
    {/if}

    {#if settingsStore.state.show_timing_badges && resultsStore.timings}
      <span class="seg lens-timings">
        {t("lens-filename")} {Math.round(resultsStore.timings.filename_ms)} ms
        · {t("lens-content")} {Math.round(resultsStore.timings.content_ms)} ms
        · {t("lens-audio")} {Math.round(resultsStore.timings.audio_ms)} ms
        · {t("lens-similarity")} {Math.round(resultsStore.timings.similarity_ms)} ms
      </span>
    {/if}

    <span class="seg endpoint">
      {settingsStore.state.endpoint.kind === "remote"
        ? t("status-endpoint-remote", { name: settingsStore.state.endpoint.name })
        : t("status-endpoint-local")}
    </span>

    <span class="seg hover-hint grow">{menuHoverStore.hint ?? idleHint}</span>

    <button
      type="button"
      class="theme-pip"
      aria-label={t("statusbar-cycle-theme")}
      title={`${t("settings-ui-theme")}: ${t(`theme-${themeStore.choice}`)}`}
      onclick={() => {
        themeStore.cycle();
        void settingsStore.patch({ theme: themeStore.choice });
      }}
    >
      {themeStore.choice === "dark" ? "🌙" : themeStore.choice === "light" ? "☀" : "◐"}
    </button>
  </footer>
{/if}

<style>
  .statusbar {
    display: flex;
    align-items: center;
    gap: 12px;
    height: 24px;
    padding: 0 8px;
    background: var(--bg-surface);
    border-top: 1px solid var(--border);
    font-size: 12px;
    color: var(--text-secondary);
    user-select: none;
  }
  .seg {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    white-space: nowrap;
  }
  .pip .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }
  .pip.indexed .dot { background: var(--success); }
  .pip.indexing .dot { background: var(--warning); animation: pulse 1.2s ease-in-out infinite; }
  .pip.paused .dot { background: var(--text-secondary); }
  .pip.error .dot { background: var(--danger); }
  @keyframes pulse {
    0%, 100% { opacity: 0.4; }
    50% { opacity: 1; }
  }
  .grow {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .theme-pip {
    background: transparent;
    border: 0;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 14px;
    padding: 0 4px;
  }
  .theme-pip:hover {
    color: var(--text-primary);
  }
</style>
