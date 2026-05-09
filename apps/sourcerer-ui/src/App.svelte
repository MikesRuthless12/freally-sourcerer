<script lang="ts">
  import { onMount } from "svelte";
  import MenuBar from "./components/menu/MenuBar.svelte";
  import SearchBar from "./components/search-bar/SearchBar.svelte";
  import QuickFiltersPalette from "./components/filters/QuickFiltersPalette.svelte";
  import ResultList from "./components/results/ResultList.svelte";
  import StatusBar from "./components/statusbar/StatusBar.svelte";
  import FirstRunWizard from "./components/wizard/FirstRunWizard.svelte";
  import OrganizeBookmarksDialog from "./components/bookmarks/OrganizeBookmarksDialog.svelte";
  import AboutDialog from "./components/dialogs/AboutDialog.svelte";
  import SettingsDialog from "./components/settings/SettingsDialog.svelte";
  import { bootstrap } from "./lib/bootstrap";
  import { dialogsStore } from "./lib/stores/dialogs.svelte";

  onMount(() => {
    void bootstrap();
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
