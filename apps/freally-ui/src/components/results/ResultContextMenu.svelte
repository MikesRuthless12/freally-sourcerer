<script lang="ts">
  // Right-click menu on a result row. Build 1 introduces it because
  // SRC-M04/M05/M06 all describe verbs that belong on a result, and
  // there was nowhere to put them: the app had a menu bar and keyboard
  // shortcuts, but no per-row menu.
  import type { AppHandler, CustomCommand, QuoteStyle } from "../../lib/ipc/types";
  import * as files from "../../lib/ipc/files";
  import * as shell from "../../lib/ipc/shell_verbs";
  import { contentViewStore } from "../../lib/stores/content_view.svelte";
  import { contextMenuStore } from "../../lib/stores/context_menu.svelte";
  import { customCommandsStore } from "../../lib/stores/custom_commands.svelte";
  import { selectionStore } from "../../lib/stores/selection.svelte";
  import { toastStore } from "../../lib/stores/toast.svelte";
  import { t } from "../../lib/i18n/t";

  let root = $state<HTMLDivElement | null>(null);
  let handlers = $state<AppHandler[]>([]);
  let handlersLoaded = $state(false);

  const target = $derived(contextMenuStore.target);
  /** Paths the verbs act on: the whole selection when the clicked row
   *  is part of it, otherwise just the clicked row. Matches how every
   *  file manager resolves a right-click. */
  const targetPaths = $derived.by(() => {
    if (!target) return [];
    const selected = selectionStore.paths();
    return selected.includes(target.path) && selected.length > 1 ? selected : [target.path];
  });

  // Pure function of the target — derived rather than assigned inside
  // the effect below, so editing a command in Settings while the menu
  // is open doesn't leave a stale list.
  const commands = $derived<CustomCommand[]>(
    target ? customCommandsStore.forPath(target.path) : []
  );

  // Enumerating handlers is a shell round-trip, so it waits until the
  // menu is actually open for a specific file.
  $effect(() => {
    const path = target?.path;
    handlersLoaded = false;
    handlers = [];
    if (!path) return;
    let cancelled = false;
    shell
      .openWithCandidates(path)
      .then((list) => {
        if (!cancelled) {
          handlers = list;
          handlersLoaded = true;
        }
      })
      .catch(() => {
        if (!cancelled) handlersLoaded = true;
      });
    return () => {
      cancelled = true;
    };
  });

  $effect(() => {
    if (target && root) root.focus();
  });

  async function guard(label: string, fn: () => Promise<void>) {
    contextMenuStore.close();
    try {
      await fn();
    } catch (e) {
      toastStore.error(t("verb-failed", { verb: label, error: String(e) }));
    }
  }

  function copyList(style: QuoteStyle) {
    return guard(t("ctx-copy-path-list"), async () => {
      await shell.copyPathList(targetPaths, style);
      toastStore.show(t("toast-copied-paths", { count: targetPaths.length }));
    });
  }

  function onKeydown(ev: KeyboardEvent) {
    if (ev.key === "Escape") {
      ev.preventDefault();
      contextMenuStore.close();
    }
  }
</script>

<svelte:window
  onresize={() => contextMenuStore.close()}
  onblur={() => contextMenuStore.close()}
/>

{#if target}
  <!-- Click-away layer. Pointer-only: Escape on the menu itself closes
       it for keyboard users, so this needs no interactive role. -->
  <div
    class="scrim"
    role="presentation"
    onpointerdown={() => contextMenuStore.close()}
    oncontextmenu={(e) => {
      e.preventDefault();
      contextMenuStore.close();
    }}
  ></div>

  <div
    bind:this={root}
    class="menu"
    role="menu"
    tabindex="-1"
    aria-label={t("ctx-menu-label")}
    style="left: {contextMenuStore.x}px; top: {contextMenuStore.y}px;"
    onkeydown={onKeydown}
  >
    <button
      type="button"
      role="menuitem"
      onclick={() => guard(t("ctx-open"), () => files.open(target.path))}
    >
      {t("ctx-open")}
    </button>
    <button
      type="button"
      role="menuitem"
      onclick={() => guard(t("ctx-reveal"), () => files.reveal(target.path))}
    >
      {t("ctx-reveal")}
    </button>

    <!-- SRC-M01 -->
    <button
      type="button"
      role="menuitem"
      onclick={() =>
        guard(t("ctx-view-hits"), () => contentViewStore.show(target.path, target.name))}
    >
      {t("ctx-view-hits")}
    </button>

    <div class="sep" role="separator"></div>

    <!-- SRC-M04 -->
    <div class="group" role="group" aria-label={t("ctx-open-with")}>
      <span class="group-title">{t("ctx-open-with")}</span>
      {#if !handlersLoaded}
        <span class="hint">{t("ctx-loading")}</span>
      {:else if handlers.length === 0}
        <span class="hint">{t("ctx-open-with-none")}</span>
      {:else}
        {#each handlers as h (h.id)}
          <button
            type="button"
            role="menuitem"
            onclick={() => guard(h.name, () => shell.openWith(target.path, h.id))}
          >
            {h.name}
          </button>
        {/each}
      {/if}
    </div>

    <div class="sep" role="separator"></div>

    <!-- SRC-M05 -->
    <button
      type="button"
      role="menuitem"
      onclick={() =>
        guard(t("ctx-copy-contents"), async () => {
          await shell.copyFileContents(target.path);
          toastStore.show(t("toast-copied-contents", { name: target.name }));
        })}
    >
      {t("ctx-copy-contents")}
    </button>
    <button
      type="button"
      role="menuitem"
      onclick={() =>
        guard(t("ctx-copy-as-file"), async () => {
          await shell.copyFilesAsObjects(targetPaths);
          toastStore.show(t("toast-copied-files", { count: targetPaths.length }));
        })}
    >
      {t("ctx-copy-as-file")}
    </button>
    <div class="group" role="group" aria-label={t("ctx-copy-path-list")}>
      <span class="group-title">{t("ctx-copy-path-list")}</span>
      <button type="button" role="menuitem" onclick={() => copyList("lines")}>
        {t("ctx-quote-lines")}
      </button>
      <button type="button" role="menuitem" onclick={() => copyList("quoted")}>
        {t("ctx-quote-quoted")}
      </button>
      <button type="button" role="menuitem" onclick={() => copyList("space_separated")}>
        {t("ctx-quote-space")}
      </button>
      <button type="button" role="menuitem" onclick={() => copyList("escaped")}>
        {t("ctx-quote-escaped")}
      </button>
    </div>

    <div class="sep" role="separator"></div>

    <!-- SRC-M06 -->
    <button
      type="button"
      role="menuitem"
      onclick={() => guard(t("ctx-terminal-here"), () => shell.openTerminalHere(target.path))}
    >
      {t("ctx-terminal-here")}
    </button>
    {#each commands as c (c.id)}
      <button
        type="button"
        role="menuitem"
        onclick={() => guard(c.name, () => shell.runCustomCommand(c.id, target.path))}
      >
        {c.name || c.program}
      </button>
    {/each}
  </div>
{/if}

<style>
  .scrim {
    position: fixed;
    inset: 0;
    z-index: 70;
  }
  .menu {
    position: fixed;
    z-index: 71;
    min-width: 220px;
    max-width: 320px;
    max-height: 70vh;
    overflow-y: auto;
    padding: 4px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg-surface-2);
    box-shadow: 0 8px 24px rgb(0 0 0 / 40%);
  }
  .menu:focus-visible {
    outline: 2px solid var(--accent-cyan);
    outline-offset: -2px;
  }
  .menu button {
    display: block;
    width: 100%;
    text-align: left;
    background: transparent;
    border: 0;
    color: var(--text-primary);
    font-size: 12px;
    padding: 5px 10px;
    border-radius: 4px;
    cursor: pointer;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .menu button:hover,
  .menu button:focus-visible {
    background: var(--bg-surface);
    outline: none;
  }
  .sep {
    height: 1px;
    margin: 4px 6px;
    background: var(--border);
  }
  .group {
    display: block;
  }
  .group-title,
  .hint {
    display: block;
    padding: 4px 10px;
    font-size: 11px;
    color: var(--text-secondary);
  }
  .group button {
    padding-left: 20px;
  }
</style>
