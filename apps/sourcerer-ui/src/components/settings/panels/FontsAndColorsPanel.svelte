<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import type {
    FontsAndColorsState,
    ItemStateStyle,
    LensId,
    RgbColor
  } from "../../../lib/ipc/types";

  const STATE_KEYS: { key: keyof FontsAndColorsState["states"]; label: string }[] = [
    { key: "normal", label: "Normal" },
    { key: "highlighted", label: "Highlighted" },
    { key: "current_sort", label: "Current Sort" },
    { key: "current_sort_highlighted", label: "Current Sort (Highlighted)" },
    { key: "selected", label: "Selected" },
    { key: "selected_highlighted", label: "Selected (Highlighted)" },
    { key: "inactive_selected", label: "Inactive Selected" },
    { key: "inactive_selected_highlighted", label: "Inactive Selected (Highlighted)" }
  ];

  function rgbToHex(c: RgbColor | null): string {
    if (!c) return "#000000";
    return "#" + [c.r, c.g, c.b].map((n) => n.toString(16).padStart(2, "0")).join("");
  }
  function hexToRgb(s: string): RgbColor {
    const r = parseInt(s.slice(1, 3), 16);
    const g = parseInt(s.slice(3, 5), 16);
    const b = parseInt(s.slice(5, 7), 16);
    return { r, g, b };
  }

  function updateState(key: keyof FontsAndColorsState["states"], patch: Partial<ItemStateStyle>) {
    const cur = settingsStore.state.fonts_and_colors;
    settingsStore.patch({
      fonts_and_colors: {
        ...cur,
        states: {
          ...cur.states,
          [key]: { ...cur.states[key], ...patch }
        }
      }
    });
    settingsDialog.markDirty("general.fonts_colors");
  }

  function updateRoot(patch: Partial<FontsAndColorsState>) {
    settingsStore.patch({ fonts_and_colors: { ...settingsStore.state.fonts_and_colors, ...patch } });
    settingsDialog.markDirty("general.fonts_colors");
  }

  function updateLensAccent(lens: LensId, hex: string | null) {
    const cur = settingsStore.state.fonts_and_colors;
    settingsStore.patch({
      fonts_and_colors: {
        ...cur,
        per_lens_accent: { ...cur.per_lens_accent, [lens]: hex ? hexToRgb(hex) : null }
      }
    });
    settingsDialog.markDirty("general.fonts_colors");
  }
</script>

<h1>Fonts & Colors</h1>

<Section title="Font (E)">
  <TextInput id="fc-font" label="Font" value={settingsStore.state.fonts_and_colors.font}
    onChange={(v) => updateRoot({ font: v })} />
  <NumberInput id="fc-size" label="Size" min={9} max={24} value={settingsStore.state.fonts_and_colors.size_px}
    suffix="px" onChange={(n) => updateRoot({ size_px: n })} />
</Section>

{#each STATE_KEYS as s (s.key)}
  <Section title={s.label}>
    <div class="row">
      <span class="lbl">Foreground</span>
      <input type="color" aria-label={`${s.label} foreground`}
        value={rgbToHex(settingsStore.state.fonts_and_colors.states[s.key].fg)}
        onchange={(e) => updateState(s.key, { fg: hexToRgb((e.currentTarget as HTMLInputElement).value) })} />
      <button type="button" onclick={() => updateState(s.key, { fg: null })}>Default</button>
    </div>
    <div class="row">
      <span class="lbl">Background</span>
      <input type="color" aria-label={`${s.label} background`}
        value={rgbToHex(settingsStore.state.fonts_and_colors.states[s.key].bg)}
        onchange={(e) => updateState(s.key, { bg: hexToRgb((e.currentTarget as HTMLInputElement).value) })} />
      <button type="button" onclick={() => updateState(s.key, { bg: null })}>Default</button>
    </div>
    <Checkbox id={`fc-${String(s.key)}-bold`} label="Bold"
      checked={settingsStore.state.fonts_and_colors.states[s.key].bold}
      onChange={(v) => updateState(s.key, { bold: v })} />
    <Checkbox id={`fc-${String(s.key)}-italic`} label="Italic"
      checked={settingsStore.state.fonts_and_colors.states[s.key].italic}
      onChange={(v) => updateState(s.key, { italic: v })} />
  </Section>
{/each}

<Section title="Per-Lens Accent (+)">
  <div class="row">
    <span class="lbl">Filename</span>
    <input type="color" aria-label="Filename accent"
      value={rgbToHex(settingsStore.state.fonts_and_colors.per_lens_accent.filename)}
      onchange={(e) => updateLensAccent("filename", (e.currentTarget as HTMLInputElement).value)} />
    <button type="button" onclick={() => updateLensAccent("filename", null)}>Default</button>
  </div>
  <div class="row">
    <span class="lbl">Content</span>
    <input type="color" aria-label="Content accent"
      value={rgbToHex(settingsStore.state.fonts_and_colors.per_lens_accent.content)}
      onchange={(e) => updateLensAccent("content", (e.currentTarget as HTMLInputElement).value)} />
    <button type="button" onclick={() => updateLensAccent("content", null)}>Default</button>
  </div>
  <div class="row">
    <span class="lbl">Audio</span>
    <input type="color" aria-label="Audio accent"
      value={rgbToHex(settingsStore.state.fonts_and_colors.per_lens_accent.audio)}
      onchange={(e) => updateLensAccent("audio", (e.currentTarget as HTMLInputElement).value)} />
    <button type="button" onclick={() => updateLensAccent("audio", null)}>Default</button>
  </div>
  <div class="row">
    <span class="lbl">Similarity</span>
    <input type="color" aria-label="Similarity accent"
      value={rgbToHex(settingsStore.state.fonts_and_colors.per_lens_accent.similarity)}
      onchange={(e) => updateLensAccent("similarity", (e.currentTarget as HTMLInputElement).value)} />
    <button type="button" onclick={() => updateLensAccent("similarity", null)}>Default</button>
  </div>
  <Checkbox id="fc-theme-inherit" label="Auto-flip custom colors on theme switch (preserve relative luminance)"
    checked={settingsStore.state.fonts_and_colors.theme_inheritance_toggle}
    onChange={(v) => updateRoot({ theme_inheritance_toggle: v })} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  .row { display: flex; align-items: center; gap: 12px; padding: 4px 0; color: var(--text-primary); font-size: 13px; }
  .lbl { flex: 1; }
  button { padding: 3px 8px; background: var(--bg-canvas); color: var(--text-primary); border: 1px solid var(--border); border-radius: 3px; cursor: pointer; font: inherit; }
  input[type="color"] { width: 40px; height: 24px; border: 1px solid var(--border); }
</style>
