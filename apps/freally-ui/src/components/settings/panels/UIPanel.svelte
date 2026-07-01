<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import { themeStore, type ThemeChoice } from "../../../lib/stores/theme.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import { t } from "../../../lib/i18n/t";
  import type { SettingsState } from "../../../lib/ipc/types";

  function patch<K extends keyof SettingsState>(key: K, value: SettingsState[K]) {
    settingsStore.patch({ [key]: value } as Partial<SettingsState>);
    settingsDialog.markDirty("general.ui");
    // Live-apply the theme switcher so the dialog (and the rest of the
    // app) re-skins immediately — no restart needed.
    if (key === "theme") {
      themeStore.set(value as ThemeChoice);
    }
  }
</script>

<h1>{t("settings-node-ui")}</h1>
<p class="hint">{t("ui-hint")}</p>

<Section title={t("ui-section-theme")}>
  <Dropdown
    id="ui-theme"
    label={t("settings-ui-theme")}
    value={settingsStore.state.theme}
    options={[
      { value: "system", label: t("ui-theme-system-default") },
      { value: "light", label: t("theme-light") },
      { value: "dark", label: t("theme-dark") }
    ]}
    onChange={(v) => patch("theme", v)}
  />
  <Checkbox
    id="ui-anim-crossfade"
    label={t("settings-ui-anim-crossfade")}
    checked={settingsStore.state.animated_theme_crossfade}
    onChange={(v) => patch("animated_theme_crossfade", v)}
  />
</Section>

<Section title={t("ui-section-tray")}>
  <Checkbox
    id="ui-run-bg"
    label={t("settings-ui-run-bg")}
    checked={settingsStore.state.run_in_background}
    onChange={(v) => patch("run_in_background", v)}
  />
  <Checkbox
    id="ui-show-tray"
    label={t("settings-ui-show-tray")}
    checked={settingsStore.state.show_tray_icon}
    onChange={(v) => patch("show_tray_icon", v)}
  />
  <Checkbox
    id="ui-single-click-tray"
    label={t("settings-ui-single-click-tray")}
    checked={settingsStore.state.single_click_tray}
    onChange={(v) => patch("single_click_tray", v)}
  />
  <Checkbox
    id="ui-new-window-from-tray"
    label={t("settings-ui-new-window-from-tray")}
    checked={settingsStore.state.open_new_window_from_tray}
    onChange={(v) => patch("open_new_window_from_tray", v)}
  />
  <Checkbox
    id="ui-new-window-on-launch"
    label={t("settings-ui-new-window-on-launch")}
    checked={settingsStore.state.open_new_window_when_launching}
    onChange={(v) => patch("open_new_window_when_launching", v)}
  />
</Section>

<Section title={t("ui-section-search-behavior")}>
  <Checkbox
    id="ui-search-as-you-type"
    label={t("settings-ui-search-as-you-type")}
    checked={settingsStore.state.search_as_you_type}
    onChange={(v) => patch("search_as_you_type", v)}
  />
  <Checkbox
    id="ui-select-on-mouse-click"
    label={t("settings-ui-select-on-mouse-click")}
    checked={settingsStore.state.select_search_on_mouse_click}
    onChange={(v) => patch("select_search_on_mouse_click", v)}
  />
  <Checkbox
    id="ui-focus-on-activate"
    label={t("settings-ui-focus-on-activate")}
    checked={settingsStore.state.focus_search_on_activate}
    onChange={(v) => patch("focus_search_on_activate", v)}
  />
</Section>

<Section title={t("ui-section-result-rows")}>
  <Checkbox
    id="ui-full-row-select"
    label={t("settings-ui-full-row-select")}
    checked={settingsStore.state.full_row_select}
    onChange={(v) => patch("full_row_select", v)}
  />
  <Dropdown
    id="ui-single-click-open"
    label={t("settings-ui-single-click-open")}
    value={settingsStore.state.single_click_open}
    options={[
      { value: "system_settings", label: t("ui-single-click-system-default") },
      { value: "always_single", label: t("ui-single-click-always") },
      { value: "always_double", label: t("ui-single-click-always-double") }
    ]}
    onChange={(v) => patch("single_click_open", v)}
  />
  <Dropdown
    id="ui-underline-titles"
    label={t("settings-ui-underline-titles")}
    value={settingsStore.state.underline_icon_titles}
    options={[
      { value: "system_settings", label: t("ui-single-click-system-default") },
      { value: "always", label: t("ui-underline-always") },
      { value: "on_hover", label: t("ui-underline-on-hover") },
      { value: "never", label: t("ui-underline-never") }
    ]}
    onChange={(v) => patch("underline_icon_titles", v)}
  />
  <Dropdown
    id="ui-row-density"
    label={t("settings-ui-row-density")}
    value={settingsStore.state.row_density}
    options={[
      { value: "compact", label: t("settings-ui-row-density-compact") },
      { value: "comfortable", label: t("settings-ui-row-density-comfortable") }
    ]}
    onChange={(v) => patch("row_density", v)}
  />
  <Checkbox
    id="ui-timing-badges"
    label={t("settings-ui-show-timing-badges")}
    checked={settingsStore.state.show_timing_badges}
    onChange={(v) => patch("show_timing_badges", v)}
  />
</Section>

<style>
  h1 {
    margin: 0 0 4px;
    font-size: 18px;
    color: var(--text-primary);
  }
  .hint {
    margin: 0 0 16px;
    color: var(--text-secondary);
    font-size: 12px;
    line-height: 1.5;
  }
</style>
