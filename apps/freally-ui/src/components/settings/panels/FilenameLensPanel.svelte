<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import { t } from "../../../lib/i18n/t";
  import Section from "../controls/Section.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import type { FilenameLensSettings } from "../../../lib/ipc/types";

  function patch(p: Partial<FilenameLensSettings>) {
    settingsStore.patch({ lens_filename: { ...settingsStore.state.lens_filename, ...p } });
    settingsDialog.markDirty("lenses.filename");
  }
</script>

<h1>{t("lens-filename")}</h1>
<p class="hint">Always on. Freally's filename trigram + suffix-array index lives here.</p>

<Section title={t("section-tuning")}>
  <Dropdown id="lf-trig" label={t("settings-lf-trigram")}
    value={settingsStore.state.lens_filename.trigram_aggressiveness}
    options={[ { value: "low", label: t("opt-low") }, { value: "normal", label: t("opt-normal-default") }, { value: "high", label: t("opt-high") } ]}
    onChange={(v) => patch({ trigram_aggressiveness: v })} />
  <NumberInput id="lf-mem" label={t("settings-lf-suffix-mem")} min={64} max={4096} suffix={t("unit-mb")}
    value={settingsStore.state.lens_filename.suffix_array_memory_mb}
    onChange={(n) => patch({ suffix_array_memory_mb: n })} />
  <NumberInput id="lf-wild" label={t("settings-lf-wildcard-limit")} min={1000} max={10000000}
    suffix="candidates"
    value={settingsStore.state.lens_filename.wildcard_expansion_limit}
    onChange={(n) => patch({ wildcard_expansion_limit: n })} />
  <NumberInput id="lf-rgx-to" label={t("settings-lf-regex-timeout")} min={1} max={5000} suffix="ms"
    value={settingsStore.state.lens_filename.regex_timeout_ms}
    onChange={(n) => patch({ regex_timeout_ms: n })} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  .hint { margin: 0 0 16px; color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
</style>
