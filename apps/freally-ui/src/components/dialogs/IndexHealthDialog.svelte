<script lang="ts">
  // SRC-M13 — Tools → Index Health. Shows, per watched root, how far
  // behind live changes the index is, what it dropped, and what to do
  // about it.
  import { untrack } from "svelte";
  import Modal from "../common/Modal.svelte";
  import { indexHealthStore } from "../../lib/stores/index_health.svelte";
  import { t } from "../../lib/i18n/t";
  import type { HealthAdvisory, VolumeHealth } from "../../lib/ipc/types";

  interface Props {
    open: boolean;
    onClose: () => void;
  }
  let { open, onClose }: Props = $props();

  // Polling costs an RPC every 2s, so it only runs while the panel is up.
  //
  // `untrack` so this effect depends on `open` and nothing else — the same
  // guard `SettingsDialog` uses around `settingsDialog.openDialog`. The store
  // no longer reads reactive state in `start()`, so this is belt and braces;
  // it is here because an effect whose cleanup stops a poller and whose body
  // starts one must never be able to re-run on that poller's own output.
  $effect(() => {
    if (!open) return;
    untrack(() => indexHealthStore.start());
    return () => indexHealthStore.stop();
  });

  const health = $derived(indexHealthStore.health);

  function lag(v: VolumeHealth): string {
    if (!v.monitoring) return "—";
    if (v.last_event_ms === 0) return t("health-no-events-yet");
    return t("health-lag-value", { last: v.last_lag_ms, max: v.max_lag_ms });
  }

  function lastEvent(v: VolumeHealth): string {
    if (v.last_event_ms === 0) return t("health-never");
    const secs = Math.max(0, Math.round((Date.now() - v.last_event_ms) / 1000));
    return t("health-ago", { secs });
  }

  function advisoryText(a: HealthAdvisory): string {
    return t(`health-advice-${a.id.replace(/_/g, "-")}`, {
      root: a.root ?? "",
      count: a.count,
    });
  }
</script>

<Modal
  {open}
  {onClose}
  labelledBy="index-health-title"
  style="width: 720px; max-width: 92vw; max-height: 82vh; overflow-y: auto; padding: 20px 24px 16px;"
>
  <h2 id="index-health-title">{t("health-title")}</h2>

  {#if indexHealthStore.error}
    <p class="error">{indexHealthStore.error}</p>
  {:else if indexHealthStore.loading}
    <p class="muted">{t("health-loading")}</p>
  {:else if health}
    {#if health.advisories.length > 0}
      <ul class="advisories">
        {#each health.advisories as a (a.id + (a.root ?? ""))}
          <li class={a.severity}>
            <span class="advice">{advisoryText(a)}</span>
            {#if a.fix === "rebuild_index"}
              <button
                type="button"
                class="fix"
                disabled={indexHealthStore.fixing}
                onclick={() => void indexHealthStore.applyFix(a.fix)}
              >
                {t("health-fix-rebuild")}
              </button>
            {/if}
          </li>
        {/each}
      </ul>
    {:else}
      <p class="ok">{t("health-all-good")}</p>
    {/if}

    {#if health.volumes.length === 0}
      <p class="muted">{t("health-no-watched-roots")}</p>
    {:else}
      <table>
        <thead>
          <tr>
            <th>{t("health-col-volume")}</th>
            <th>{t("health-col-status")}</th>
            <th>{t("health-col-lag")}</th>
            <th>{t("health-col-last-event")}</th>
            <th>{t("health-col-applied")}</th>
            <th>{t("health-col-dropped")}</th>
            <th>{t("health-col-queue")}</th>
          </tr>
        </thead>
        <tbody>
          {#each health.volumes as v (v.root)}
            <tr>
              <td title={v.root}>{v.label}</td>
              <td>
                {#if v.monitoring}
                  <span class="pill live">{t("health-status-live")}</span>
                {:else}
                  <span class="pill idle" title={v.unavailable_reason ?? ""}>
                    {t("health-status-scan-only")}
                  </span>
                {/if}
              </td>
              <td>{lag(v)}</td>
              <td>{lastEvent(v)}</td>
              <td>{v.events_applied} <span class="sub">({v.events_coalesced})</span></td>
              <td class={v.events_dropped > 0 ? "bad" : ""}>{v.events_dropped}</td>
              <td>{v.queue_depth}/{v.queue_capacity}</td>
            </tr>
          {/each}
        </tbody>
      </table>
      <p class="legend">{t("health-legend")}</p>
    {/if}

    <p class="muted">
      {#if health.extraction_backlog === undefined}
        {t("health-extraction-untracked")}
      {:else}
        {t("health-extraction-backlog", { count: health.extraction_backlog })}
      {/if}
    </p>
  {/if}

  <footer>
    <button type="button" onclick={onClose}>{t("close")}</button>
  </footer>
</Modal>

<style>
  h2 {
    margin: 0 0 16px;
    font-size: 16px;
    font-weight: 600;
  }
  .muted {
    color: var(--text-secondary);
    font-size: 12px;
  }
  .error {
    color: var(--danger, #d94040);
    font-size: 13px;
  }
  .ok {
    font-size: 13px;
    color: var(--text-secondary);
  }
  .advisories {
    list-style: none;
    margin: 0 0 16px;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .advisories li {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 8px 10px;
    border-radius: 6px;
    font-size: 13px;
    border-left: 3px solid var(--border);
    background: var(--bg-elevated, rgba(127, 127, 127, 0.08));
  }
  .advisories li.critical {
    border-left-color: var(--danger, #d94040);
  }
  .advisories li.warning {
    border-left-color: var(--warning, #d9a040);
  }
  .advice {
    flex: 1;
  }
  .fix {
    flex: none;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 12px;
  }
  th {
    text-align: left;
    font-weight: 600;
    color: var(--text-secondary);
    padding: 4px 8px 6px 0;
    border-bottom: 1px solid var(--border);
  }
  td {
    padding: 5px 8px 5px 0;
    border-bottom: 1px solid var(--border);
  }
  td.bad {
    color: var(--danger, #d94040);
    font-weight: 600;
  }
  .sub {
    color: var(--text-secondary);
  }
  .pill {
    display: inline-block;
    padding: 1px 7px;
    border-radius: 10px;
    font-size: 11px;
  }
  .pill.live {
    background: var(--success-bg, rgba(64, 160, 96, 0.18));
  }
  .pill.idle {
    background: rgba(127, 127, 127, 0.18);
  }
  .legend {
    margin: 8px 0 0;
    font-size: 11px;
    color: var(--text-secondary);
  }
  footer {
    display: flex;
    justify-content: flex-end;
    margin-top: 16px;
  }
  /* Matches the other dialogs' footer buttons. */
  footer button,
  .fix {
    padding: 6px 18px;
    background: var(--bg-surface-2);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    cursor: pointer;
    font-size: 13px;
  }
  .fix {
    padding: 4px 12px;
    font-size: 12px;
  }
  footer button:disabled,
  .fix:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
