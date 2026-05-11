<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import type { SettingsState } from "../../../lib/ipc/types";

  function patch<K extends keyof SettingsState>(key: K, value: SettingsState[K]) {
    settingsStore.patch({ [key]: value } as Partial<SettingsState>);
    settingsDialog.markDirty("general.view");
  }
</script>

<h1>View</h1>

<Section title="Rendering">
  <Checkbox id="vw-double" label="Double buffer (Windows-only legacy; macOS/Linux always double-buffered)"
    checked={settingsStore.state.double_buffer} onChange={(v) => patch("double_buffer", v)} />
  <Checkbox id="vw-alt-rows" label="Alternate row color"
    checked={settingsStore.state.alternate_row_color} onChange={(v) => patch("alternate_row_color", v)} />
  <Checkbox id="vw-row-mouse" label="Show row mouseover"
    checked={settingsStore.state.show_row_mouseover} onChange={(v) => patch("show_row_mouseover", v)} />
  <Checkbox id="vw-highlight" label="Show highlighted search terms"
    checked={settingsStore.state.show_highlighted_search_terms} onChange={(v) => patch("show_highlighted_search_terms", v)} />
  <Checkbox id="vw-tooltips" label="Show tooltips"
    checked={settingsStore.state.show_tooltips} onChange={(v) => patch("show_tooltips", v)} />
  <Checkbox id="vw-update-scroll" label="Update display immediately after scrolling"
    checked={settingsStore.state.update_display_immediately_after_scrolling} onChange={(v) => patch("update_display_immediately_after_scrolling", v)} />
</Section>

<Section title="Status Bar">
  <Checkbox id="vw-sel-status" label="Show selected item in status bar"
    checked={settingsStore.state.show_selected_item_in_status_bar} onChange={(v) => patch("show_selected_item_in_status_bar", v)} />
  <Checkbox id="vw-rc-sel" label="Show the result count with the selection count"
    checked={settingsStore.state.show_result_count_with_selection_count} onChange={(v) => patch("show_result_count_with_selection_count", v)} />
  <Checkbox id="vw-size-status" label="Show size in status bar"
    checked={settingsStore.state.show_size_in_status_bar} onChange={(v) => patch("show_size_in_status_bar", v)} />
</Section>

<Section title="Display Format">
  <Dropdown id="vw-size-fmt" label="Size format"
    value={settingsStore.state.size_format}
    options={[ { value: "b", label: "B" }, { value: "kb", label: "KB" }, { value: "mb", label: "MB" }, { value: "gb", label: "GB" }, { value: "auto_binary", label: "Auto (binary)" }, { value: "auto_decimal", label: "Auto (decimal)" } ]}
    onChange={(v) => patch("size_format", v)} />
  <Dropdown id="vw-sel-rect" label="Selection rectangle"
    value={settingsStore.state.selection_rectangle}
    options={[ { value: "system", label: "System default" }, { value: "drag_select", label: "Drag-select" }, { value: "none", label: "None" } ]}
    onChange={(v) => patch("selection_rectangle", v)} />
</Section>

<Section title="Sourcerer Additions (+)">
  <Checkbox id="vw-audio-badges" label="Show LUFS / codec / length badges on audio rows"
    checked={settingsStore.state.show_lufs_codec_length_badges} onChange={(v) => patch("show_lufs_codec_length_badges", v)} />
  <Checkbox id="vw-sim-score" label="Show MinHash similarity score on similarity rows"
    checked={settingsStore.state.show_minhash_similarity_score} onChange={(v) => patch("show_minhash_similarity_score", v)} />
  <Dropdown id="vw-preview-pane" label="Preview pane"
    value={settingsStore.state.preview_pane}
    options={[ { value: "right", label: "Right (default)" }, { value: "bottom", label: "Bottom" }, { value: "off", label: "Off" } ]}
    onChange={(v) => patch("preview_pane", v)} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
</style>
