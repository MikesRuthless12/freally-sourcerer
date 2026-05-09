<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import type { PrivacyAndUpdatesSettings } from "../../../lib/ipc/types";

  function patch(p: Partial<PrivacyAndUpdatesSettings>) {
    settingsStore.patch({ privacy_and_updates: { ...settingsStore.state.privacy_and_updates, ...p } });
    settingsDialog.markDirty("privacy");
  }
</script>

<h1>Privacy & Updates</h1>

<Section title="Auto-update (+)">
  <Dropdown id="pu-au" label="Auto-update"
    value={settingsStore.state.privacy_and_updates.auto_update}
    options={[ { value: "default", label: "On (default)" }, { value: "weekly", label: "Weekly" }, { value: "monthly", label: "Monthly" }, { value: "off", label: "Off" } ]}
    onChange={(v) => patch({ auto_update: v })} />
  <Checkbox id="pu-pre" label="Pre-release channel"
    checked={settingsStore.state.privacy_and_updates.pre_release_channel}
    onChange={(v) => patch({ pre_release_channel: v })} />
</Section>

<Section title="Privacy">
  <p class="muted">Crash reports and telemetry are <strong>permanently disabled</strong> in Sourcerer
  per PRD §8.23 — there is no toggle. The only outbound URL when auto-update is on is
  <code>auto-update.api.sourcerer.app</code>.</p>
</Section>

<Section title="Network calls policy">
  <ul class="urls">
    {#if settingsStore.state.privacy_and_updates.auto_update !== "off"}
      <li><code>auto-update.api.sourcerer.app</code></li>
    {/if}
    {#if settingsStore.state.privacy_and_updates.auto_update === "off"}
      <li class="muted">No outbound URLs — auto-update is off.</li>
    {/if}
  </ul>
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  ul.urls { list-style: none; padding: 0; margin: 0; }
  ul.urls li { padding: 4px 8px; border-bottom: 1px solid var(--border); color: var(--text-primary); font-size: 13px; }
  .muted { color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
  code { font-family: var(--font-mono); background: var(--bg-canvas); border: 1px solid var(--border); border-radius: 2px; padding: 0 4px; font-size: 11px; }
  strong { color: var(--text-primary); }
</style>
