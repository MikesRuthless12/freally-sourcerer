<script lang="ts">
  import { historyStore } from "../../../lib/stores/history.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import type { HistoryConfig } from "../../../lib/ipc/types";

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

<h1>History</h1>

<Section title="Search History">
  <Checkbox id="hi-search-en" label="Enable search history"
    checked={historyStore.cfg.search_history_enabled}
    onChange={(v) => patch({ search_history_enabled: v })} />
  <NumberInput id="hi-search-days" label="Keep search history for"
    min={1} max={3650} suffix="days"
    value={historyStore.cfg.search_history_keep_days}
    onChange={(n) => patch({ search_history_keep_days: n })} />
  <button type="button" onclick={clearNow} disabled={busy}>Clear Now</button>
</Section>

<Section title="Run History">
  <Checkbox id="hi-run-en" label="Enable run history"
    checked={historyStore.cfg.run_history_enabled}
    onChange={(v) => patch({ run_history_enabled: v })} />
  <NumberInput id="hi-run-days" label="Keep run history for"
    min={1} max={3650} suffix="days"
    value={historyStore.cfg.run_history_keep_days}
    onChange={(n) => patch({ run_history_keep_days: n })} />
</Section>

<Section title="Privacy (+)">
  <Checkbox id="hi-privacy" label="Privacy mode (disables all history features at once)"
    checked={historyStore.cfg.privacy_mode}
    onChange={(v) => patch({ privacy_mode: v })} />
  <Checkbox id="hi-per-lens-fn" label="Record filename-lens history"
    checked={historyStore.cfg.per_lens.filename}
    onChange={(v) => patch({ per_lens: { ...historyStore.cfg.per_lens, filename: v } })} />
  <Checkbox id="hi-per-lens-ct" label="Record content-lens history"
    checked={historyStore.cfg.per_lens.content}
    onChange={(v) => patch({ per_lens: { ...historyStore.cfg.per_lens, content: v } })} />
  <Checkbox id="hi-per-lens-au" label="Record audio-lens history"
    checked={historyStore.cfg.per_lens.audio}
    onChange={(v) => patch({ per_lens: { ...historyStore.cfg.per_lens, audio: v } })} />
  <Checkbox id="hi-per-lens-sm" label="Record similarity-lens history"
    checked={historyStore.cfg.per_lens.similarity}
    onChange={(v) => patch({ per_lens: { ...historyStore.cfg.per_lens, similarity: v } })} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  button { padding: 5px 12px; background: var(--accent-cyan); color: var(--bg-canvas); border: 0; border-radius: 4px; cursor: pointer; font: inherit; margin-top: 6px; }
  button:disabled { opacity: 0.55; cursor: not-allowed; }
</style>
