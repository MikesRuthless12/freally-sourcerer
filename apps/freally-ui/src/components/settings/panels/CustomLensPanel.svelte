<script lang="ts">
  import { customExtractorsStore } from "../../../lib/stores/custom_extractors.svelte";
  import { t } from "../../../lib/i18n/t";
  import Section from "../controls/Section.svelte";
  import Checkbox from "../controls/Checkbox.svelte";

  let busyId = $state<string | null>(null);

  async function setTrust(id: string, on: boolean) {
    busyId = id;
    try {
      await customExtractorsStore.setTrusted(id, on);
    } finally {
      busyId = null;
    }
  }

  async function refreshHashes() {
    await customExtractorsStore.refreshHashes();
  }
</script>

<h1>{t("settings-tree-custom-lens")}</h1>
<p class="hint">Wasm-sandboxed community extractors. Untrusted by default — flip the trust
toggle on each row before the host loads the sidecar.</p>

<Section title={t("settings-custom-registry")}>
  <button type="button" onclick={refreshHashes}>{t("settings-custom-refresh-hashes")}</button>
  {#if customExtractorsStore.list.length === 0}
    <p class="muted">No custom extractors installed. Drop a TOML manifest + .wasm sidecar into
      the index root's <code>extractors/</code> directory and re-open this panel.</p>
  {:else}
    <ul>
      {#each customExtractorsStore.list as e (e.id)}
        <li>
          <header>
            <strong>{e.display_name}</strong>
            <span class="ver">v{e.version}</span>
            <span class="hash">{e.hash_blake3.slice(0, 12)}</span>
          </header>
          <div class="meta">
            Formats: {e.formats.join(", ") || "(none)"} · Time {e.time_budget_ms} ms · Memory {e.memory_budget_mb} MiB
          </div>
          <div class="meta sandbox">
            Sandbox: network={e.sandbox_view.network ? "yes" : "no"},
            fs-write={e.sandbox_view.filesystem_write ? "yes" : "no"},
            clock={e.sandbox_view.clock ? "yes" : "no"}
          </div>
          <Checkbox id={`ce-trust-${e.id}`} label={t("settings-custom-trust")}
            checked={e.trusted} disabled={busyId === e.id}
            onChange={(v) => setTrust(e.id, v)} />
        </li>
      {/each}
    </ul>
  {/if}
</Section>

<style>
  h1 { margin: 0 0 4px; font-size: 18px; color: var(--text-primary); }
  .hint { margin: 0 0 16px; color: var(--text-secondary); font-size: 12px; line-height: 1.5; }
  ul { list-style: none; padding: 0; margin: 8px 0 0; }
  li { padding: 8px 12px; border: 1px solid var(--border); border-radius: 4px; margin-bottom: 6px; color: var(--text-primary); font-size: 13px; }
  header { display: flex; align-items: baseline; gap: 8px; margin-bottom: 4px; }
  .ver { color: var(--text-secondary); font-size: 11px; }
  .hash { font-family: var(--font-mono); color: var(--text-secondary); font-size: 10px; }
  .meta { color: var(--text-secondary); font-size: 12px; }
  .sandbox { font-family: var(--font-mono); }
  button { padding: 4px 10px; background: var(--bg-canvas); border: 1px solid var(--border); color: var(--text-primary); border-radius: 3px; cursor: pointer; font: inherit; margin-bottom: 8px; }
  .muted { color: var(--text-secondary); }
  code { background: var(--bg-canvas); border: 1px solid var(--border); border-radius: 2px; padding: 0 4px; font-family: var(--font-mono); font-size: 11px; }
</style>
