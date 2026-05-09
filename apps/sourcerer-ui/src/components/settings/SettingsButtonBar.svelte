<script lang="ts">
  import { settingsDialog } from "../../lib/stores/settings_dialog.svelte";

  let busy = $state(false);

  async function restoreDefaults() {
    if (busy) return;
    busy = true;
    try {
      await settingsDialog.resetPanel(settingsDialog.selected);
    } finally {
      busy = false;
    }
  }

  async function apply() {
    if (busy || !settingsDialog.dirty) return;
    busy = true;
    try {
      await settingsDialog.apply();
    } finally {
      busy = false;
    }
  }

  async function ok() {
    if (busy) return;
    busy = true;
    try {
      await settingsDialog.ok();
    } finally {
      busy = false;
    }
  }

  function cancel() {
    if (busy) return;
    settingsDialog.cancel();
  }
</script>

<div class="bar">
  <button type="button" onclick={restoreDefaults} disabled={busy}>Restore Defaults</button>
  <span class="spacer"></span>
  <button type="button" onclick={ok} disabled={busy} class="primary">OK</button>
  <button type="button" onclick={cancel} disabled={busy}>Cancel</button>
  <button
    type="button"
    onclick={apply}
    disabled={busy || !settingsDialog.dirty}
    class="primary"
    class:enabled={settingsDialog.dirty}>Apply</button
  >
</div>

<style>
  .bar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    border-top: 1px solid var(--border);
    background: var(--bg-surface);
  }
  .spacer {
    flex: 1;
  }
  button {
    padding: 6px 14px;
    background: var(--bg-canvas);
    color: var(--text-primary);
    border: 1px solid var(--border);
    border-radius: 4px;
    cursor: pointer;
    font: inherit;
    min-width: 80px;
  }
  button:hover:not(:disabled) {
    background: var(--bg-surface);
  }
  button.primary {
    background: var(--accent-cyan);
    border-color: var(--accent-cyan);
    color: var(--bg-canvas);
  }
  button.primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
