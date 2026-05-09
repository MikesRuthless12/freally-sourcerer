<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import type { SettingsState } from "../../../lib/ipc/types";

  function patch<K extends keyof SettingsState>(key: K, value: SettingsState[K]) {
    settingsStore.patch({ [key]: value } as Partial<SettingsState>);
    settingsDialog.markDirty("general.ui");
  }
</script>

<h1>UI</h1>
<p class="hint">Theme, tray / menu-bar integration, search-as-you-type, row density. Direct
voidtools-Everything parity plus Sourcerer additions marked with (+).</p>

<Section title="Theme">
  <Dropdown
    id="ui-theme"
    label="Theme"
    value={settingsStore.state.theme}
    options={[
      { value: "system", label: "System (default)" },
      { value: "light", label: "Light" },
      { value: "dark", label: "Dark" }
    ]}
    onChange={(v) => patch("theme", v)}
  />
  <Checkbox
    id="ui-anim-crossfade"
    label="Animated theme cross-fade (+)"
    checked={settingsStore.state.animated_theme_crossfade}
    onChange={(v) => patch("animated_theme_crossfade", v)}
  />
</Section>

<Section title="Tray / Menu Bar">
  <Checkbox
    id="ui-run-bg"
    label="Run in background (E)"
    checked={settingsStore.state.run_in_background}
    onChange={(v) => patch("run_in_background", v)}
  />
  <Checkbox
    id="ui-show-tray"
    label="Show tray / menu-bar icon (E)"
    checked={settingsStore.state.show_tray_icon}
    onChange={(v) => patch("show_tray_icon", v)}
  />
  <Checkbox
    id="ui-single-click-tray"
    label="Single click tray / menu bar (E)"
    checked={settingsStore.state.single_click_tray}
    onChange={(v) => patch("single_click_tray", v)}
  />
  <Checkbox
    id="ui-new-window-from-tray"
    label="Open new window from tray icon (E)"
    checked={settingsStore.state.open_new_window_from_tray}
    onChange={(v) => patch("open_new_window_from_tray", v)}
  />
  <Checkbox
    id="ui-new-window-on-launch"
    label="Open new window when launching Sourcerer (E)"
    checked={settingsStore.state.open_new_window_when_launching}
    onChange={(v) => patch("open_new_window_when_launching", v)}
  />
</Section>

<Section title="Search Behavior">
  <Checkbox
    id="ui-search-as-you-type"
    label="Search as you type (E)"
    checked={settingsStore.state.search_as_you_type}
    onChange={(v) => patch("search_as_you_type", v)}
  />
  <Checkbox
    id="ui-select-on-mouse-click"
    label="Select search on mouse click (E)"
    checked={settingsStore.state.select_search_on_mouse_click}
    onChange={(v) => patch("select_search_on_mouse_click", v)}
  />
  <Checkbox
    id="ui-focus-on-activate"
    label="Focus search on activate (E)"
    checked={settingsStore.state.focus_search_on_activate}
    onChange={(v) => patch("focus_search_on_activate", v)}
  />
</Section>

<Section title="Result Rows">
  <Checkbox
    id="ui-full-row-select"
    label="Full row select (E)"
    checked={settingsStore.state.full_row_select}
    onChange={(v) => patch("full_row_select", v)}
  />
  <Dropdown
    id="ui-single-click-open"
    label="Single click open (E)"
    value={settingsStore.state.single_click_open}
    options={[
      { value: "system_settings", label: "System settings (default)" },
      { value: "always_single", label: "Always single click" },
      { value: "always_double", label: "Always double click" }
    ]}
    onChange={(v) => patch("single_click_open", v)}
  />
  <Dropdown
    id="ui-underline-titles"
    label="Underline icon titles (E)"
    value={settingsStore.state.underline_icon_titles}
    options={[
      { value: "system_settings", label: "System settings (default)" },
      { value: "always", label: "Always" },
      { value: "on_hover", label: "On hover" },
      { value: "never", label: "Never" }
    ]}
    onChange={(v) => patch("underline_icon_titles", v)}
  />
  <Dropdown
    id="ui-row-density"
    label="Result density (+)"
    value={settingsStore.state.row_density}
    options={[
      { value: "compact", label: "Compact (32 px)" },
      { value: "comfortable", label: "Comfortable (44 px)" }
    ]}
    onChange={(v) => patch("row_density", v)}
  />
  <Checkbox
    id="ui-timing-badges"
    label="Show timing badges per lens (+)"
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
