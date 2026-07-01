<script lang="ts">
  import { MENU_BAR, type MenuRoot, type MenuNode, type MenuItemSpec } from "../../lib/commands/menu_spec";
  import type { CommandId } from "../../lib/commands/ids";
  import { registry } from "../../lib/commands/registry.svelte";
  import { menuHoverStore } from "../../lib/stores/menu_hover.svelte";
  import { searchOptsStore } from "../../lib/stores/search_opts.svelte";
  import { settingsStore } from "../../lib/stores/settings.svelte";
  import { sortStore } from "../../lib/stores/sort.svelte";
  import { typeFilterStore, type TypeFilterId } from "../../lib/stores/type_filter.svelte";
  import BookmarksDropdown from "../bookmarks/BookmarksDropdown.svelte";
  import { t } from "../../lib/i18n/t";

  function labelOf(spec: { label: string; l10n?: string }): string {
    if (!spec.l10n) return spec.label;
    const translated = t(spec.l10n);
    return translated === spec.l10n ? spec.label : translated;
  }
  function hintOf(spec: { hint?: string; hintL10n?: string }): string | undefined {
    if (!spec.hintL10n) return spec.hint;
    const translated = t(spec.hintL10n);
    return translated === spec.hintL10n ? spec.hint : translated;
  }

  const SORT_FIELD_BY_ID: Record<string, string> = {
    "view.sort.name": "name",
    "view.sort.path": "path",
    "view.sort.size": "size",
    "view.sort.ext": "ext",
    "view.sort.type": "type",
    "view.sort.modified": "modified",
    "view.sort.lufs": "lufs",
    "view.sort.length": "length",
    "view.sort.similarity": "similarity"
  };
  const FILTER_ID_BY_MENU_ID: Record<string, TypeFilterId> = {
    "search.filter.audio": "audio",
    "search.filter.compressed": "compressed",
    "search.filter.document": "document",
    "search.filter.executable": "executable",
    "search.filter.folder": "folder",
    "search.filter.picture": "picture",
    "search.filter.video": "video"
  };

  function isItemEnabled(item: MenuItemSpec): boolean {
    // The "Disconnect" command should only be live when the user has
    // actually connected to a remote endpoint. Local DB = nothing to
    // disconnect from, so grey it out (voidtools-Everything parity).
    if (item.id === "tools.disconnect_endpoint") {
      return settingsStore.state.endpoint?.kind !== "local";
    }
    return true;
  }

  function isItemChecked(item: MenuItemSpec): boolean {
    const id: CommandId = item.id;
    if (id === "search.match_case") return searchOptsStore.get("match_case");
    if (id === "search.match_whole_word") return searchOptsStore.get("match_whole_word");
    if (id === "search.match_path") return searchOptsStore.get("match_path");
    if (id === "search.match_diacritics") return searchOptsStore.get("match_diacritics");
    if (id === "search.enable_regex") return searchOptsStore.get("enable_regex");
    if (id === "view.theme.system") return settingsStore.state.theme === "system";
    if (id === "view.theme.light") return settingsStore.state.theme === "light";
    if (id === "view.theme.dark") return settingsStore.state.theme === "dark";
    if (id === "view.sort.ascending") return sortStore.order === "asc";
    if (id === "view.sort.descending") return sortStore.order === "desc";
    if (id === "view.lens.filename") return settingsStore.state.lens_visibility.filename;
    if (id === "view.lens.content") return settingsStore.state.lens_visibility.content;
    if (id === "view.lens.audio") return settingsStore.state.lens_visibility.audio;
    if (id === "view.lens.similarity") return settingsStore.state.lens_visibility.similarity;
    if (id === "view.on_top.never") return settingsStore.state.on_top === "never";
    if (id === "view.on_top.always") return settingsStore.state.on_top === "always";
    if (id === "view.on_top.while_searching") return settingsStore.state.on_top === "while_searching";
    if (id in SORT_FIELD_BY_ID) return sortStore.field === SORT_FIELD_BY_ID[id];
    if (id === "search.filter.everything") return typeFilterStore.isEverythingChecked();
    if (id in FILTER_ID_BY_MENU_ID) return typeFilterStore.has(FILTER_ID_BY_MENU_ID[id]!);
    return false;
  }

  let openIdx = $state<number | null>(null);
  let openSubmenuPath = $state<string | null>(null);

  function toggle(i: number) {
    openIdx = openIdx === i ? null : i;
    openSubmenuPath = null;
  }

  function close() {
    openIdx = null;
    openSubmenuPath = null;
    menuHoverStore.clear();
  }

  async function fire(node: MenuNode) {
    if (node.kind !== "item") return;
    close();
    await registry.run(node.id);
  }

  function onRootEnter(root: MenuRoot, i: number) {
    // Only update the hover hint when a menu is actually open — H9 fix.
    if (openIdx !== null) {
      openIdx = i;
      menuHoverStore.set(hintOf(root) ?? root.hint);
    }
  }

  function onItemEnter(node: MenuNode, fallbackHint: string) {
    if (node.kind === "item" || node.kind === "submenu") {
      menuHoverStore.set(hintOf(node) ?? node.hint ?? fallbackHint);
    }
  }

  function rootKey(ev: KeyboardEvent, i: number, root: MenuRoot) {
    if (ev.key === "Enter" || ev.key === " " || ev.key === "ArrowDown") {
      ev.preventDefault();
      openIdx = i;
      menuHoverStore.set(hintOf(root) ?? root.hint);
    } else if (ev.key === "ArrowRight") {
      ev.preventDefault();
      const next = (i + 1) % MENU_BAR.length;
      const btn = document.querySelectorAll<HTMLButtonElement>(".menubar .root-btn")[next];
      btn?.focus();
      if (openIdx !== null) openIdx = next;
    } else if (ev.key === "ArrowLeft") {
      ev.preventDefault();
      const prev = (i - 1 + MENU_BAR.length) % MENU_BAR.length;
      const btn = document.querySelectorAll<HTMLButtonElement>(".menubar .root-btn")[prev];
      btn?.focus();
      if (openIdx !== null) openIdx = prev;
    } else if (ev.key === "Escape") {
      ev.preventDefault();
      close();
    }
  }

  function submenuKey(ev: KeyboardEvent, key: string) {
    if (ev.key === "ArrowRight" || ev.key === "Enter") {
      ev.preventDefault();
      openSubmenuPath = key;
    } else if (ev.key === "ArrowLeft") {
      ev.preventDefault();
      openSubmenuPath = null;
    }
  }

  function isSubmenuOpen(key: string): boolean {
    return openSubmenuPath === key;
  }
</script>

<svelte:window onclick={close} />

<nav class="menubar" aria-label="Main menu">
  {#each MENU_BAR as root, i (root.label)}
    <div class="root" class:open={openIdx === i}>
      <button
        type="button"
        class="root-btn"
        aria-haspopup="menu"
        aria-expanded={openIdx === i}
        onpointerdown={(e) => {
          e.stopPropagation();
          toggle(i);
          if (openIdx !== null) menuHoverStore.set(hintOf(root) ?? root.hint);
        }}
        onclick={(e) => e.stopPropagation()}
        onmouseenter={() => onRootEnter(root, i)}
        onkeydown={(e) => rootKey(e, i, root)}
      >
        {labelOf(root)}
      </button>
      {#if openIdx === i}
        <div class="dropdown" role="menu">
          {#each root.children as child, ci (ci)}
            {#if child.kind === "separator"}
              <div class="sep" role="separator"></div>
            {:else if child.kind === "submenu"}
              {@const subKey = `${i}-${ci}`}
              <div
                class="submenu-row"
                class:submenu-open={isSubmenuOpen(subKey)}
                onmouseenter={() => {
                  onItemEnter(child, root.hint);
                  openSubmenuPath = subKey;
                }}
                onmouseleave={() => {
                  if (openSubmenuPath === subKey) openSubmenuPath = null;
                }}
                onfocusin={() => {
                  onItemEnter(child, root.hint);
                  openSubmenuPath = subKey;
                }}
                onkeydown={(e) => submenuKey(e, subKey)}
                role="presentation"
              >
                <button
                  type="button"
                  class="submenu-trigger"
                  role="menuitem"
                  aria-haspopup="menu"
                  aria-expanded={isSubmenuOpen(subKey)}
                  onclick={(e) => {
                    e.stopPropagation();
                    openSubmenuPath = isSubmenuOpen(subKey) ? null : subKey;
                  }}
                >
                  <span class="check" aria-hidden="true"></span>
                  <span class="label">{labelOf(child)}</span>
                  <span class="caret" aria-hidden="true">▸</span>
                </button>
                {#if isSubmenuOpen(subKey)}
                  <div class="submenu" role="menu">
                    {#each child.children as gc, gi (gi)}
                      {#if gc.kind === "separator"}
                        <div class="sep" role="separator"></div>
                      {:else if gc.kind === "item"}
                        {@const gcChecked = isItemChecked(gc)}
                        <button
                          type="button"
                          class="item"
                          role={gc.radio ? "menuitemradio" : gc.checkable ? "menuitemcheckbox" : "menuitem"}
                          aria-checked={gc.radio || gc.checkable ? gcChecked : undefined}
                          aria-keyshortcuts={gc.accelerator}
                          onmouseenter={() => onItemEnter(gc, child.hint ?? root.hint)}
                          onfocus={() => onItemEnter(gc, child.hint ?? root.hint)}
                          onclick={(e) => {
                            e.stopPropagation();
                            void fire(gc);
                          }}
                        >
                          <span class="check" aria-hidden="true">{gcChecked ? "✓" : ""}</span>
                          <span class="label">{labelOf(gc)}</span>
                          {#if gc.accelerator}
                            <span class="accel">{gc.accelerator}</span>
                          {/if}
                        </button>
                      {/if}
                    {/each}
                  </div>
                {/if}
              </div>
            {:else}
              {@const childChecked = isItemChecked(child)}
              {@const childEnabled = isItemEnabled(child)}
              <button
                type="button"
                class="item"
                class:disabled={!childEnabled}
                disabled={!childEnabled}
                role={child.radio ? "menuitemradio" : child.checkable ? "menuitemcheckbox" : "menuitem"}
                aria-checked={child.radio || child.checkable ? childChecked : undefined}
                aria-disabled={!childEnabled}
                aria-keyshortcuts={child.accelerator}
                onmouseenter={() => onItemEnter(child, root.hint)}
                onfocus={() => onItemEnter(child, root.hint)}
                onclick={(e) => {
                  e.stopPropagation();
                  if (!childEnabled) return;
                  void fire(child);
                }}
              >
                <span class="check" aria-hidden="true">{childChecked ? "✓" : ""}</span>
                <span class="label">{labelOf(child)}</span>
                {#if child.accelerator}
                  <span class="accel">{child.accelerator}</span>
                {/if}
              </button>
            {/if}
          {/each}
        </div>
      {/if}
    </div>
  {/each}
  <span class="grow"></span>
  <BookmarksDropdown />
</nav>

<style>
  .menubar {
    display: flex;
    align-items: stretch;
    background: var(--bg-surface);
    border-bottom: 1px solid var(--border);
    height: 32px;
    user-select: none;
  }
  .grow {
    flex: 1;
  }
  .root {
    position: relative;
  }
  .root-btn {
    height: 100%;
    padding: 0 12px;
    background: transparent;
    border: 0;
    color: var(--text-primary);
    font-size: 13px;
    cursor: pointer;
  }
  .root-btn:hover,
  .root-btn:focus-visible,
  .root.open .root-btn {
    background: var(--bg-surface-2);
  }
  .dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    min-width: 240px;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    box-shadow: 0 6px 24px rgba(0, 0, 0, 0.18);
    padding: 4px 0;
    z-index: 50;
  }
  .item,
  .submenu-trigger {
    display: flex;
    align-items: center;
    width: 100%;
    height: 28px;
    padding: 0 12px;
    background: transparent;
    border: 0;
    color: var(--text-primary);
    font-size: 13px;
    text-align: left;
    cursor: default;
    position: relative;
  }
  .item:hover,
  .item:focus-visible,
  .submenu-trigger:hover,
  .submenu-trigger:focus-visible,
  .submenu-row.submenu-open .submenu-trigger {
    background: var(--bg-surface-2);
  }
  .item.disabled {
    color: var(--text-secondary);
    opacity: 0.55;
    pointer-events: none;
  }
  .submenu-row {
    position: relative;
  }
  .check {
    width: 14px;
    margin-right: 8px;
    color: var(--accent-cyan);
    font-size: 12px;
    text-align: center;
    flex-shrink: 0;
  }
  .label {
    flex: 1;
  }
  .accel,
  .caret {
    color: var(--text-secondary);
    font-size: 11px;
    margin-left: 12px;
  }
  .submenu {
    position: absolute;
    left: 100%;
    top: 0;
    min-width: 240px;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    box-shadow: 0 6px 24px rgba(0, 0, 0, 0.18);
    padding: 4px 0;
  }
  .sep {
    height: 1px;
    margin: 4px 0;
    background: var(--border);
  }
</style>
