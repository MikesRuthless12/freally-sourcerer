<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import type { SettingsState } from "../../../lib/ipc/types";

  function patch<K extends keyof SettingsState>(key: K, value: SettingsState[K]) {
    settingsStore.patch({ [key]: value } as Partial<SettingsState>);
    settingsDialog.markDirty("general.results");
  }
</script>

<h1>Results</h1>

<Section title="Behavior">
  <Checkbox id="rs-hide-empty" label="Hide results when the search is empty"
    checked={settingsStore.state.hide_results_when_empty} onChange={(v) => patch("hide_results_when_empty", v)} />
  <Checkbox id="rs-clear-sel" label="Clear selection on search"
    checked={settingsStore.state.clear_selection_on_search} onChange={(v) => patch("clear_selection_on_search", v)} />
  <Checkbox id="rs-close-on-exec" label="Close window on execute"
    checked={settingsStore.state.close_window_on_execute} onChange={(v) => patch("close_window_on_execute", v)} />
  <Checkbox id="rs-dbl-path" label="Open path with double click in path column"
    checked={settingsStore.state.open_path_with_double_click_in_path_column} onChange={(v) => patch("open_path_with_double_click_in_path_column", v)} />
  <Checkbox id="rs-auto-scroll" label="Automatically scroll view"
    checked={settingsStore.state.automatically_scroll_view} onChange={(v) => patch("automatically_scroll_view", v)} />
  <Checkbox id="rs-dquote" label="Double quote copy as path"
    checked={settingsStore.state.double_quote_copy_as_path} onChange={(v) => patch("double_quote_copy_as_path", v)} />
  <Checkbox id="rs-no-ext-rename" label="Do not select extension when renaming"
    checked={settingsStore.state.do_not_select_extension_when_renaming} onChange={(v) => patch("do_not_select_extension_when_renaming", v)} />
  <Checkbox id="rs-sort-date-desc" label="Sort date descending first"
    checked={settingsStore.state.sort_date_descending_first} onChange={(v) => patch("sort_date_descending_first", v)} />
  <Checkbox id="rs-sort-size-desc" label="Sort size descending first"
    checked={settingsStore.state.sort_size_descending_first} onChange={(v) => patch("sort_size_descending_first", v)} />
  <Dropdown id="rs-focus" label="Result list focus"
    value={settingsStore.state.result_list_focus}
    options={[ { value: "clamp", label: "Clamp (default)" }, { value: "wrap", label: "Wrap" }, { value: "none", label: "None" } ]}
    onChange={(v) => patch("result_list_focus", v)} />
</Section>

<Section title="Loading Priority">
  <Dropdown id="rs-icon-prio" label="Load icon priority"
    value={settingsStore.state.load_icon_priority}
    options={[ { value: "high", label: "High" }, { value: "normal", label: "Normal (default)" }, { value: "low", label: "Low" }, { value: "disabled", label: "Disabled" } ]}
    onChange={(v) => patch("load_icon_priority", v)} />
  <Dropdown id="rs-thumb-prio" label="Load thumbnail priority"
    value={settingsStore.state.load_thumbnail_priority}
    options={[ { value: "high", label: "High" }, { value: "normal", label: "Normal (default)" }, { value: "low", label: "Low" }, { value: "disabled", label: "Disabled" } ]}
    onChange={(v) => patch("load_thumbnail_priority", v)} />
  <Dropdown id="rs-ext-prio" label="Load extended information priority"
    value={settingsStore.state.load_extended_information_priority}
    options={[ { value: "high", label: "High" }, { value: "normal", label: "Normal (default)" }, { value: "low", label: "Low" }, { value: "disabled", label: "Disabled" } ]}
    onChange={(v) => patch("load_extended_information_priority", v)} />
</Section>

<Section title="Sourcerer Extras (+)">
  <Checkbox id="rs-group-by-lens" label="Group results by lens"
    checked={settingsStore.state.group_by_lens} onChange={(v) => patch("group_by_lens", v)} />
  <Checkbox id="rs-snippet" label="Show snippet preview inline"
    checked={settingsStore.state.show_snippet_preview_inline} onChange={(v) => patch("show_snippet_preview_inline", v)} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
</style>
