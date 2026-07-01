<script lang="ts">
  import { networkStore } from "../../../lib/stores/network.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import TextInput from "../controls/TextInput.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import { t } from "../../../lib/i18n/t";

  let busy = $state(false);
  let toast = $state("");

  async function startStop() {
    if (busy) return;
    busy = true;
    toast = "";
    try {
      if (networkStore.status.https_running) {
        await networkStore.stopHttps();
        toast = "HTTPS server stopped";
      } else {
        await networkStore.startHttps();
        toast = "HTTPS server started";
      }
    } catch (e) {
      toast = `Failed: ${e}`;
    } finally {
      busy = false;
      setTimeout(() => (toast = ""), 3000);
    }
  }

  async function regen() {
    if (busy) return;
    busy = true;
    try {
      await networkStore.regenToken();
      toast = "Token rotated";
    } finally {
      busy = false;
      setTimeout(() => (toast = ""), 3000);
    }
  }
</script>

<h1>HTTPS Server</h1>

<Section title={t("section-bind")}>
  <Checkbox id="ns-running" label={t("settings-net-https-enable")}
    checked={networkStore.status.https_running} disabled={busy}
    onChange={() => startStop()} />
  <TextInput id="ns-bind" label={t("settings-net-bind")}
    value={networkStore.desiredBind}
    onChange={(v) => (networkStore.desiredBind = v)} />
  <NumberInput id="ns-port" label={t("settings-net-port")} min={1} max={65535}
    value={networkStore.desiredPort}
    onChange={(n) => (networkStore.desiredPort = n)} />
  <Checkbox id="ns-force-https" label={t("settings-net-force-https")}
    checked={networkStore.forceHttps}
    onChange={(v) => (networkStore.forceHttps = v)} />
  <Checkbox id="ns-legacy" label={t("settings-net-legacy-auth")}
    checked={networkStore.legacyAuth}
    onChange={(v) => (networkStore.legacyAuth = v)} />
  {#if networkStore.status.https_token_fingerprint}
    <p class="muted">Token fingerprint: <code>{networkStore.status.https_token_fingerprint}</code></p>
  {/if}
  <button type="button" onclick={regen} disabled={busy}>{t("settings-net-token-regen")}</button>
  {#if toast}<p class="toast">{toast}</p>{/if}
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  button { padding: 5px 12px; background: var(--accent-cyan); color: var(--bg-canvas); border: 0; border-radius: 4px; cursor: pointer; font: inherit; margin-top: 6px; }
  button:disabled { opacity: 0.55; cursor: not-allowed; }
  .toast { margin-top: 8px; color: var(--accent-cyan); font-size: 12px; }
  .muted { color: var(--text-secondary); font-size: 12px; }
  code { font-family: var(--font-mono); }
</style>
