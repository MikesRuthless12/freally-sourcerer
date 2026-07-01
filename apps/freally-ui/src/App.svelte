<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import MenuBar from "./components/menu/MenuBar.svelte";
  import SearchBar from "./components/search-bar/SearchBar.svelte";
  import QuickFiltersPalette from "./components/filters/QuickFiltersPalette.svelte";
  import ResultList from "./components/results/ResultList.svelte";
  import StatusBar from "./components/statusbar/StatusBar.svelte";
  import FirstRunWizard from "./components/wizard/FirstRunWizard.svelte";
  import OrganizeBookmarksDialog from "./components/bookmarks/OrganizeBookmarksDialog.svelte";
  import AboutDialog from "./components/dialogs/AboutDialog.svelte";
  import ConnectEndpointDialog from "./components/dialogs/ConnectEndpointDialog.svelte";
  import SettingsDialog from "./components/settings/SettingsDialog.svelte";
  import { bootstrap } from "./lib/bootstrap";
  import { dialogsStore } from "./lib/stores/dialogs.svelte";
  import { settingsStore } from "./lib/stores/settings.svelte";
  import { queryStore } from "./lib/stores/query.svelte";

  onMount(() => {
    void bootstrap();
  });

  // Live "always on top" effect: re-applies on every settings change AND
  // every keystroke when in "while_searching" mode. Survives reloads
  // because settingsStore is hydrated before the first effect run.
  $effect(() => {
    const mode = settingsStore.state.on_top;
    const hasQuery = queryStore.source.trim().length > 0;
    const target =
      mode === "always" ? true : mode === "while_searching" ? hasQuery : false;
    getCurrentWindow()
      .setAlwaysOnTop(target)
      .catch((e) => console.warn("[app] setAlwaysOnTop failed:", e));
  });

  // Mirror selected view-panel toggles to <body data-*> attributes so
  // CSS in ResultRow.svelte / ResultList.svelte can react without each
  // component owning its own settings subscription.
  $effect(() => {
    if (typeof document === "undefined") return;
    const s = settingsStore.state as unknown as Record<string, unknown>;
    const body = document.body;
    body.dataset.alternateRows = String(s.alternate_row_color === true);
    body.dataset.rowMouseover = String(s.show_row_mouseover !== false);
    body.dataset.showTooltips = String(s.show_tooltips !== false);
    body.dataset.showLufsBadges = String(s.show_lufs_codec_length_badges !== false);
    body.dataset.showSimilarityScore = String(s.show_minhash_similarity_score !== false);
  });
</script>

<div class="app">
  <MenuBar />
  <SearchBar />
  <QuickFiltersPalette />
  <ResultList />
  <StatusBar />
</div>

<FirstRunWizard />
<OrganizeBookmarksDialog
  open={dialogsStore.active === "organize_bookmarks"}
  onClose={() => dialogsStore.close()}
/>
<AboutDialog
  open={dialogsStore.active === "about"}
  onClose={() => dialogsStore.close()}
/>
<SettingsDialog
  open={dialogsStore.active === "settings"}
  onClose={() => dialogsStore.close()}
/>
<ConnectEndpointDialog
  open={dialogsStore.active === "connect_endpoint"}
  onClose={() => dialogsStore.close()}
/>

<style>
  .app {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100vh;
    background: var(--bg-canvas);
    color: var(--text-primary);
  }
</style>
