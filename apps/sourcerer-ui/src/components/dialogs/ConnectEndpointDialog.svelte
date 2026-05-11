<script lang="ts">
  import { settingsStore } from "../../lib/stores/settings.svelte";

  interface Props {
    open: boolean;
    onClose: () => void;
  }
  let { open, onClose }: Props = $props();

  let host = $state("");
  let port = $state(21);
  let username = $state("");
  let password = $state("");
  let linkType = $state("\\\\Server\\C");

  const canSubmit = $derived(host.trim().length > 0 && Number.isFinite(port) && port > 0);

  async function submit() {
    if (!canSubmit) return;
    // Persist as the active endpoint. `kind` is what the menu's
    // "Disconnect" gate reads — anything ≠ "local" enables it.
    await settingsStore.patch({
      endpoint: {
        name: host,
        kind: "ftp",
      },
    });
    onClose();
  }

  function reset() {
    host = "";
    port = 21;
    username = "";
    password = "";
    linkType = "\\\\Server\\C";
  }

  function close() {
    reset();
    onClose();
  }
</script>

{#if open}
  <div
    class="backdrop"
    role="presentation"
    onclick={close}
    onkeydown={(e) => e.key === "Escape" && close()}
  >
    <div
      class="modal"
      role="dialog"
      aria-modal="true"
      aria-labelledby="connect-endpoint-title"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <h2 id="connect-endpoint-title">Connect To FTP Server</h2>
      <form
        onsubmit={(e) => {
          e.preventDefault();
          void submit();
        }}
      >
        <label>
          <span>Host:</span>
          <input type="text" bind:value={host} autocomplete="off" />
        </label>
        <label>
          <span>Port:</span>
          <input type="number" min="1" max="65535" bind:value={port} />
        </label>
        <label>
          <span>Username:</span>
          <input type="text" bind:value={username} autocomplete="off" />
        </label>
        <label>
          <span>Password:</span>
          <input type="password" bind:value={password} autocomplete="off" />
        </label>
        <label>
          <span>Link type:</span>
          <select bind:value={linkType}>
            <option value="\\\\Server\\C">{"\\\\Server\\C"}</option>
            <option value="C:\\">C:\</option>
            <option value="/">/</option>
          </select>
        </label>
        <footer>
          <button type="submit" class="primary" disabled={!canSubmit}>OK</button>
          <button type="button" onclick={close}>Cancel</button>
        </footer>
      </form>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    display: grid;
    place-items: center;
    z-index: 100;
  }
  .modal {
    width: 420px;
    padding: 20px 24px 16px;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 10px;
    box-shadow: 0 12px 48px rgba(0, 0, 0, 0.4);
    color: var(--text-primary);
  }
  h2 {
    margin: 0 0 16px;
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
  }
  form {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  label {
    display: grid;
    grid-template-columns: 80px 1fr;
    align-items: center;
    gap: 10px;
    font-size: 13px;
  }
  label > span {
    color: var(--text-secondary);
    text-align: right;
  }
  input,
  select {
    padding: 6px 8px;
    background: var(--bg-canvas);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    font-size: 13px;
    font-family: inherit;
  }
  input:focus,
  select:focus {
    outline: none;
    border-color: var(--accent-cyan);
  }
  footer {
    margin-top: 12px;
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }
  footer button {
    padding: 6px 18px;
    background: var(--bg-surface-2);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    cursor: pointer;
    font-size: 13px;
  }
  footer button.primary {
    background: var(--accent-cyan);
    border-color: var(--accent-cyan);
    color: var(--bg-canvas);
  }
  footer button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
