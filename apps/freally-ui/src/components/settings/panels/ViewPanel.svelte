<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import { t } from "../../../lib/i18n/t";
  import type { SettingsState } from "../../../lib/ipc/types";

  function patch<K extends keyof SettingsState>(key: K, value: SettingsState[K]) {
    settingsStore.patch({ [key]: value } as Partial<SettingsState>);
    settingsDialog.markDirty("general.view");
  }
</script>

<h1>{t("settings-node-view")}</h1>

<Section title={t("section-rendering")}>
  <Checkbox id="vw-double" label={t("settings-view-double-buffer")}
    checked={settingsStore.state.double_buffer} onChange={(v) => patch("double_buffer", v)} />
  <Checkbox id="vw-alt-rows" label={t("settings-view-alt-rows")}
    checked={settingsStore.state.alternate_row_color} onChange={(v) => patch("alternate_row_color", v)} />
  <Checkbox id="vw-row-mouse" label={t("settings-view-row-mouseover")}
    checked={settingsStore.state.show_row_mouseover} onChange={(v) => patch("show_row_mouseover", v)} />
  <Checkbox id="vw-highlight" label={t("settings-view-highlight-terms")}
    checked={settingsStore.state.show_highlighted_search_terms} onChange={(v) => patch("show_highlighted_search_terms", v)} />
  <Checkbox id="vw-tooltips" label={t("settings-view-tooltips")}
    checked={settingsStore.state.show_tooltips} onChange={(v) => patch("show_tooltips", v)} />
  <Checkbox id="vw-update-scroll" label={t("settings-view-update-on-scroll")}
    checked={settingsStore.state.update_display_immediately_after_scrolling} onChange={(v) => patch("update_display_immediately_after_scrolling", v)} />
</Section>

<Section title={t("section-status-bar")}>
  <Checkbox id="vw-sel-status" label={t("settings-view-status-show-selected")}
    checked={settingsStore.state.show_selected_item_in_status_bar} onChange={(v) => patch("show_selected_item_in_status_bar", v)} />
  <Checkbox id="vw-rc-sel" label={t("settings-view-rc-with-sel")}
    checked={settingsStore.state.show_result_count_with_selection_count} onChange={(v) => patch("show_result_count_with_selection_count", v)} />
  <Checkbox id="vw-size-status" label={t("settings-view-status-show-size")}
    checked={settingsStore.state.show_size_in_status_bar} onChange={(v) => patch("show_size_in_status_bar", v)} />
</Section>

<Section title={t("section-display-format")}>
  <Dropdown id="vw-size-fmt" label={t("settings-view-size-format")}
    value={settingsStore.state.size_format}
    options={[ { value: "b", label: t("unit-b") }, { value: "kb", label: t("unit-kb") }, { value: "mb", label: t("unit-mb") }, { value: "gb", label: t("unit-gb") }, { value: "auto_binary", label: t("opt-auto-binary") }, { value: "auto_decimal", label: t("opt-auto-decimal") } ]}
    onChange={(v) => patch("size_format", v)} />
  <Dropdown id="vw-sel-rect" label={t("settings-view-selection-rect")}
    value={settingsStore.state.selection_rectangle}
    options={[ { value: "system", label: t("opt-system-default") }, { value: "drag_select", label: t("opt-drag-select") }, { value: "none", label: t("opt-none") } ]}
    onChange={(v) => patch("selection_rectangle", v)} />
</Section>

<Section title={t("section-freally-additions")}>
  <Checkbox id="vw-audio-badges" label={t("settings-view-audio-badges")}
    checked={settingsStore.state.show_lufs_codec_length_badges} onChange={(v) => patch("show_lufs_codec_length_badges", v)} />
  <Checkbox id="vw-sim-score" label={t("settings-view-similarity-score")}
    checked={settingsStore.state.show_minhash_similarity_score} onChange={(v) => patch("show_minhash_similarity_score", v)} />
  <Dropdown id="vw-preview-pane" label={t("settings-view-preview-pane")}
    value={settingsStore.state.preview_pane}
    options={[ { value: "right", label: t("opt-right-default") }, { value: "bottom", label: t("opt-bottom") }, { value: "off", label: t("opt-off") } ]}
    onChange={(v) => patch("preview_pane", v)} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
</style>
