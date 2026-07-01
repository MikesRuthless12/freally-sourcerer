<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import { t } from "../../../lib/i18n/t";
  import { LOCALES } from "../../../lib/i18n/bundle";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import type { ContentLensSettings } from "../../../lib/ipc/types";

  // Stop-words language list is derived from the 18 ship-locales so the
  // entries are shown in their native scripts (same convention as the
  // LocalePanel current-locale dropdown). The leading "auto" option is
  // a per-document detect; it's not a locale.
  const STOP_WORDS_OPTIONS = $derived([
    { value: "auto", label: t("opt-auto-per-doc") },
    ...LOCALES.map((l) => ({ value: l.value, label: l.label }))
  ]);

  const FORMATS = [
    "plain_text",
    "markdown",
    "pdf",
    "docx",
    "xlsx",
    "pptx",
    "code",
    "archive",
    "json",
    "csv",
    "yaml"
  ];

  function patch(p: Partial<ContentLensSettings>) {
    settingsStore.patch({ lens_content: { ...settingsStore.state.lens_content, ...p } });
    settingsDialog.markDirty("lenses.content");
  }

  function setFormatMode(fmt: string, mode: "eager" | "lazy" | "disabled") {
    patch({ per_format: { ...settingsStore.state.lens_content.per_format, [fmt]: mode } });
  }
  function effectiveMode(fmt: string): "eager" | "lazy" | "disabled" {
    return settingsStore.state.lens_content.per_format[fmt] ?? "lazy";
  }
</script>

<h1>{t("lens-content")}</h1>

<Section title={t("section-lens")}>
  <Checkbox id="lc-en" label={t("settings-lc-enable")}
    checked={settingsStore.state.lens_content.enabled}
    onChange={(v) => patch({ enabled: v })} />
</Section>

<Section title={t("section-per-format-mode")}>
  {#each FORMATS as fmt (fmt)}
    <Dropdown id={`lc-fmt-${fmt}`} label={fmt}
      value={effectiveMode(fmt)}
      options={[ { value: "eager", label: t("opt-eager") }, { value: "lazy", label: t("opt-lazy-default") }, { value: "disabled", label: t("opt-disabled") } ]}
      onChange={(v) => setFormatMode(fmt, v)} />
  {/each}
</Section>

<Section title={t("section-budgets")}>
  <NumberInput id="lc-time" label={t("settings-lc-time-budget")} min={50} max={60000} suffix="ms"
    value={settingsStore.state.lens_content.time_budget_ms}
    onChange={(n) => patch({ time_budget_ms: n })} />
  <NumberInput id="lc-mem" label={t("settings-lc-mem-ceiling")} min={16} max={4096} suffix={t("unit-mb")}
    value={settingsStore.state.lens_content.memory_ceiling_mb}
    onChange={(n) => patch({ memory_ceiling_mb: n })} />
  <NumberInput id="lc-snip" label={t("settings-lc-snippet-len")} min={50} max={2000} suffix="chars"
    value={settingsStore.state.lens_content.snippet_length}
    onChange={(n) => patch({ snippet_length: n })} />
</Section>

<Section title={t("section-other")}>
  <Dropdown id="lc-stop" label={t("settings-lc-stop-words")}
    value={settingsStore.state.lens_content.stop_words_language}
    options={STOP_WORDS_OPTIONS}
    onChange={(v) => patch({ stop_words_language: v })} />
  <Checkbox id="lc-re-ext" label={t("settings-lc-re-extract")}
    checked={settingsStore.state.lens_content.re_extract_on_settings_change}
    onChange={(v) => patch({ re_extract_on_settings_change: v })} />
  <Checkbox id="lc-verify" label={t("settings-lc-verify-blobs")}
    checked={settingsStore.state.lens_content.verify_blob_checksums_on_read}
    onChange={(v) => patch({ verify_blob_checksums_on_read: v })} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
</style>
