<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import { t } from "../../../lib/i18n/t";
  import type { SettingsState, LensId } from "../../../lib/ipc/types";

  function patch<K extends keyof SettingsState>(key: K, value: SettingsState[K]) {
    settingsStore.patch({ [key]: value } as Partial<SettingsState>);
    settingsDialog.markDirty("general.home");
  }

  const LAST_OPTIONS = [
    { value: "use_last", label: t("opt-use-last-value-default") },
    { value: "on", label: t("opt-on") },
    { value: "off", label: t("opt-off") }
  ] as const;

  const FILTER_OPTIONS = [
    { value: "use_last", label: t("opt-use-last-value") },
    { value: "all", label: t("opt-all") },
    { value: "audio", label: t("quick-filter-audio") },
    { value: "video", label: t("quick-filter-video") },
    { value: "image", label: t("quick-filter-image") },
    { value: "document", label: t("quick-filter-document") },
    { value: "executable", label: t("quick-filter-executable") },
    { value: "archive", label: t("quick-filter-archive") }
  ] as const;

  const SORT_OPTIONS = [
    { value: "use_last", label: t("opt-use-last-value") },
    { value: "name_asc", label: t("opt-name-asc") },
    { value: "name_desc", label: t("opt-name-desc") },
    { value: "path", label: t("column-path") },
    { value: "size_asc", label: t("opt-size-asc") },
    { value: "size_desc", label: t("opt-size-desc") },
    { value: "modified_asc", label: t("opt-modified-asc") },
    { value: "modified_desc", label: t("opt-modified-desc") }
  ] as const;

  const VIEW_OPTIONS = [
    { value: "use_last", label: t("opt-use-last-value") },
    { value: "compact", label: t("opt-compact") },
    { value: "comfortable", label: t("opt-comfortable") },
    { value: "details", label: t("opt-details") },
    { value: "thumbnails", label: t("opt-thumbnails") }
  ] as const;

  const INDEX_OPTIONS = [
    { value: "local", label: t("opt-local-db-default") },
    { value: "file_list", label: t("opt-file-list") },
    { value: "https_endpoint", label: t("opt-https-endpoint") }
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

<h1>{t("settings-node-home")}</h1>
<p class="hint">{t("home-hint")}</p>

<Section title={t("home-section-match")}>
  <Dropdown id="hm-match-case" label={t("settings-home-match-case")} value={settingsStore.state.default_match_case}
    options={Array.from(LAST_OPTIONS)} onChange={(v) => patch("default_match_case", v)} />
  <Dropdown id="hm-match-whole" label={t("settings-home-match-whole-word")} value={settingsStore.state.default_match_whole_word}
    options={Array.from(LAST_OPTIONS)} onChange={(v) => patch("default_match_whole_word", v)} />
  <Dropdown id="hm-match-path" label={t("settings-home-match-path")} value={settingsStore.state.default_match_path}
    options={Array.from(LAST_OPTIONS)} onChange={(v) => patch("default_match_path", v)} />
  <Dropdown id="hm-match-diac" label={t("settings-home-match-diacritics")} value={settingsStore.state.default_match_diacritics}
    options={Array.from(LAST_OPTIONS)} onChange={(v) => patch("default_match_diacritics", v)} />
  <Dropdown id="hm-match-rgx" label={t("settings-home-match-regex")} value={settingsStore.state.default_match_regex}
    options={Array.from(LAST_OPTIONS)} onChange={(v) => patch("default_match_regex", v)} />
</Section>

<Section title={t("home-section-search-sort")}>
  <TextInput id="hm-search" label={t("settings-home-search")} value={settingsStore.state.default_search}
    placeholder={t("home-search-placeholder")} onChange={(v) => patch("default_search", v)} />
  <Dropdown id="hm-filter" label={t("settings-home-filter")} value={settingsStore.state.default_filter}
    options={Array.from(FILTER_OPTIONS)} onChange={(v) => patch("default_filter", v)} />
  <Dropdown id="hm-sort" label={t("settings-home-sort")} value={settingsStore.state.default_sort}
    options={Array.from(SORT_OPTIONS)} onChange={(v) => patch("default_sort", v)} />
  <Dropdown id="hm-view" label={t("settings-home-view")} value={settingsStore.state.default_view}
    options={Array.from(VIEW_OPTIONS)} onChange={(v) => patch("default_view", v)} />
</Section>

<Section title={t("home-section-index")}>
  <Dropdown id="hm-index" label={t("settings-home-index")} value={settingsStore.state.default_index}
    options={Array.from(INDEX_OPTIONS)} onChange={(v) => patch("default_index", v)} />
  <TextInput id="hm-file-list" label={t("home-file-list-path")}
    value={settingsStore.state.default_file_list}
    onChange={(v) => patch("default_file_list", v)} />
  <TextInput id="hm-endpoint-url" label={t("home-https-endpoint")}
    value={settingsStore.state.default_https_endpoint.url}
    onChange={(v) => patch("default_https_endpoint", { ...settingsStore.state.default_https_endpoint, url: v })} />
  <TextInput id="hm-endpoint-token" label={t("home-endpoint-token")}
    value={settingsStore.state.default_https_endpoint.token_fingerprint}
    onChange={(v) => patch("default_https_endpoint", { ...settingsStore.state.default_https_endpoint, token_fingerprint: v })} />
</Section>

<Section title={t("settings-home-default-lens-visibility")}>
  <Checkbox id="hm-lens-fn" label={t("lens-filename")} checked={settingsStore.state.default_lens_visibility.filename} onChange={(v) => setLens("filename" as LensId, v)} />
  <Checkbox id="hm-lens-ct" label={t("lens-content")}  checked={settingsStore.state.default_lens_visibility.content}  onChange={(v) => setLens("content" as LensId, v)} />
  <Checkbox id="hm-lens-au" label={t("lens-audio")}    checked={settingsStore.state.default_lens_visibility.audio}    onChange={(v) => setLens("audio" as LensId, v)} />
  <Checkbox id="hm-lens-sm" label={t("lens-similarity")} checked={settingsStore.state.default_lens_visibility.similarity} onChange={(v) => setLens("similarity" as LensId, v)} />
</Section>

<Section title={t("settings-home-default-lens-result-limits")}>
  <NumberInput id="hm-lim-fn" label={t("lens-filename")} min={1} max={10000} value={settingsStore.state.default_lens_result_limits.filename} onChange={(n) => setLensLimit("filename" as LensId, n)} />
  <NumberInput id="hm-lim-ct" label={t("lens-content")}  min={1} max={10000} value={settingsStore.state.default_lens_result_limits.content}  onChange={(n) => setLensLimit("content" as LensId, n)} />
  <NumberInput id="hm-lim-au" label={t("lens-audio")}    min={1} max={10000} value={settingsStore.state.default_lens_result_limits.audio}    onChange={(n) => setLensLimit("audio" as LensId, n)} />
  <NumberInput id="hm-lim-sm" label={t("lens-similarity")} min={1} max={10000} value={settingsStore.state.default_lens_result_limits.similarity} onChange={(n) => setLensLimit("similarity" as LensId, n)} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  .hint { margin: 0 0 16px; color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
</style>
