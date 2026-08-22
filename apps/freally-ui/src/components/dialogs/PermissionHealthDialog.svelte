<script lang="ts">
  // SRC-M21 — what search cannot see, and how to fix it.
  //
  // Until now a subtree the scanner could not read looked exactly like a
  // subtree that was empty: no result, no explanation, one warn line in
  // a log nobody opens. This is the drill-down — which folders, on which
  // volume, for which reason — plus the fix for the OS the *index* runs
  // on (which is not necessarily the one rendering this dialog, hence
  // the daemon deciding).
  import Modal from "../common/Modal.svelte";

  import * as indexApi from "../../lib/ipc/index_api";
  import type { PermissionReport } from "../../lib/ipc/index_api";
  import { t } from "../../lib/i18n/t";

  interface Props {
    open: boolean;
    onClose: () => void;
  }
  let { open, onClose }: Props = $props();

  let report = $state<PermissionReport | null>(null);
  let error = $state<string | null>(null);
  let expanded = $state<Set<string>>(new Set());

  $effect(() => {
    if (!open) return;
    let cancelled = false;
    void (async () => {
      try {
        const r = await indexApi.permissions();
        if (!cancelled) {
          report = r;
          error = null;
        }
      } catch (e) {
        if (!cancelled) {
          error = String(e);
          report = null;
        }
      }
    })();
    return () => {
      cancelled = true;
    };
  });

  function toggle(volume: string) {
    const next = new Set(expanded);
    if (next.has(volume)) next.delete(volume);
    else next.add(volume);
    expanded = next;
  }

  /** macOS: jump straight to the Full Disk Access pane. */
  async function openFullDiskAccess() {
    try {
      const opener = await import("@tauri-apps/plugin-opener");
      await opener.openUrl(
        "x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles"
      );
    } catch (e) {
      console.warn("[permissions] could not open the settings pane", e);
    }
  }
</script>

<Modal
  {open}
  {onClose}
  label={t("permissions-title")}
  testId="permission-health"
  style="display: flex; flex-direction: column; width: min(720px, 92vw); max-height: 82vh; overflow: hidden;"
>
  <header>
    <strong>{t("permissions-title")}</strong>
    <button type="button" class="close" onclick={onClose} aria-label={t("about-close")}>×</button
    >
  </header>

  <div class="body">
    {#if error}
      <p class="hint">{error}</p>
    {:else if !report}
      <p class="hint">{t("preview-loading")}</p>
    {:else if report.denied === 0 && report.other === 0}
      <p class="ok">{t("permissions-all-clear")}</p>
    {:else}
      <p class="summary">
        {t("permissions-summary", { denied: report.denied, other: report.other })}
      </p>
      {#if report.dropped > 0}
        <!-- Never imply the list is complete when it is not. -->
        <p class="hint">{t("permissions-truncated", { dropped: report.dropped })}</p>
      {/if}

      <section class="fix">
        <h3>{t("permissions-how-to-fix")}</h3>
        {#if report.guidance === "macos_full_disk_access"}
          {#if report.full_disk_access === true}
            <p class="ok">{t("permissions-macos-granted")}</p>
          {:else}
            <p>{t("permissions-macos-steps")}</p>
            <button type="button" class="primary" onclick={openFullDiskAccess}>
              {t("permissions-macos-open-settings")}
            </button>
            {#if report.full_disk_access === null}
              <p class="hint">{t("permissions-macos-undetermined")}</p>
            {/if}
          {/if}
        {:else if report.guidance === "linux_permissions"}
          <p>{t("permissions-linux-steps")}</p>
        {:else if report.guidance === "windows_acl"}
          <p>{t("permissions-windows-steps")}</p>
        {:else}
          <p>{t("permissions-generic-steps")}</p>
        {/if}
      </section>

      <section class="list">
        <h3>{t("permissions-by-volume")}</h3>
        {#each report.by_volume as g (g.volume)}
          <div class="group">
            <button type="button" class="group-head" onclick={() => toggle(g.volume)}>
              <span class="chev">{expanded.has(g.volume) ? "▾" : "▸"}</span>
              <span class="vol">{g.volume}</span>
              <span class="counts"
                >{t("permissions-group-counts", { denied: g.denied, other: g.other })}</span
              >
            </button>
            {#if expanded.has(g.volume)}
              <ul>
                {#each g.entries as e (e.path)}
                  <li>
                    <span class="path">{e.path}</span>
                    <span class="detail">{e.detail}</span>
                  </li>
                {/each}
              </ul>
            {/if}
          </div>
        {/each}
      </section>
    {/if}
  </div>
</Modal>

<style>
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 14px;
    background: var(--bg-surface-2);
    border-bottom: 1px solid var(--border);
    color: var(--text-primary);
    font-size: 13px;
  }
  .close {
    background: none;
    border: 0;
    color: var(--text-secondary);
    font-size: 20px;
    line-height: 1;
    cursor: pointer;
  }
  .body {
    overflow: auto;
    padding: 14px;
  }
  p {
    margin: 0 0 8px;
    color: var(--text-primary);
    font-size: 13px;
    line-height: 1.5;
  }
  .hint {
    color: var(--text-secondary);
    font-size: 12px;
  }
  .ok {
    color: var(--success);
  }
  .summary {
    font-weight: 600;
  }
  h3 {
    margin: 14px 0 6px;
    color: var(--text-secondary);
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }
  .primary {
    padding: 5px 12px;
    border: 1px solid var(--accent-cyan);
    border-radius: 4px;
    background: color-mix(in srgb, var(--accent-cyan) 20%, transparent);
    color: var(--text-primary);
    font-size: 12px;
    cursor: pointer;
  }
  .group {
    border: 1px solid var(--border);
    border-radius: 4px;
    margin-bottom: 6px;
    overflow: hidden;
  }
  .group-head {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 6px 10px;
    border: 0;
    background: var(--bg-surface-2);
    color: var(--text-primary);
    font: 12px/1.4 inherit;
    text-align: left;
    cursor: pointer;
  }
  .chev {
    color: var(--text-secondary);
  }
  .vol {
    flex: 1;
    font-family: var(--font-mono);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .counts {
    color: var(--text-secondary);
    font-size: 11px;
  }
  ul {
    list-style: none;
    margin: 0;
    padding: 4px 0;
    max-height: 260px;
    overflow: auto;
  }
  li {
    display: flex;
    flex-direction: column;
    padding: 3px 10px 3px 26px;
  }
  .path {
    font-family: var(--font-mono);
    font-size: 12px;
    color: var(--text-primary);
    word-break: break-all;
  }
  .detail {
    font-size: 11px;
    color: var(--text-secondary);
  }
</style>
