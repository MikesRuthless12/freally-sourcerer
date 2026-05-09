<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import { applyRtlForLocale } from "../../../lib/bootstrap";
  import type { LocaleSettings } from "../../../lib/ipc/types";

  // 18 ship-locales per PRD §8.26. English is pinned first regardless
  // of the current display language; the remaining 17 sit below in
  // alphabetical order by their native-name spelling. Each label is
  // the language's own self-name so the user can pick their language
  // even when the UI is currently in a script they cannot read.
  const LOCALES = [
    // English pinned first.
    { value: "en", label: "English (en)" },
    // Latin-script natives, alphabetical by native name.
    { value: "id", label: "Bahasa Indonesia (id)" },
    { value: "de", label: "Deutsch (de)" },
    { value: "es", label: "Español (es)" },
    { value: "fr", label: "Français (fr)" },
    { value: "it", label: "Italiano (it)" },
    { value: "nl", label: "Nederlands (nl)" },
    { value: "pl", label: "Polski (pl)" },
    { value: "pt-BR", label: "Português (pt-BR)" },
    { value: "tr", label: "Türkçe (tr)" },
    { value: "vi", label: "Tiếng Việt (vi)" },
    // Cyrillic, alphabetical by native first letter.
    { value: "ru", label: "Русский (ru)" },
    { value: "uk", label: "Українська (uk)" },
    // RTL.
    { value: "ar", label: "العربية (ar) — RTL" },
    // Other scripts.
    { value: "hi", label: "हिन्दी (hi)" },
    { value: "ja", label: "日本語 (ja)" },
    { value: "ko", label: "한국어 (ko)" },
    { value: "zh-CN", label: "简体中文 (zh-CN)" }
  ];

  function patch(p: Partial<LocaleSettings>) {
    settingsStore.patch({ locale_settings: { ...settingsStore.state.locale_settings, ...p } });
    if (p.locale) {
      settingsStore.patch({ locale: p.locale });
      // Live RTL flip — the chosen locale (or RTL preview override)
      // immediately applies `dir="rtl"` on the document root.
      applyRtlForLocale(
        settingsStore.state.locale_settings.rtl_preview ? "ar" : p.locale
      );
    }
    if (p.rtl_preview !== undefined) {
      applyRtlForLocale(p.rtl_preview ? "ar" : settingsStore.state.locale);
    }
    settingsDialog.markDirty("locale");
  }
</script>

<h1>Locale</h1>

<Section title="Language (+)">
  <Dropdown id="lc-locale" label="Current locale"
    value={settingsStore.state.locale_settings.locale}
    options={LOCALES}
    onChange={(v) => patch({ locale: v })} />
  <Checkbox id="lc-rtl" label="RTL preview (mirror layout to test localization)"
    checked={settingsStore.state.locale_settings.rtl_preview}
    onChange={(v) => patch({ rtl_preview: v })} />
</Section>

<Section title="Time / Date (+)">
  <Dropdown id="lc-date-fmt" label="Date format"
    value={settingsStore.state.locale_settings.date_format}
    options={[ { value: "os", label: "OS default" }, { value: "iso8601", label: "ISO 8601" }, { value: "rfc3339", label: "RFC 3339" }, { value: "custom", label: "Custom" } ]}
    onChange={(v) => patch({ date_format: v })} />
  {#if settingsStore.state.locale_settings.date_format === "custom"}
    <TextInput id="lc-date-custom" label="Custom format"
      value={settingsStore.state.locale_settings.date_format_custom}
      placeholder="YYYY-MM-DD"
      onChange={(v) => patch({ date_format_custom: v })} />
  {/if}
</Section>

<Section title="Numbers (+)">
  <Dropdown id="lc-num-fmt" label="Number format"
    value={settingsStore.state.locale_settings.number_format}
    options={[ { value: "os", label: "OS default" }, { value: "custom", label: "Custom" } ]}
    onChange={(v) => patch({ number_format: v })} />
  {#if settingsStore.state.locale_settings.number_format === "custom"}
    <TextInput id="lc-thou" label="Thousands separator"
      value={settingsStore.state.locale_settings.thousands_separator}
      onChange={(v) => patch({ thousands_separator: v })} />
    <TextInput id="lc-dec" label="Decimal separator"
      value={settingsStore.state.locale_settings.decimal_separator}
      onChange={(v) => patch({ decimal_separator: v })} />
  {/if}
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
</style>
