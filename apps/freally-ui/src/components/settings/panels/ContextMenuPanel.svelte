<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import { t } from "../../../lib/i18n/t";
  import type { ContextMenuConfig, ContextMenuEntry } from "../../../lib/ipc/types";

  const ENTRIES: { key: keyof ContextMenuConfig; label: string }[] = [
    { key: "open_folders", label: t("settings-context-menu-open-folders") },
    { key: "open_files", label: t("settings-context-menu-open-files") },
    { key: "open_path", label: t("settings-context-menu-open-path") },
    { key: "explore", label: t("settings-context-menu-explore") },
    { key: "explore_path", label: t("settings-context-menu-explore-path") },
    { key: "copy_name", label: t("settings-context-menu-copy-name") },
    { key: "copy_path", label: t("settings-context-menu-copy-path") },
    { key: "copy_full_name", label: t("settings-context-menu-copy-full-name") },
    { key: "reveal_in_freally", label: t("settings-context-menu-reveal") },
    { key: "send_to_freally", label: t("settings-context-menu-send-to") }
  ];

  function update(key: keyof ContextMenuConfig, patch: Partial<ContextMenuEntry>) {
    const cur = settingsStore.state.context_menu[key];
    settingsStore.patch({
      context_menu: { ...settingsStore.state.context_menu, [key]: { ...cur, ...patch } }
    });
    settingsDialog.markDirty("general.context_menu");
  }
</script>

<h1>{t("settings-node-context-menu")}</h1>
<p class="hint">Per-OS shell-extension entries. Each can be Show / Show only when Shift held / Hide,
with an optional command-string macro.</p>

{#each ENTRIES as e (e.key)}
  <Section title={e.label}>
    <Dropdown id={`cm-${e.key}-vis`}
      label={t("settings-context-menu-visibility")}
      value={settingsStore.state.context_menu[e.key].visibility}
      options={[ { value: "show", label: t("settings-context-menu-show") }, { value: "shift_only", label: t("settings-context-menu-shift") }, { value: "hide", label: t("settings-context-menu-hide") } ]}
      onChange={(v) => update(e.key, { visibility: v })} />
    <TextInput id={`cm-${e.key}-cmd`} label={t("settings-context-menu-command")} placeholder="(empty = default)"
      value={settingsStore.state.context_menu[e.key].command}
      onChange={(v) => update(e.key, { command: v })} />
  </Section>
{/each}

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  .hint { margin: 0 0 16px; color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
</style>
