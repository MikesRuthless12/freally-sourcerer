<script lang="ts">
  import { historyStore } from "../../../lib/stores/history.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import type { HistoryConfig } from "../../../lib/ipc/types";
  import { t } from "../../../lib/i18n/t";

  function patch(p: Partial<HistoryConfig>) {
    historyStore.patch(p);
    settingsDialog.markDirty("history");
  }

  let busy = $state(false);
  async function clearNow() {
    if (busy) return;
    busy = true;
    try {
      await historyStore.clear();
    } finally {
      busy = false;
    }
  }
</script>

<h1>{t("settings-group-history")}</h1>

<Section title={t("history-section-search")}>
  <Checkbox id="hi-search-en" label={t("settings-history-search-enable")}
    checked={historyStore.cfg.search_history_enabled}
    onChange={(v) => patch({ search_history_enabled: v })} />
  <NumberInput id="hi-search-days" label={t("settings-history-search-keep", { days: historyStore.cfg.search_history_keep_days })}
    min={1} max={3650} suffix={t("unit-days")}
    value={historyStore.cfg.search_history_keep_days}
    onChange={(n) => patch({ search_history_keep_days: n })} />
  <button type="button" onclick={clearNow} disabled={busy}>{t("settings-history-clear-now")}</button>
</Section>

<Section title={t("history-section-run")}>
  <Checkbox id="hi-run-en" label={t("settings-history-run-enable")}
    checked={historyStore.cfg.run_history_enabled}
    onChange={(v) => patch({ run_history_enabled: v })} />
  <NumberInput id="hi-run-days" label={t("settings-history-run-keep", { days: historyStore.cfg.run_history_keep_days })}
    min={1} max={3650} suffix={t("unit-days")}
    value={historyStore.cfg.run_history_keep_days}
    onChange={(n) => patch({ run_history_keep_days: n })} />
</Section>

<Section title={t("history-section-privacy")}>
  <Checkbox id="hi-privacy" label={t("settings-history-privacy-mode")}
    checked={historyStore.cfg.privacy_mode}
    onChange={(v) => patch({ privacy_mode: v })} />
  <Checkbox id="hi-per-lens-fn" label={t("history-record-filename")}
    checked={historyStore.cfg.per_lens.filename}
    onChange={(v) => patch({ per_lens: { ...historyStore.cfg.per_lens, filename: v } })} />
  <Checkbox id="hi-per-lens-ct" label={t("history-record-content")}
    checked={historyStore.cfg.per_lens.content}
    onChange={(v) => patch({ per_lens: { ...historyStore.cfg.per_lens, content: v } })} />
  <Checkbox id="hi-per-lens-au" label={t("history-record-audio")}
    checked={historyStore.cfg.per_lens.audio}
    onChange={(v) => patch({ per_lens: { ...historyStore.cfg.per_lens, audio: v } })} />
  <Checkbox id="hi-per-lens-sm" label={t("history-record-similarity")}
    checked={historyStore.cfg.per_lens.similarity}
    onChange={(v) => patch({ per_lens: { ...historyStore.cfg.per_lens, similarity: v } })} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  button { padding: 5px 12px; background: var(--accent-cyan); color: var(--bg-canvas); border: 0; border-radius: 4px; cursor: pointer; font: inherit; margin-top: 6px; }
  button:disabled { opacity: 0.55; cursor: not-allowed; }
</style>
