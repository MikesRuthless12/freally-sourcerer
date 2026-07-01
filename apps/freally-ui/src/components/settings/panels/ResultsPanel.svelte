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
    settingsDialog.markDirty("general.results");
  }
</script>

<h1>{t("settings-node-results")}</h1>

<Section title={t("section-behavior")}>
  <Checkbox id="rs-hide-empty" label={t("settings-results-hide-empty")}
    checked={settingsStore.state.hide_results_when_empty} onChange={(v) => patch("hide_results_when_empty", v)} />
  <Checkbox id="rs-clear-sel" label={t("settings-results-clear-on-search")}
    checked={settingsStore.state.clear_selection_on_search} onChange={(v) => patch("clear_selection_on_search", v)} />
  <Checkbox id="rs-close-on-exec" label={t("settings-results-close-on-execute")}
    checked={settingsStore.state.close_window_on_execute} onChange={(v) => patch("close_window_on_execute", v)} />
  <Checkbox id="rs-dbl-path" label={t("settings-results-dbl-path")}
    checked={settingsStore.state.open_path_with_double_click_in_path_column} onChange={(v) => patch("open_path_with_double_click_in_path_column", v)} />
  <Checkbox id="rs-auto-scroll" label={t("settings-results-auto-scroll")}
    checked={settingsStore.state.automatically_scroll_view} onChange={(v) => patch("automatically_scroll_view", v)} />
  <Checkbox id="rs-dquote" label={t("settings-results-dquote-copy")}
    checked={settingsStore.state.double_quote_copy_as_path} onChange={(v) => patch("double_quote_copy_as_path", v)} />
  <Checkbox id="rs-no-ext-rename" label={t("settings-results-no-ext-rename")}
    checked={settingsStore.state.do_not_select_extension_when_renaming} onChange={(v) => patch("do_not_select_extension_when_renaming", v)} />
  <Checkbox id="rs-sort-date-desc" label={t("settings-results-sort-date-desc")}
    checked={settingsStore.state.sort_date_descending_first} onChange={(v) => patch("sort_date_descending_first", v)} />
  <Checkbox id="rs-sort-size-desc" label={t("settings-results-sort-size-desc")}
    checked={settingsStore.state.sort_size_descending_first} onChange={(v) => patch("sort_size_descending_first", v)} />
  <Dropdown id="rs-focus" label={t("settings-results-list-focus")}
    value={settingsStore.state.result_list_focus}
    options={[ { value: "clamp", label: t("opt-clamp-default") }, { value: "wrap", label: t("opt-wrap") }, { value: "none", label: t("opt-none") } ]}
    onChange={(v) => patch("result_list_focus", v)} />
</Section>

<Section title={t("section-loading-priority")}>
  <Dropdown id="rs-icon-prio" label={t("settings-results-icon-prio")}
    value={settingsStore.state.load_icon_priority}
    options={[ { value: "high", label: t("opt-high") }, { value: "normal", label: t("opt-normal-default") }, { value: "low", label: t("opt-low") }, { value: "disabled", label: t("opt-disabled") } ]}
    onChange={(v) => patch("load_icon_priority", v)} />
  <Dropdown id="rs-thumb-prio" label={t("settings-results-thumb-prio")}
    value={settingsStore.state.load_thumbnail_priority}
    options={[ { value: "high", label: t("opt-high") }, { value: "normal", label: t("opt-normal-default") }, { value: "low", label: t("opt-low") }, { value: "disabled", label: t("opt-disabled") } ]}
    onChange={(v) => patch("load_thumbnail_priority", v)} />
  <Dropdown id="rs-ext-prio" label={t("settings-results-ext-prio")}
    value={settingsStore.state.load_extended_information_priority}
    options={[ { value: "high", label: t("opt-high") }, { value: "normal", label: t("opt-normal-default") }, { value: "low", label: t("opt-low") }, { value: "disabled", label: t("opt-disabled") } ]}
    onChange={(v) => patch("load_extended_information_priority", v)} />
</Section>

<Section title={t("section-freally-extras")}>
  <Checkbox id="rs-group-by-lens" label={t("settings-results-group-by-lens")}
    checked={settingsStore.state.group_by_lens} onChange={(v) => patch("group_by_lens", v)} />
  <Checkbox id="rs-snippet" label={t("settings-results-snippet-inline")}
    checked={settingsStore.state.show_snippet_preview_inline} onChange={(v) => patch("show_snippet_preview_inline", v)} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
</style>
