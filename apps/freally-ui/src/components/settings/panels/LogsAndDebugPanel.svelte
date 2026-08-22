<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { LogsAndDebugSettings } from "../../../lib/ipc/types";
  import { t } from "../../../lib/i18n/t";
  import { dialogsStore } from "../../../lib/stores/dialogs.svelte";

  function patch(p: Partial<LogsAndDebugSettings>) {
    settingsStore.patch({ logs_and_debug: { ...settingsStore.state.logs_and_debug, ...p } });
    settingsDialog.markDirty("logs");
  }

  async function openFolder() {
    const path = settingsStore.state.logs_and_debug.log_file_location;
    if (!path) return;
    try {
      const opener = await import("@tauri-apps/plugin-opener");
      await opener.openPath(path);
    } catch (e) {
      console.warn("open log folder failed", e);
    }
  }

  async function exportBundle() {
    try {
      await invoke("export_diagnostics_bundle");
    } catch (e) {
      console.warn("diagnostics bundle export failed (Phase 13)", e);
    }
  }

  // TASK-BR1 drills. The crash → relaunch → offer loop is otherwise only
  // exercised by a real crash, which is a poor moment to discover that
  // the offer never appears.
  //
  // The simulate button keeps the session: it writes a clearly-labelled
  // fake crash file and returns, so the report dialog can be opened
  // against it from Help → Report a Bug. The force button drills the
  // whole loop and therefore really does exit.
  async function simulateCrash() {
    try {
      await invoke("bug_report_simulate_crash");
      dialogsStore.open("bug_report");
    } catch (e) {
      console.warn("simulate crash report failed", e);
    }
  }

  // No confirmation: this is behind a Logs & Debug section titled
  // "Crash-report drills", the button says it exits, and a modal asking
  // "are you sure you want to test the crash test" is noise. It does not
  // reach a user who has not gone looking for it. No catch either — the
  // command does not return.
  async function forceCrash() {
    await invoke("bug_report_force_crash");
  }
</script>

<h1>{t("settings-group-logs")}</h1>

<Section title={t("section-logging")}>
  <Dropdown id="ld-level" label={t("settings-logs-level")}
    value={settingsStore.state.logs_and_debug.log_level}
    options={[ { value: "error", label: t("opt-log-error") }, { value: "warn", label: t("opt-log-warn") }, { value: "info", label: t("opt-log-info-default") }, { value: "debug", label: t("opt-log-debug") }, { value: "trace", label: t("opt-log-trace") } ]}
    onChange={(v) => patch({ log_level: v })} />
  <TextInput id="ld-loc" label={t("settings-logs-location")}
    value={settingsStore.state.logs_and_debug.log_file_location}
    onChange={(v) => patch({ log_file_location: v })} />
  <NumberInput id="ld-ret" label={t("settings-logs-retention")} min={1} max={1000} suffix={t("unit-mb")}
    value={settingsStore.state.logs_and_debug.log_retention_mb}
    onChange={(n) => patch({ log_retention_mb: n })} />
  <Checkbox id="ld-overlay" label={t("settings-logs-debug-overlay")}
    checked={settingsStore.state.logs_and_debug.show_debug_overlay}
    onChange={(v) => patch({ show_debug_overlay: v })} />
</Section>

<Section title={t("section-tools")}>
  <button type="button" onclick={openFolder}>{t("settings-logs-open-folder")}</button>
  <button type="button" onclick={exportBundle}>{t("settings-logs-export-bundle")}</button>
</Section>

<Section title={t("logs-crash-drills")}>
  <button type="button" onclick={simulateCrash}>{t("logs-simulate-crash")}</button>
  <button type="button" onclick={forceCrash}>{t("logs-force-crash")}</button>
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  button { padding: 5px 12px; background: var(--bg-canvas); color: var(--text-primary); border: 1px solid var(--border); border-radius: 3px; cursor: pointer; font: inherit; margin-right: 8px; }
</style>
