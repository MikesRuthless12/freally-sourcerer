<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import type { ContextMenuConfig, ContextMenuEntry } from "../../../lib/ipc/types";

  const ENTRIES: { key: keyof ContextMenuConfig; label: string }[] = [
    { key: "open_folders", label: "Open (Folders)" },
    { key: "open_files", label: "Open (Files)" },
    { key: "open_path", label: "Open Path" },
    { key: "explore", label: "Explore" },
    { key: "explore_path", label: "Explore Path" },
    { key: "copy_name", label: "Copy Name to Clipboard" },
    { key: "copy_path", label: "Copy Path to Clipboard" },
    { key: "copy_full_name", label: "Copy Full Name to Clipboard" },
    { key: "reveal_in_sourcerer", label: "Reveal in Sourcerer (+)" },
    { key: "send_to_sourcerer", label: "Send to Sourcerer (path) (+)" }
  ];

  function update(key: keyof ContextMenuConfig, patch: Partial<ContextMenuEntry>) {
    const cur = settingsStore.state.context_menu[key];
    settingsStore.patch({
      context_menu: { ...settingsStore.state.context_menu, [key]: { ...cur, ...patch } }
    });
    settingsDialog.markDirty("general.context_menu");
  }
</script>

<h1>Context Menu</h1>
<p class="hint">Per-OS shell-extension entries. Each can be Show / Show only when Shift held / Hide,
with an optional command-string macro.</p>

{#each ENTRIES as e (e.key)}
  <Section title={e.label}>
    <Dropdown id={`cm-${e.key}-vis`}
      label="Visibility"
      value={settingsStore.state.context_menu[e.key].visibility}
      options={[ { value: "show", label: "Show" }, { value: "shift_only", label: "Show only when Shift held" }, { value: "hide", label: "Hide" } ]}
      onChange={(v) => update(e.key, { visibility: v })} />
    <TextInput id={`cm-${e.key}-cmd`} label="Command macro" placeholder="(empty = default)"
      value={settingsStore.state.context_menu[e.key].command}
      onChange={(v) => update(e.key, { command: v })} />
  </Section>
{/each}

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  .hint { margin: 0 0 16px; color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
</style>
