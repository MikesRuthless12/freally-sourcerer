<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import type { SettingsState, LensId } from "../../../lib/ipc/types";

  function patch<K extends keyof SettingsState>(key: K, value: SettingsState[K]) {
    settingsStore.patch({ [key]: value } as Partial<SettingsState>);
    settingsDialog.markDirty("general.home");
  }

  const LAST_OPTIONS = [
    { value: "use_last", label: "Use last value (default)" },
    { value: "on", label: "On" },
    { value: "off", label: "Off" }
  ] as const;

  const FILTER_OPTIONS = [
    { value: "use_last", label: "Use last value" },
    { value: "all", label: "All" },
    { value: "audio", label: "Audio" },
    { value: "video", label: "Video" },
    { value: "image", label: "Image" },
    { value: "document", label: "Document" },
    { value: "executable", label: "Executable" },
    { value: "archive", label: "Archive" }
  ] as const;

  const SORT_OPTIONS = [
    { value: "use_last", label: "Use last value" },
    { value: "name_asc", label: "Name asc" },
    { value: "name_desc", label: "Name desc" },
    { value: "path", label: "Path" },
    { value: "size_asc", label: "Size asc" },
    { value: "size_desc", label: "Size desc" },
    { value: "modified_asc", label: "Date modified asc" },
    { value: "modified_desc", label: "Date modified desc" }
  ] as const;

  const VIEW_OPTIONS = [
    { value: "use_last", label: "Use last value" },
    { value: "compact", label: "Compact" },
    { value: "comfortable", label: "Comfortable" },
    { value: "details", label: "Details" },
    { value: "thumbnails", label: "Thumbnails" }
  ] as const;

  const INDEX_OPTIONS = [
    { value: "local", label: "Local database (default)" },
    { value: "file_list", label: "File list" },
    { value: "https_endpoint", label: "HTTPS API endpoint" }
  ] as const;

  function setLens(lens: LensId, on: boolean) {
    patch("default_lens_visibility", { ...settingsStore.state.default_lens_visibility, [lens]: on });
  }
  function setLensLimit(lens: LensId, n: number) {
    patch("default_lens_result_limits", {
      ...settingsStore.state.default_lens_result_limits,
      [lens]: n
    });
  }
</script>

<h1>Home</h1>
<p class="hint">Defaults loaded on app launch — every dropdown can stick to "Use last value"
or pin a fixed value (E). Lens visibility / result limits are Sourcerer additions (+).</p>

<Section title="Match Defaults">
  <Dropdown id="hm-match-case" label="Match case" value={settingsStore.state.default_match_case}
    options={Array.from(LAST_OPTIONS)} onChange={(v) => patch("default_match_case", v)} />
  <Dropdown id="hm-match-whole" label="Match whole word" value={settingsStore.state.default_match_whole_word}
    options={Array.from(LAST_OPTIONS)} onChange={(v) => patch("default_match_whole_word", v)} />
  <Dropdown id="hm-match-path" label="Match path" value={settingsStore.state.default_match_path}
    options={Array.from(LAST_OPTIONS)} onChange={(v) => patch("default_match_path", v)} />
  <Dropdown id="hm-match-diac" label="Match diacritics" value={settingsStore.state.default_match_diacritics}
    options={Array.from(LAST_OPTIONS)} onChange={(v) => patch("default_match_diacritics", v)} />
  <Dropdown id="hm-match-rgx" label="Match regex" value={settingsStore.state.default_match_regex}
    options={Array.from(LAST_OPTIONS)} onChange={(v) => patch("default_match_regex", v)} />
</Section>

<Section title="Search & Sort Defaults">
  <TextInput id="hm-search" label="Search (custom default query)" value={settingsStore.state.default_search}
    placeholder="Empty by default" onChange={(v) => patch("default_search", v)} />
  <Dropdown id="hm-filter" label="Filter" value={settingsStore.state.default_filter}
    options={Array.from(FILTER_OPTIONS)} onChange={(v) => patch("default_filter", v)} />
  <Dropdown id="hm-sort" label="Sort" value={settingsStore.state.default_sort}
    options={Array.from(SORT_OPTIONS)} onChange={(v) => patch("default_sort", v)} />
  <Dropdown id="hm-view" label="View" value={settingsStore.state.default_view}
    options={Array.from(VIEW_OPTIONS)} onChange={(v) => patch("default_view", v)} />
</Section>

<Section title="Index Source">
  <Dropdown id="hm-index" label="Index" value={settingsStore.state.default_index}
    options={Array.from(INDEX_OPTIONS)} onChange={(v) => patch("default_index", v)} />
  <TextInput id="hm-file-list" label="File list path"
    value={settingsStore.state.default_file_list}
    onChange={(v) => patch("default_file_list", v)} />
  <TextInput id="hm-endpoint-url" label="HTTPS API endpoint URL"
    value={settingsStore.state.default_https_endpoint.url}
    onChange={(v) => patch("default_https_endpoint", { ...settingsStore.state.default_https_endpoint, url: v })} />
  <TextInput id="hm-endpoint-token" label="Token (fingerprint shown)"
    value={settingsStore.state.default_https_endpoint.token_fingerprint}
    onChange={(v) => patch("default_https_endpoint", { ...settingsStore.state.default_https_endpoint, token_fingerprint: v })} />
</Section>

<Section title="Default Lens Visibility (+)">
  <Checkbox id="hm-lens-fn" label="Filename" checked={settingsStore.state.default_lens_visibility.filename} onChange={(v) => setLens("filename" as LensId, v)} />
  <Checkbox id="hm-lens-ct" label="Content"  checked={settingsStore.state.default_lens_visibility.content}  onChange={(v) => setLens("content" as LensId, v)} />
  <Checkbox id="hm-lens-au" label="Audio"    checked={settingsStore.state.default_lens_visibility.audio}    onChange={(v) => setLens("audio" as LensId, v)} />
  <Checkbox id="hm-lens-sm" label="Similarity" checked={settingsStore.state.default_lens_visibility.similarity} onChange={(v) => setLens("similarity" as LensId, v)} />
</Section>

<Section title="Default Lens Result Limits (+)">
  <NumberInput id="hm-lim-fn" label="Filename" min={1} max={10000} value={settingsStore.state.default_lens_result_limits.filename} onChange={(n) => setLensLimit("filename" as LensId, n)} />
  <NumberInput id="hm-lim-ct" label="Content"  min={1} max={10000} value={settingsStore.state.default_lens_result_limits.content}  onChange={(n) => setLensLimit("content" as LensId, n)} />
  <NumberInput id="hm-lim-au" label="Audio"    min={1} max={10000} value={settingsStore.state.default_lens_result_limits.audio}    onChange={(n) => setLensLimit("audio" as LensId, n)} />
  <NumberInput id="hm-lim-sm" label="Similarity" min={1} max={10000} value={settingsStore.state.default_lens_result_limits.similarity} onChange={(n) => setLensLimit("similarity" as LensId, n)} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  .hint { margin: 0 0 16px; color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
</style>
