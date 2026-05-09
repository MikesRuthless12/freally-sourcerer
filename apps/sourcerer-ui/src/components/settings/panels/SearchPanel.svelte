<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import type { SettingsState } from "../../../lib/ipc/types";

  function patch<K extends keyof SettingsState>(key: K, value: SettingsState[K]) {
    settingsStore.patch({ [key]: value } as Partial<SettingsState>);
    settingsDialog.markDirty("general.search");
  }
</script>

<h1>Search</h1>
<p class="hint">DSL behavior — voidtools-Everything compatibility on top, Sourcerer extensions
(strict-Everything mode, auto-regex, modifier completions) below.</p>

<Section title="Compatibility (E)">
  <Checkbox id="se-fast-ascii" label="Fast ASCII search"
    checked={settingsStore.state.fast_ascii_search} onChange={(v) => patch("fast_ascii_search", v)} />
  <Checkbox id="se-mp-sep" label="Match path when a search term contains a path separator"
    checked={settingsStore.state.match_path_when_term_contains_separator} onChange={(v) => patch("match_path_when_term_contains_separator", v)} />
  <Checkbox id="se-mw-fn" label="Match whole filename when using wildcards"
    checked={settingsStore.state.match_whole_filename_with_wildcards} onChange={(v) => patch("match_whole_filename_with_wildcards", v)} />
  <Checkbox id="se-lit-ops" label="Allow literal operators (when off, AND / OR are matched literally)"
    checked={settingsStore.state.allow_literal_operators} onChange={(v) => patch("allow_literal_operators", v)} />
  <Checkbox id="se-paren" label="Allow round bracket grouping"
    checked={settingsStore.state.allow_round_bracket_grouping} onChange={(v) => patch("allow_round_bracket_grouping", v)} />
  <Checkbox id="se-env" label="Expand environment variables"
    checked={settingsStore.state.expand_environment_variables} onChange={(v) => patch("expand_environment_variables", v)} />
  <Checkbox id="se-fwd" label="Replace forward slashes with backslashes (Windows utility)"
    checked={settingsStore.state.replace_forward_with_backslashes} onChange={(v) => patch("replace_forward_with_backslashes", v)} />
  <Dropdown id="se-prec" label="Operator precedence"
    value={settingsStore.state.operator_precedence}
    options={[ { value: "or_first", label: "OR > AND (default)" }, { value: "and_first", label: "AND > OR" } ]}
    onChange={(v) => patch("operator_precedence", v)} />
</Section>

<Section title="Sourcerer Extensions (+)">
  <Checkbox id="se-strict" label="Strict Everything syntax mode (reject Sourcerer-only modifiers)"
    checked={settingsStore.state.strict_everything_mode} onChange={(v) => patch("strict_everything_mode", v)} />
  <Checkbox id="se-auto-rgx" label="Auto-detect regex (^…$ patterns route to regex automatically)"
    checked={settingsStore.state.auto_detect_regex} onChange={(v) => patch("auto_detect_regex", v)} />
  <Checkbox id="se-mod-comp" label="Modifier completions (lufs: / size: / codec: hint chips)"
    checked={settingsStore.state.modifier_completions} onChange={(v) => patch("modifier_completions", v)} />
  <Checkbox id="se-parse-tree" label="Show parse-tree on hover"
    checked={settingsStore.state.show_parse_tree_on_hover} onChange={(v) => patch("show_parse_tree_on_hover", v)} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  .hint { margin: 0 0 16px; color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
</style>
