<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import { t } from "../../../lib/i18n/t";
  import type { KeyboardChord, KeyboardState } from "../../../lib/ipc/types";

  let filter = $state("");

  function patchKbd(p: Partial<KeyboardState>) {
    settingsStore.patch({ keyboard: { ...settingsStore.state.keyboard, ...p } });
    settingsDialog.markDirty("general.keyboard");
  }

  function patchHotkey(v: string) {
    settingsStore.patch({ hotkey: v });
    settingsDialog.markDirty("general.keyboard");
  }

  function addChord() {
    const next: KeyboardChord = { command: "", binding: "" };
    patchKbd({ per_action: [...settingsStore.state.keyboard.per_action, next] });
  }
  function removeChord(idx: number) {
    const next = settingsStore.state.keyboard.per_action.slice();
    next.splice(idx, 1);
    patchKbd({ per_action: next });
  }
  function setChord(idx: number, p: Partial<KeyboardChord>) {
    const next = settingsStore.state.keyboard.per_action.slice();
    next[idx] = { ...next[idx], ...p };
    patchKbd({ per_action: next });
  }

  let visible = $derived(
    settingsStore.state.keyboard.per_action.filter((c) =>
      c.command.toLowerCase().includes(filter.toLowerCase())
    )
  );
</script>

<h1>{t("settings-node-keyboard")}</h1>

<Section title={t("keyboard-section-global")}>
  <TextInput id="kb-global" label={t("settings-keyboard-global-hotkey")} value={settingsStore.state.hotkey}
    placeholder={t("keyboard-placeholder-example")}
    onChange={(v) => patchHotkey(v)} />
  <TextInput id="kb-new" label={t("settings-keyboard-new-window")} value={settingsStore.state.keyboard.new_window_hotkey}
    onChange={(v) => patchKbd({ new_window_hotkey: v })} />
  <TextInput id="kb-show" label={t("settings-keyboard-show-window")} value={settingsStore.state.keyboard.show_window_hotkey}
    onChange={(v) => patchKbd({ show_window_hotkey: v })} />
  <TextInput id="kb-toggle" label={t("settings-keyboard-toggle-window")} value={settingsStore.state.keyboard.toggle_window_hotkey}
    onChange={(v) => patchKbd({ toggle_window_hotkey: v })} />
</Section>

<Section title={t("keyboard-section-commands")}>
  <TextInput id="kb-filter" label={t("settings-keyboard-show-commands")} value={filter}
    onChange={(v) => (filter = v)} />
  <button type="button" class="add" onclick={addChord}>{t("settings-keyboard-add-chord")}</button>
  <ul>
    {#each visible as chord, i (i)}
      <li>
        <input class="cmd" placeholder={t("keyboard-placeholder-command")}
          value={chord.command}
          oninput={(e) => setChord(i, { command: (e.currentTarget as HTMLInputElement).value })} />
        <input class="bind" placeholder={t("keyboard-placeholder-binding")}
          value={chord.binding}
          oninput={(e) => setChord(i, { binding: (e.currentTarget as HTMLInputElement).value })} />
        <button type="button" onclick={() => removeChord(i)}>{t("settings-keyboard-remove-chord")}</button>
      </li>
    {/each}
  </ul>
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  ul { list-style: none; padding: 0; margin: 8px 0 0; }
  li { display: flex; gap: 6px; align-items: center; padding: 3px 0; }
  .cmd { flex: 1; padding: 4px 6px; background: var(--bg-canvas); color: var(--text-primary); border: 1px solid var(--border); border-radius: 3px; font: inherit; }
  .bind { width: 200px; padding: 4px 6px; background: var(--bg-canvas); color: var(--text-primary); border: 1px solid var(--border); border-radius: 3px; font: inherit; }
  button { padding: 3px 8px; background: var(--bg-canvas); color: var(--text-primary); border: 1px solid var(--border); border-radius: 3px; cursor: pointer; font: inherit; }
  .add { margin: 8px 0; }
</style>
