<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import type { FilenameLensSettings } from "../../../lib/ipc/types";

  function patch(p: Partial<FilenameLensSettings>) {
    settingsStore.patch({ lens_filename: { ...settingsStore.state.lens_filename, ...p } });
    settingsDialog.markDirty("lenses.filename");
  }
</script>

<h1>Filename Lens</h1>
<p class="hint">Always on. Sourcerer's filename trigram + suffix-array index lives here.</p>

<Section title="Tuning (+)">
  <Dropdown id="lf-trig" label="Trigram pre-filter aggressiveness"
    value={settingsStore.state.lens_filename.trigram_aggressiveness}
    options={[ { value: "low", label: "Low" }, { value: "normal", label: "Normal (default)" }, { value: "high", label: "High" } ]}
    onChange={(v) => patch({ trigram_aggressiveness: v })} />
  <NumberInput id="lf-mem" label="Suffix-array memory budget" min={64} max={4096} suffix="MB"
    value={settingsStore.state.lens_filename.suffix_array_memory_mb}
    onChange={(n) => patch({ suffix_array_memory_mb: n })} />
  <NumberInput id="lf-wild" label="Wildcard expansion limit" min={1000} max={10000000}
    suffix="candidates"
    value={settingsStore.state.lens_filename.wildcard_expansion_limit}
    onChange={(n) => patch({ wildcard_expansion_limit: n })} />
  <NumberInput id="lf-rgx-to" label="Regex timeout" min={1} max={5000} suffix="ms"
    value={settingsStore.state.lens_filename.regex_timeout_ms}
    onChange={(n) => patch({ regex_timeout_ms: n })} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  .hint { margin: 0 0 16px; color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
</style>
