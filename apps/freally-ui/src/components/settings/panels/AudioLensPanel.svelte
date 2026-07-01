<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import { t } from "../../../lib/i18n/t";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import type { AudioLensSettings } from "../../../lib/ipc/types";

  const FORMATS = ["flac", "mp3", "aac", "ogg", "opus", "wav", "aiff", "m4a", "alac", "vorbis"];

  function patch(p: Partial<AudioLensSettings>) {
    settingsStore.patch({ lens_audio: { ...settingsStore.state.lens_audio, ...p } });
    settingsDialog.markDirty("lenses.audio");
  }
  function setFmt(f: string, m: "eager" | "lazy" | "disabled") {
    patch({ per_format: { ...settingsStore.state.lens_audio.per_format, [f]: m } });
  }
  function modeOf(f: string): "eager" | "lazy" | "disabled" {
    return settingsStore.state.lens_audio.per_format[f] ?? "lazy";
  }
</script>

<h1>{t("lens-audio")}</h1>

<Section title={t("section-lens")}>
  <Checkbox id="la-en" label={t("settings-la-enable")}
    checked={settingsStore.state.lens_audio.enabled}
    onChange={(v) => patch({ enabled: v })} />
</Section>

<Section title={t("section-per-format-mode")}>
  {#each FORMATS as fmt (fmt)}
    <Dropdown id={`la-fmt-${fmt}`} label={fmt}
      value={modeOf(fmt)}
      options={[ { value: "eager", label: t("opt-eager") }, { value: "lazy", label: t("opt-lazy-default") }, { value: "disabled", label: t("opt-disabled") } ]}
      onChange={(v) => setFmt(fmt, v)} />
  {/each}
</Section>

<Section title={t("section-loudness")}>
  <Dropdown id="la-lufs-ref" label={t("settings-la-lufs-ref")}
    value={settingsStore.state.lens_audio.lufs_reference}
    options={[ { value: "ebu_r128", label: t("opt-ebu-r128-default") }, { value: "atsc_a85", label: t("opt-atsc-a85") }, { value: "spotify", label: t("opt-spotify") }, { value: "apple_music", label: t("opt-apple-music") }, { value: "broadcast_film", label: t("opt-broadcast-film") } ]}
    onChange={(v) => patch({ lufs_reference: v })} />
  <Dropdown id="la-peak" label={t("settings-la-peak-compute")}
    value={settingsStore.state.lens_audio.peak_compute}
    options={[ { value: "true_peak", label: t("opt-true-peak") }, { value: "sample_peak", label: t("opt-sample-peak") } ]}
    onChange={(v) => patch({ peak_compute: v })} />
  <NumberInput id="la-silence" label={t("settings-la-silence-thresh")} min={-90} max={0} step={1} suffix="dBFS"
    value={settingsStore.state.lens_audio.silence_threshold_dbfs}
    onChange={(n) => patch({ silence_threshold_dbfs: n })} />
  <Checkbox id="la-re-extract" label={t("settings-la-re-extract-modify")}
    checked={settingsStore.state.lens_audio.re_extract_on_modify}
    onChange={(v) => patch({ re_extract_on_modify: v })} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
</style>
