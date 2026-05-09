<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import type { SimilarityLensSettings } from "../../../lib/ipc/types";

  function patch(p: Partial<SimilarityLensSettings>) {
    settingsStore.patch({ lens_similarity: { ...settingsStore.state.lens_similarity, ...p } });
    settingsDialog.markDirty("lenses.similarity");
  }
</script>

<h1>Similarity Lens (X-08)</h1>

<Section title="Lens">
  <Checkbox id="ls-en" label="Enable similarity lens"
    checked={settingsStore.state.lens_similarity.enabled}
    onChange={(v) => patch({ enabled: v })} />
</Section>

<Section title="MinHash + LSH Parameters (+)">
  <Dropdown id="ls-sig" label="MinHash signature size (k)"
    value={String(settingsStore.state.lens_similarity.signature_size)}
    options={[ { value: "64", label: "64" }, { value: "128", label: "128 (default)" }, { value: "256", label: "256" } ]}
    onChange={(v) => patch({ signature_size: Number(v) as 64 | 128 | 256 })} />
  <Dropdown id="ls-bands" label="LSH bands"
    value={String(settingsStore.state.lens_similarity.bands)}
    options={[ { value: "8", label: "8" }, { value: "16", label: "16 (default)" }, { value: "32", label: "32" } ]}
    onChange={(v) => patch({ bands: Number(v) as 8 | 16 | 32 })} />
  <Dropdown id="ls-recall" label="Recall threshold"
    value={String(settingsStore.state.lens_similarity.recall_threshold)}
    options={[ { value: "0.75", label: "0.75 (loose)" }, { value: "0.85", label: "0.85" }, { value: "0.95", label: "0.95 (tight, default)" } ]}
    onChange={(v) => patch({ recall_threshold: Number(v) })} />
  <NumberInput id="ls-cap" label="Result cap" min={1} max={1000}
    value={settingsStore.state.lens_similarity.result_cap}
    onChange={(n) => patch({ result_cap: n })} />
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
</style>
