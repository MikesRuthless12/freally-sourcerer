<script lang="ts">
  import { settingsDialog, type PanelId } from "../../lib/stores/settings_dialog.svelte";

  interface TreeNode {
    id?: PanelId;
    label: string;
    children?: TreeNode[];
    keywords?: string[];
  }

  const TREE: TreeNode[] = [
    {
      label: "General",
      children: [
        { id: "general.ui", label: "UI", keywords: ["theme", "tray", "row density", "thumbnails"] },
        { id: "general.home", label: "Home", keywords: ["defaults"] },
        { id: "general.search", label: "Search", keywords: ["dsl", "regex", "wildcard"] },
        { id: "general.results", label: "Results", keywords: ["columns", "sort", "icons"] },
        { id: "general.view", label: "View", keywords: ["preview", "size", "tooltips"] },
        { id: "general.context_menu", label: "Context Menu", keywords: ["shell", "explorer", "finder"] },
        { id: "general.fonts_colors", label: "Fonts & Colors", keywords: ["font", "color", "accent"] },
        { id: "general.keyboard", label: "Keyboard", keywords: ["hotkey", "shortcut", "chord"] }
      ]
    },
    { id: "history", label: "History", keywords: ["recent queries", "privacy"] },
    {
      label: "Indexes",
      children: [
        { id: "indexes.top", label: "(top-level)", keywords: ["force rebuild", "compact", "verify"] },
        { id: "indexes.volumes", label: "Volumes", keywords: ["ntfs", "apfs", "ext4", "journal"] },
        { id: "indexes.folders", label: "Folders", keywords: ["watched", "rescan"] },
        { id: "indexes.file_lists", label: "File Lists", keywords: ["import", "export"] },
        { id: "indexes.exclude", label: "Exclude", keywords: ["hidden", "system", "cache"] }
      ]
    },
    {
      label: "Lenses",
      children: [
        { id: "lenses.filename", label: "Filename", keywords: ["trigram", "wildcard", "regex"] },
        { id: "lenses.content", label: "Content", keywords: ["pdf", "office", "code"] },
        { id: "lenses.audio", label: "Audio", keywords: ["lufs", "codec", "silence"] },
        { id: "lenses.similarity", label: "Similarity", keywords: ["minhash", "lsh", "jaccard"] },
        { id: "lenses.custom", label: "Custom", keywords: ["wasm", "sandbox", "community"] }
      ]
    },
    {
      label: "Network",
      children: [
        { id: "network.https", label: "HTTP / HTTPS Server", keywords: ["axum", "rustls", "token"] },
        { id: "network.api", label: "ETP / FTP API", keywords: ["legacy", "ftp"] }
      ]
    },
    { id: "privacy", label: "Privacy & Updates", keywords: ["auto-update", "telemetry"] },
    { id: "logs", label: "Logs & Debug", keywords: ["tracing", "diagnostics"] },
    { id: "backup", label: "Backup, Export, Reset", keywords: ["import", "export", "reset"] },
    { id: "locale", label: "Locale", keywords: ["language", "rtl", "date"] },
    { id: "about", label: "About", keywords: ["version", "credits"] }
  ];

  function matches(node: TreeNode, query: string): boolean {
    const q = query.trim().toLowerCase();
    if (!q) return true;
    if (node.label.toLowerCase().includes(q)) return true;
    if ((node.keywords ?? []).some((k) => k.toLowerCase().includes(q))) return true;
    return (node.children ?? []).some((c) => matches(c, query));
  }

  function visibleChildren(parent: TreeNode, query: string): TreeNode[] {
    return (parent.children ?? []).filter((c) => matches(c, query));
  }
</script>

<aside class="tree" role="tree" aria-label="Settings categories">
  <input
    type="search"
    aria-label="Search settings"
    placeholder="Search options…"
    value={settingsDialog.search}
    oninput={(e) => settingsDialog.setSearch((e.currentTarget as HTMLInputElement).value)}
  />
  <ul>
    {#each TREE as group (group.label + (group.id ?? ""))}
      {@const showThis = matches(group, settingsDialog.search)}
      {#if showThis}
        {#if group.children}
          <li role="treeitem" aria-expanded="true">
            <span class="group">{group.label}</span>
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
                    {child.label}
                    {#if child.id && settingsDialog.dirtyPanels.has(child.id)}
                      <span class="dirty-dot" aria-label="unsaved changes">•</span>
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
              {group.label}
              {#if group.id && settingsDialog.dirtyPanels.has(group.id)}
                <span class="dirty-dot" aria-label="unsaved changes">•</span>
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
