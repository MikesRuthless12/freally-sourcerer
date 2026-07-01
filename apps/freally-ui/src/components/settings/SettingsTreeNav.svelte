<script lang="ts">
  import { settingsDialog, type PanelId } from "../../lib/stores/settings_dialog.svelte";
  import { t } from "../../lib/i18n/t";

  interface TreeNode {
    id?: PanelId;
    /** Translation key. */
    labelKey: string;
    children?: TreeNode[];
    keywords?: string[];
  }

  const TREE: TreeNode[] = [
    {
      labelKey: "settings-group-general",
      children: [
        { id: "general.ui", labelKey: "settings-node-ui", keywords: ["theme", "tray", "row density", "thumbnails"] },
        { id: "general.home", labelKey: "settings-node-home", keywords: ["defaults"] },
        { id: "general.search", labelKey: "settings-node-search", keywords: ["dsl", "regex", "wildcard"] },
        { id: "general.results", labelKey: "settings-node-results", keywords: ["columns", "sort", "icons"] },
        { id: "general.view", labelKey: "settings-node-view", keywords: ["preview", "size", "tooltips"] },
        { id: "general.context_menu", labelKey: "settings-node-context-menu", keywords: ["shell", "explorer", "finder"] },
        { id: "general.fonts_colors", labelKey: "settings-node-fonts-colors", keywords: ["font", "color", "accent"] },
        { id: "general.keyboard", labelKey: "settings-node-keyboard", keywords: ["hotkey", "shortcut", "chord"] }
      ]
    },
    { id: "history", labelKey: "settings-group-history", keywords: ["recent queries", "privacy"] },
    {
      labelKey: "settings-group-indexes",
      children: [
        { id: "indexes.top", labelKey: "settings-node-indexes-top", keywords: ["force rebuild", "compact", "verify"] },
        { id: "indexes.volumes", labelKey: "settings-node-volumes", keywords: ["ntfs", "apfs", "ext4", "journal"] },
        { id: "indexes.folders", labelKey: "settings-node-folders", keywords: ["watched", "rescan"] },
        { id: "indexes.file_lists", labelKey: "settings-node-file-lists", keywords: ["import", "export"] },
        { id: "indexes.exclude", labelKey: "settings-node-exclude", keywords: ["hidden", "system", "cache"] }
      ]
    },
    {
      labelKey: "settings-group-lenses",
      children: [
        { id: "lenses.filename", labelKey: "lens-filename", keywords: ["trigram", "wildcard", "regex"] },
        { id: "lenses.content", labelKey: "lens-content", keywords: ["pdf", "office", "code"] },
        { id: "lenses.audio", labelKey: "lens-audio", keywords: ["lufs", "codec", "silence"] },
        { id: "lenses.similarity", labelKey: "lens-similarity", keywords: ["minhash", "lsh", "jaccard"] },
        { id: "lenses.custom", labelKey: "settings-tree-custom-lens", keywords: ["wasm", "sandbox", "community"] }
      ]
    },
    {
      labelKey: "settings-group-network",
      children: [
        { id: "network.https", labelKey: "settings-node-https-server", keywords: ["axum", "rustls", "token"] },
        { id: "network.api", labelKey: "settings-node-etp-api", keywords: ["legacy", "ftp"] }
      ]
    },
    { id: "privacy", labelKey: "settings-group-privacy", keywords: ["auto-update", "telemetry"] },
    { id: "logs", labelKey: "settings-group-logs", keywords: ["tracing", "diagnostics"] },
    { id: "backup", labelKey: "settings-group-backup", keywords: ["import", "export", "reset"] },
    { id: "locale", labelKey: "settings-node-locale", keywords: ["language", "rtl", "date"] },
    { id: "about", labelKey: "settings-node-about", keywords: ["version", "credits"] }
  ];

  function matches(node: TreeNode, query: string): boolean {
    const q = query.trim().toLowerCase();
    if (!q) return true;
    if (t(node.labelKey).toLowerCase().includes(q)) return true;
    if (node.labelKey.toLowerCase().includes(q)) return true;
    if ((node.keywords ?? []).some((k) => k.toLowerCase().includes(q))) return true;
    return (node.children ?? []).some((c) => matches(c, query));
  }

  function visibleChildren(parent: TreeNode, query: string): TreeNode[] {
    return (parent.children ?? []).filter((c) => matches(c, query));
  }
</script>

<aside class="tree" role="tree" aria-label={t("settings-title")}>
  <input
    type="search"
    aria-label={t("settings-title")}
    placeholder={t("settings-search-placeholder")}
    value={settingsDialog.search}
    oninput={(e) => settingsDialog.setSearch((e.currentTarget as HTMLInputElement).value)}
  />
  <ul>
    {#each TREE as group (group.labelKey + (group.id ?? ""))}
      {@const showThis = matches(group, settingsDialog.search)}
      {#if showThis}
        {#if group.children}
          <li role="treeitem" aria-expanded="true">
            <span class="group">{t(group.labelKey)}</span>
            <ul>
              {#each visibleChildren(group, settingsDialog.search) as child (child.id)}
                <li role="treeitem">
                  <button
                    type="button"
                    class="leaf"
                    class:selected={settingsDialog.selected === child.id}
                    aria-current={settingsDialog.selected === child.id ? "page" : undefined}
                    onclick={() => child.id && settingsDialog.setSelected(child.id)}
                  >
                    {t(child.labelKey)}
                    {#if child.id && settingsDialog.dirtyPanels.has(child.id)}
                      <span class="dirty-dot" aria-label={t("settings-unsaved-changes")}>•</span>
                    {/if}
                  </button>
                </li>
              {/each}
            </ul>
          </li>
        {:else}
          <li role="treeitem">
            <button
              type="button"
              class="leaf top"
              class:selected={settingsDialog.selected === group.id}
              aria-current={settingsDialog.selected === group.id ? "page" : undefined}
              onclick={() => group.id && settingsDialog.setSelected(group.id)}
            >
              {t(group.labelKey)}
              {#if group.id && settingsDialog.dirtyPanels.has(group.id)}
                <span class="dirty-dot" aria-label={t("settings-unsaved-changes")}>•</span>
              {/if}
            </button>
          </li>
        {/if}
      {/if}
    {/each}
  </ul>
</aside>

<style>
  .tree {
    width: 220px;
    min-width: 180px;
    max-width: 360px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    padding: 12px 8px;
    overflow-y: auto;
    background: var(--bg-surface);
  }
  input[type="search"] {
    width: 100%;
    padding: 6px 8px;
    margin-bottom: 8px;
    background: var(--bg-canvas);
    color: var(--text-primary);
    border: 1px solid var(--border);
    border-radius: 4px;
    font: inherit;
  }
  ul {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  ul ul {
    margin-left: 8px;
  }
  .group {
    display: block;
    padding: 4px 6px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-secondary);
  }
  .leaf {
    width: 100%;
    text-align: left;
    background: none;
    border: 0;
    color: var(--text-primary);
    padding: 5px 8px;
    border-radius: 4px;
    cursor: pointer;
    font: inherit;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .leaf:hover {
    background: var(--bg-canvas);
  }
  .leaf.selected {
    background: var(--accent-cyan);
    color: var(--bg-canvas);
  }
  .leaf.selected .dirty-dot {
    color: var(--bg-canvas);
  }
  .leaf.top {
    font-weight: 600;
  }
  .dirty-dot {
    color: var(--accent-purple);
    font-size: 18px;
    line-height: 0;
  }
</style>
