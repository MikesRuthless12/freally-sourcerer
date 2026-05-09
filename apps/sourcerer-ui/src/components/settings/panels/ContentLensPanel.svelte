<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import type { ContentLensSettings } from "../../../lib/ipc/types";

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

<h1>Content Lens (X-02)</h1>

<Section title="Lens">
  <Checkbox id="lc-en" label="Enable content lens"
    checked={settingsStore.state.lens_content.enabled}
    onChange={(v) => patch({ enabled: v })} />
</Section>

<Section title="Per-format Mode">
  {#each FORMATS as fmt (fmt)}
    <Dropdown id={`lc-fmt-${fmt}`} label={fmt}
      value={effectiveMode(fmt)}
      options={[ { value: "eager", label: "Eager" }, { value: "lazy", label: "Lazy (default)" }, { value: "disabled", label: "Disabled" } ]}
      onChange={(v) => setFormatMode(fmt, v)} />
  {/each}
</Section>

<Section title="Budgets">
  <NumberInput id="lc-time" label="Time budget per document" min={50} max={60000} suffix="ms"
    value={settingsStore.state.lens_content.time_budget_ms}
    onChange={(n) => patch({ time_budget_ms: n })} />
  <NumberInput id="lc-mem" label="Memory ceiling per document" min={16} max={4096} suffix="MB"
    value={settingsStore.state.lens_content.memory_ceiling_mb}
    onChange={(n) => patch({ memory_ceiling_mb: n })} />
  <NumberInput id="lc-snip" label="Snippet length" min={50} max={2000} suffix="chars"
    value={settingsStore.state.lens_content.snippet_length}
    onChange={(n) => patch({ snippet_length: n })} />
</Section>

<Section title="Other">
  <Dropdown id="lc-stop" label="Stop-words language"
    value={settingsStore.state.lens_content.stop_words_language}
    options={[ { value: "auto", label: "Auto (per-doc)" }, { value: "en", label: "English" }, { value: "es", label: "Spanish" }, { value: "zh-CN", label: "Chinese (Simplified)" }, { value: "hi", label: "Hindi" }, { value: "ar", label: "Arabic" }, { value: "pt-BR", label: "Portuguese (BR)" }, { value: "ru", label: "Russian" }, { value: "ja", label: "Japanese" }, { value: "de", label: "German" }, { value: "fr", label: "French" }, { value: "ko", label: "Korean" }, { value: "it", label: "Italian" }, { value: "tr", label: "Turkish" }, { value: "vi", label: "Vietnamese" }, { value: "pl", label: "Polish" }, { value: "nl", label: "Dutch" }, { value: "id", label: "Indonesian" }, { value: "uk", label: "Ukrainian" } ]}
    onChange={(v) => patch({ stop_words_language: v })} />
  <Checkbox id="lc-re-ext" label="Re-extract on settings change"
    checked={settingsStore.state.lens_content.re_extract_on_settings_change}
    onChange={(v) => patch({ re_extract_on_settings_change: v })} />
  <Checkbox id="lc-verify" label="Verify extracted-text blob checksums on read"
    checked={settingsStore.state.lens_content.verify_blob_checksums_on_read}
    onChange={(v) => patch({ verify_blob_checksums_on_read: v })} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
</style>
