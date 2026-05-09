<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
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

<h1>Audio Lens (X-06)</h1>

<Section title="Lens">
  <Checkbox id="la-en" label="Enable audio lens"
    checked={settingsStore.state.lens_audio.enabled}
    onChange={(v) => patch({ enabled: v })} />
</Section>

<Section title="Per-format Mode">
  {#each FORMATS as fmt (fmt)}
    <Dropdown id={`la-fmt-${fmt}`} label={fmt}
      value={modeOf(fmt)}
      options={[ { value: "eager", label: "Eager" }, { value: "lazy", label: "Lazy (default)" }, { value: "disabled", label: "Disabled" } ]}
      onChange={(v) => setFmt(fmt, v)} />
  {/each}
</Section>

<Section title="Loudness">
  <Dropdown id="la-lufs-ref" label="LUFS reference standard"
    value={settingsStore.state.lens_audio.lufs_reference}
    options={[ { value: "ebu_r128", label: "EBU R128 (default)" }, { value: "atsc_a85", label: "ATSC A/85" }, { value: "spotify", label: "Spotify (-14)" }, { value: "apple_music", label: "Apple Music (-16)" }, { value: "broadcast_film", label: "Broadcast film (-23)" } ]}
    onChange={(v) => patch({ lufs_reference: v })} />
  <Dropdown id="la-peak" label="Compute peak via"
    value={settingsStore.state.lens_audio.peak_compute}
    options={[ { value: "true_peak", label: "True peak (4× oversampling, default)" }, { value: "sample_peak", label: "Sample peak" } ]}
    onChange={(v) => patch({ peak_compute: v })} />
  <NumberInput id="la-silence" label="Silence threshold" min={-90} max={0} step={1} suffix="dBFS"
    value={settingsStore.state.lens_audio.silence_threshold_dbfs}
    onChange={(n) => patch({ silence_threshold_dbfs: n })} />
  <Checkbox id="la-re-extract" label="Re-extract on Modify event"
    checked={settingsStore.state.lens_audio.re_extract_on_modify}
    onChange={(v) => patch({ re_extract_on_modify: v })} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
</style>
