<script lang="ts">
  import { networkStore } from "../../../lib/stores/network.svelte";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";
  import NumberInput from "../controls/NumberInput.svelte";
  import { t } from "../../../lib/i18n/t";

  let busy = $state(false);
  let toast = $state("");

  async function startStop() {
    if (busy) return;
    busy = true;
    toast = "";
    try {
      if (networkStore.status.api_running) {
        await networkStore.stopApi();
        toast = "API server stopped";
      } else {
        await networkStore.startApi();
        toast = "API server started";
      }
    } catch (e) {
      toast = `Failed: ${e}`;
    } finally {
      busy = false;
      setTimeout(() => (toast = ""), 3000);
    }
  }
</script>

<h1>{t("settings-node-etp-api")}</h1>
<p class="hint">Modern HTTPS+token JSON API replacing voidtools-Everything's FTP/ETP server.</p>

<Section title={t("section-api-server")}>
  <Checkbox id="api-running" label={t("settings-net-api-enable")}
    checked={networkStore.status.api_running} disabled={busy}
    onChange={() => startStop()} />
  <NumberInput id="api-port" label={t("settings-net-port")} min={1} max={65535}
    value={networkStore.desiredApiPort}
    onChange={(n) => (networkStore.desiredApiPort = n)} />
  <Checkbox id="api-legacy-ftp" label={t("settings-net-legacy-ftp")}
    checked={networkStore.legacyFtp}
    onChange={(v) => (networkStore.legacyFtp = v)} />
  {#if toast}<p class="toast">{toast}</p>{/if}
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  .hint { margin: 0 0 16px; color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
  .toast { margin-top: 8px; color: var(--accent-cyan); font-size: 12px; }
</style>
