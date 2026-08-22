<script lang="ts">
  import { settingsStore } from "../../../lib/stores/settings.svelte";
  import { settingsDialog } from "../../../lib/stores/settings_dialog.svelte";
  import Section from "../controls/Section.svelte";
  import Dropdown from "../controls/Dropdown.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import type { PrivacyAndUpdatesSettings } from "../../../lib/ipc/types";
  import { t } from "../../../lib/i18n/t";
  import {
    check as checkForUpdate,
    confirmAndInstall,
    lastCheckedUnixSecs,
    type UpdateCheck
  } from "../../../lib/stores/updates.svelte";
  import { parseReleaseNotes, isOpenableUrl } from "../../../lib/util/release_notes";
  import { formatDateMs } from "../../../lib/util/format";

  function patch(p: Partial<PrivacyAndUpdatesSettings>) {
    settingsStore.patch({ privacy_and_updates: { ...settingsStore.state.privacy_and_updates, ...p } });
    settingsDialog.markDirty("privacy");
  }

  // TASK-UP1. The updater plugin has been signed and verified since
  // TASK-103 and nothing ever asked it a question — Help → Check for
  // Updates opened a web page. This is the surface that asks.
  let checking = $state(false);
  let result = $state<UpdateCheck | null>(null);
  let error = $state("");
  let installing = $state(false);
  let lastChecked = $state<number | null>(lastCheckedUnixSecs());

  const notes = $derived(result?.notes ? parseReleaseNotes(result.notes) : []);

  async function checkNow() {
    checking = true;
    error = "";
    try {
      result = await checkForUpdate();
      lastChecked = result.checkedAtUnixSecs;
      if (result.isNewer) await offerInstall();
    } catch (e) {
      // Loud on purpose. The updater fails closed and silent: an
      // unreachable endpoint and "you are current" are the same answer
      // from inside the plugin, and that is exactly what hid the
      // destroyed signing key from every 0.23.1 install.
      error = t("updates-failed", { error: String(e) });
      result = null;
    } finally {
      checking = false;
    }
  }

  async function offerInstall() {
    // The box names both versions and carries the head of the release
    // notes. Installing replaces the running application and closes it —
    // never something to do because somebody clicked "check".
    const r = result;
    if (!r?.isNewer) return;
    try {
      installing = true;
      installing = await confirmAndInstall(r);
    } catch (e) {
      installing = false;
      error = t("updates-failed", { error: String(e) });
    }
  }

  // The app's existing way to open a link (see `AboutPanel`). The notes
  // are remote text, so `isOpenableUrl` — http(s) only — is the gate;
  // routing them through the bug reporter's opener would have handed
  // them `mailto:` permission they should never have.
  async function openLink(href: string) {
    if (!isOpenableUrl(href)) return;
    try {
      const opener = await import("@tauri-apps/plugin-opener");
      await opener.openUrl(href);
    } catch (e) {
      console.warn("[updates] could not open link", e);
    }
  }

  // The app's own formatter, which the tests pin against — and which
  // produces the fixed-width shape the mono cell below is sized for.
  function formatChecked(secs: number | null): string {
    return secs ? formatDateMs(secs * 1000) : t("updates-never-checked");
  }
</script>

<h1>{t("settings-group-privacy")}</h1>

<Section title={t("section-auto-update")}>
  <Dropdown id="pu-au" label={t("settings-privacy-auto-update")}
    value={settingsStore.state.privacy_and_updates.auto_update}
    options={[ { value: "default", label: t("opt-on-default") }, { value: "weekly", label: t("opt-weekly") }, { value: "monthly", label: t("opt-monthly") }, { value: "off", label: t("opt-off") } ]}
    onChange={(v) => patch({ auto_update: v })} />
  <Checkbox id="pu-pre" label={t("settings-privacy-prerelease")}
    checked={settingsStore.state.privacy_and_updates.pre_release_channel}
    onChange={(v) => patch({ pre_release_channel: v })} />

  <div class="checkrow">
    <span class="label">{t("updates-last-checked")}</span>
    <span class="value">{formatChecked(lastChecked)}</span>
  </div>

  <button type="button" onclick={checkNow} disabled={checking || installing}>
    {checking ? t("updates-checking") : t("updates-check-now")}
  </button>

  {#if installing}
    <p class="status">{t("updates-installing", { version: result!.availableVersion })}</p>
  {:else if error}
    <p class="status error">{error}</p>
  {:else if result && !result.isNewer}
    <p class="status">{t("updates-current", { current: result.currentVersion })}</p>
  {:else if result?.isNewer}
    <p class="status">
      {t("updates-available", { version: result.availableVersion, current: result.currentVersion })}
    </p>
    <button type="button" onclick={offerInstall}>{t("updates-confirm-title")}</button>
  {/if}

  {#if notes.length > 0}
    <p class="notes-heading">{t("updates-notes-heading")}</p>
    <!-- Segments, not `{@html}`. The notes are remote text from the
         release manifest; rendering them through normal interpolation
         means there is no sanitiser to get wrong, because no markup is
         ever produced. -->
    <p class="notes">{#each notes as seg}{#if seg.kind === "text"}{seg.value}{:else}<a
          href={seg.href}
          onclick={(e) => { e.preventDefault(); void openLink(seg.href); }}>{seg.href}</a>{/if}{/each}</p>
  {/if}
</Section>

<Section title={t("section-privacy")}>
  <p class="muted">There is <strong>no telemetry and no analytics</strong> in Freally, and no
  toggle for them, per PRD §8.23 — nothing about your searches, files, or usage is collected
  or transmitted, ever.</p>
  <p class="muted">Bug reports are <strong>opt-in and manual</strong>: Help → Report a Bug shows
  you the exact anonymous text (app version and OS only) and opens a draft you read and send
  yourself. Nothing is sent automatically and there is no server we run.</p>
</Section>

<Section title={t("settings-privacy-network-policy")}>
  <ul class="urls">
    {#if settingsStore.state.privacy_and_updates.auto_update !== "off"}
      <li><code>github.com</code> — the signed update manifest and installer</li>
    {:else}
      <li class="muted">No outbound URLs — auto-update is off.</li>
    {/if}
  </ul>
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  ul.urls { list-style: none; padding: 0; margin: 0; }
  ul.urls li { padding: 4px 8px; border-bottom: 1px solid var(--border); color: var(--text-primary); font-size: 13px; }
  .muted { color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
  code { font-family: var(--font-mono); background: var(--bg-canvas); border: 1px solid var(--border); border-radius: 2px; padding: 0 4px; font-size: 11px; }
  strong { color: var(--text-primary); }
  button { padding: 5px 12px; background: var(--bg-canvas); color: var(--text-primary); border: 1px solid var(--border); border-radius: 3px; cursor: pointer; font: inherit; margin-top: 8px; margin-right: 8px; }
  button:disabled { opacity: 0.55; cursor: default; }
  .checkrow { display: flex; gap: 12px; align-items: baseline; margin-top: 10px; font-size: 12px; }
  .checkrow .label { color: var(--text-secondary); }
  .checkrow .value { color: var(--text-primary); font-family: var(--font-mono); font-size: 11px; }
  .status { margin: 10px 0 0; font-size: 12px; color: var(--text-primary); }
  .status.error { color: var(--accent-orange); }
  .notes-heading { margin: 12px 0 4px; font-size: 12px; color: var(--text-secondary); }
  .notes { margin: 0; font-size: 12px; line-height: 1.6; color: var(--text-primary); white-space: pre-wrap; overflow-wrap: anywhere; }
  .notes a { color: var(--accent-cyan); }
</style>
