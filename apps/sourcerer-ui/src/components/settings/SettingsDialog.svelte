<script lang="ts">
  import { settingsDialog } from "../../lib/stores/settings_dialog.svelte";
  import SettingsTreeNav from "./SettingsTreeNav.svelte";
  import SettingsButtonBar from "./SettingsButtonBar.svelte";
  import UIPanel from "./panels/UIPanel.svelte";
  import HomePanel from "./panels/HomePanel.svelte";
  import SearchPanel from "./panels/SearchPanel.svelte";
  import ResultsPanel from "./panels/ResultsPanel.svelte";
  import ViewPanel from "./panels/ViewPanel.svelte";
  import ContextMenuPanel from "./panels/ContextMenuPanel.svelte";
  import FontsAndColorsPanel from "./panels/FontsAndColorsPanel.svelte";
  import KeyboardPanel from "./panels/KeyboardPanel.svelte";
  import HistoryPanel from "./panels/HistoryPanel.svelte";
  import IndexesTopPanel from "./panels/IndexesTopPanel.svelte";
  import VolumesPanel from "./panels/VolumesPanel.svelte";
  import FoldersPanel from "./panels/FoldersPanel.svelte";
  import FileListsPanel from "./panels/FileListsPanel.svelte";
  import ExcludePanel from "./panels/ExcludePanel.svelte";
  import FilenameLensPanel from "./panels/FilenameLensPanel.svelte";
  import ContentLensPanel from "./panels/ContentLensPanel.svelte";
  import AudioLensPanel from "./panels/AudioLensPanel.svelte";
  import SimilarityLensPanel from "./panels/SimilarityLensPanel.svelte";
  import CustomLensPanel from "./panels/CustomLensPanel.svelte";
  import HttpsServerPanel from "./panels/HttpsServerPanel.svelte";
  import EtpApiPanel from "./panels/EtpApiPanel.svelte";
  import PrivacyAndUpdatesPanel from "./panels/PrivacyAndUpdatesPanel.svelte";
  import LogsAndDebugPanel from "./panels/LogsAndDebugPanel.svelte";
  import BackupPanel from "./panels/BackupPanel.svelte";
  import LocalePanel from "./panels/LocalePanel.svelte";
  import AboutPanel from "./panels/AboutPanel.svelte";

  interface Props {
    open: boolean;
    onClose: () => void;
  }
  let { open, onClose }: Props = $props();

  $effect(() => {
    settingsDialog.openDialog(open);
  });

  function handleEsc(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    }
  }
</script>

{#if open}
  <div
    class="backdrop"
    role="presentation"
    onkeydown={handleEsc}
  >
    <div
      class="modal"
      role="dialog"
      aria-modal="true"
      aria-label="Sourcerer Options"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <header>
        <h2>Options</h2>
      </header>
      <div class="body">
        <SettingsTreeNav />
        <main class="pane">
          {#if settingsDialog.selected === "general.ui"}
            <UIPanel />
          {:else if settingsDialog.selected === "general.home"}
            <HomePanel />
          {:else if settingsDialog.selected === "general.search"}
            <SearchPanel />
          {:else if settingsDialog.selected === "general.results"}
            <ResultsPanel />
          {:else if settingsDialog.selected === "general.view"}
            <ViewPanel />
          {:else if settingsDialog.selected === "general.context_menu"}
            <ContextMenuPanel />
          {:else if settingsDialog.selected === "general.fonts_colors"}
            <FontsAndColorsPanel />
          {:else if settingsDialog.selected === "general.keyboard"}
            <KeyboardPanel />
          {:else if settingsDialog.selected === "history"}
            <HistoryPanel />
          {:else if settingsDialog.selected === "indexes.top"}
            <IndexesTopPanel />
          {:else if settingsDialog.selected === "indexes.volumes"}
            <VolumesPanel />
          {:else if settingsDialog.selected === "indexes.folders"}
            <FoldersPanel />
          {:else if settingsDialog.selected === "indexes.file_lists"}
            <FileListsPanel />
          {:else if settingsDialog.selected === "indexes.exclude"}
            <ExcludePanel />
          {:else if settingsDialog.selected === "lenses.filename"}
            <FilenameLensPanel />
          {:else if settingsDialog.selected === "lenses.content"}
            <ContentLensPanel />
          {:else if settingsDialog.selected === "lenses.audio"}
            <AudioLensPanel />
          {:else if settingsDialog.selected === "lenses.similarity"}
            <SimilarityLensPanel />
          {:else if settingsDialog.selected === "lenses.custom"}
            <CustomLensPanel />
          {:else if settingsDialog.selected === "network.https"}
            <HttpsServerPanel />
          {:else if settingsDialog.selected === "network.api"}
            <EtpApiPanel />
          {:else if settingsDialog.selected === "privacy"}
            <PrivacyAndUpdatesPanel />
          {:else if settingsDialog.selected === "logs"}
            <LogsAndDebugPanel />
          {:else if settingsDialog.selected === "backup"}
            <BackupPanel />
          {:else if settingsDialog.selected === "locale"}
            <LocalePanel />
          {:else if settingsDialog.selected === "about"}
            <AboutPanel />
          {/if}
        </main>
      </div>
      <SettingsButtonBar />
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    display: grid;
    place-items: center;
    z-index: 100;
  }
  .modal {
    width: min(960px, 95vw);
    height: min(720px, 90vh);
    min-width: 800px;
    min-height: 620px;
    display: flex;
    flex-direction: column;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 10px;
    box-shadow: 0 18px 60px rgba(0, 0, 0, 0.5);
    overflow: hidden;
  }
  header {
    padding: 12px 18px;
    border-bottom: 1px solid var(--border);
    background: var(--bg-canvas);
  }
  h2 {
    margin: 0;
    font-size: 14px;
    color: var(--text-primary);
  }
  .body {
    flex: 1;
    display: flex;
    overflow: hidden;
  }
  .pane {
    flex: 1;
    overflow-y: auto;
    padding: 18px 24px;
    color: var(--text-primary);
  }
</style>
