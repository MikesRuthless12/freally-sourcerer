<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import { applyRtlForLocale } from "../../../lib/bootstrap";
  import { LOCALES } from "../../../lib/i18n/bundle";
  import { t } from "../../../lib/i18n/t";
  import type { LocaleSettings } from "../../../lib/ipc/types";

  function patch(p: Partial<LocaleSettings>) {
    settingsStore.patch({ locale_settings: { ...settingsStore.state.locale_settings, ...p } });
    if (p.locale) {
      settingsStore.patch({ locale: p.locale });
      // Live RTL flip — locales whose native script is RTL (e.g. Arabic)
      // automatically get `dir="rtl"` on the document root. There's no
      // user-facing override; `applyRtlForLocale` consults its internal
      // RTL_LOCALES allowlist.
      applyRtlForLocale(p.locale);
    }
    settingsDialog.markDirty("locale");
  }
</script>

<h1>{t("settings-node-locale")}</h1>

<Section title={t("locale-section-language")}>
  <Dropdown id="lc-locale" label={t("settings-locale-current")}
    value={settingsStore.state.locale_settings.locale}
    options={LOCALES}
    onChange={(v) => patch({ locale: v })} />
</Section>

<Section title={t("locale-section-time-date")}>
  <Dropdown id="lc-date-fmt" label={t("settings-locale-date-format")}
    value={settingsStore.state.locale_settings.date_format}
    options={[
      { value: "os", label: t("locale-date-os") },
      { value: "iso8601", label: t("locale-date-iso8601") },
      { value: "rfc3339", label: t("locale-date-rfc3339") },
      { value: "custom", label: t("locale-date-custom-label") }
    ]}
    onChange={(v) => patch({ date_format: v })} />
  {#if settingsStore.state.locale_settings.date_format === "custom"}
    <TextInput id="lc-date-custom" label={t("locale-date-custom-format")}
      value={settingsStore.state.locale_settings.date_format_custom}
      placeholder={t("locale-date-placeholder")}
      onChange={(v) => patch({ date_format_custom: v })} />
  {/if}
</Section>

<Section title={t("locale-section-numbers")}>
  <Dropdown id="lc-num-fmt" label={t("settings-locale-number-format")}
    value={settingsStore.state.locale_settings.number_format}
    options={[
      { value: "os", label: t("locale-number-os") },
      { value: "custom", label: t("locale-number-custom") }
    ]}
    onChange={(v) => patch({ number_format: v })} />
  {#if settingsStore.state.locale_settings.number_format === "custom"}
    <TextInput id="lc-thou" label={t("locale-thousands-sep")}
      value={settingsStore.state.locale_settings.thousands_separator}
      onChange={(v) => patch({ thousands_separator: v })} />
    <TextInput id="lc-dec" label={t("locale-decimal-sep")}
      value={settingsStore.state.locale_settings.decimal_separator}
      onChange={(v) => patch({ decimal_separator: v })} />
  {/if}
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
</style>
