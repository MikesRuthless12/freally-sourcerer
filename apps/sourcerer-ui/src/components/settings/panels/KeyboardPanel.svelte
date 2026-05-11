<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import TextInput from "../controls/TextInput.svelte";
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

<h1>Keyboard</h1>

<Section title="Global Hotkeys">
  <TextInput id="kb-global" label="Global Hotkey" value={settingsStore.state.hotkey}
    placeholder="Super+Space"
    onChange={(v) => patchHotkey(v)} />
  <TextInput id="kb-new" label="New window Hotkey" value={settingsStore.state.keyboard.new_window_hotkey}
    onChange={(v) => patchKbd({ new_window_hotkey: v })} />
  <TextInput id="kb-show" label="Show window Hotkey" value={settingsStore.state.keyboard.show_window_hotkey}
    onChange={(v) => patchKbd({ show_window_hotkey: v })} />
  <TextInput id="kb-toggle" label="Toggle window Hotkey" value={settingsStore.state.keyboard.toggle_window_hotkey}
    onChange={(v) => patchKbd({ toggle_window_hotkey: v })} />
</Section>

<Section title="Commands">
  <TextInput id="kb-filter" label="Show commands containing" value={filter}
    onChange={(v) => (filter = v)} />
  <button type="button" class="add" onclick={addChord}>+ Add chord</button>
  <ul>
    {#each visible as chord, i (i)}
      <li>
        <input class="cmd" placeholder="command id (e.g. file.export_results)"
          value={chord.command}
          oninput={(e) => setChord(i, { command: (e.currentTarget as HTMLInputElement).value })} />
        <input class="bind" placeholder="Ctrl+K, B"
          value={chord.binding}
          oninput={(e) => setChord(i, { binding: (e.currentTarget as HTMLInputElement).value })} />
        <button type="button" onclick={() => removeChord(i)}>Remove</button>
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
