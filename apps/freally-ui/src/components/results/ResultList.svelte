<script lang="ts">
  import DidYouMeanStrip from "./DidYouMeanStrip.svelte";
  import LensSection from "./LensSection.svelte";
  import PreviewPane from "../preview/PreviewPane.svelte";
  import Sidebar from "../sidebar/Sidebar.svelte";
  import { resultsStore } from "../../lib/stores/results.svelte";
  import { queryStore } from "../../lib/stores/query.svelte";
  import { settingsStore } from "../../lib/stores/settings.svelte";
  import { typeFilterStore } from "../../lib/stores/type_filter.svelte";
  import { t } from "../../lib/i18n/t";
  import type { LensId } from "../../lib/ipc/types";

  // `$derived` so locale switches re-render the section headers.
  const lensTitles = $derived<Record<LensId, string>>({
    filename: t("lens-filename"),
    content: t("lens-content"),
    audio: t("lens-audio"),
    similarity: t("lens-similarity")
  });

  const order: LensId[] = ["filename", "content", "audio", "similarity"];

  function timingFor(lens: LensId): number {
    const t = resultsStore.timings;
    if (!t) return 0;
    switch (lens) {
      case "filename": return t.filename_ms;
      case "content": return t.content_ms;
      case "audio": return t.audio_ms;
      case "similarity": return t.similarity_ms;
    }
  }
</script>

<div class="result-area">
  <!-- SRC-M22 — lives inside `.result-area` so the sidebar, the list
       and the preview pane share one flex row, and so toggling it does
       not disturb the menu / search / status chrome above and below. -->
  {#if settingsStore.state.show_sidebar}
    <Sidebar />
  {/if}
  <div class="result-list">
    {#if !queryStore.source.trim() && typeFilterStore.isNoneSelected()}
      <div class="empty-state">
        <p>{t("parse-error-empty")}</p>
      </div>
    {:else}
      <DidYouMeanStrip />
      {#each order as lens (lens)}
        {@const view = resultsStore.viewForLens(lens)}
        <LensSection
          {lens}
          title={lensTitles[lens]}
          hits={view.hits}
          groups={view.groups}
          timingMs={timingFor(lens)}
        />
      {/each}
    {/if}
  </div>
  {#if settingsStore.state.show_preview}
    <PreviewPane />
  {/if}
</div>

<style>
  .result-area {
    flex: 1;
    display: flex;
    overflow: hidden;
  }
  .result-list {
    flex: 1;
    overflow-y: auto;
    padding: 12px;
    background: var(--bg-canvas);
  }
  .empty-state {
    display: grid;
    place-items: center;
    height: 100%;
    color: var(--text-secondary);
    font-size: 14px;
  }
</style>
