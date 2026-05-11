<script lang="ts">
  import { settingsStore } from "../../lib/stores/settings.svelte";
  import { themeStore } from "../../lib/stores/theme.svelte";
  import { foldersStore } from "../../lib/stores/folders.svelte";
  import { SUPPORTED_LOCALES } from "../../lib/i18n/bundle";

  let step = $state(0);
  let roots = $state<string[]>([]);
  let newRoot = $state("");
  let hotkey = $state(settingsStore.state.hotkey);
  let locale = $state(settingsStore.state.locale);
  let themeChoice = $state<"system" | "light" | "dark">(settingsStore.state.theme);

  function next() {
    step = Math.min(3, step + 1);
  }
  function back() {
    step = Math.max(0, step - 1);
  }
  function addRoot() {
    if (newRoot.trim()) {
      roots = [...roots, newRoot.trim()];
      newRoot = "";
    }
  }

  async function browseRoot() {
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const picked = await open({ directory: true, multiple: false });
      if (typeof picked === "string" && picked.length > 0 && !roots.includes(picked)) {
        roots = [...roots, picked];
      }
    } catch (e) {
      console.warn("[wizard] folder picker failed:", e);
    }
  }
  function removeRoot(r: string) {
    roots = roots.filter((x) => x !== r);
  }

  async function finish() {
    themeStore.set(themeChoice);
    await settingsStore.patch({
      theme: themeChoice,
      locale,
      hotkey,
      first_run_complete: true
    });
    for (const path of roots) {
      try {
        await foldersStore.add({
          id: `wizard-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
          path,
          monitor_changes: true,
          buffer_kb: 64,
          rescan_on_full_buffer: true,
          rescan_schedule: { kind: "never" }
        });
      } catch (e) {
        console.warn("[wizard] failed to register root:", path, e);
      }
    }
  }
</script>

{#if !settingsStore.state.first_run_complete && settingsStore.loaded}
  <div class="backdrop" role="presentation">
    <div class="modal" role="dialog" aria-modal="true" aria-label="First-run wizard">
      <header>
        <h2>Welcome to Sourcerer</h2>
        <span class="step">Step {step + 1} of 4</span>
      </header>

      <div class="body">
        {#if step === 0}
          <h3>Choose what to index</h3>
          <p class="hint">Add the folders or volumes you want Sourcerer to watch. You can change this later from Indexes settings.</p>
          <div class="root-add">
            <button type="button" class="primary" onclick={browseRoot}>Browse…</button>
            <input
              type="text"
              placeholder="…or paste a path"
              bind:value={newRoot}
              onkeydown={(e) => e.key === "Enter" && addRoot()}
            />
            <button type="button" onclick={addRoot}>Add</button>
          </div>
          <ul class="roots">
            {#each roots as r (r)}
              <li>
                <span class="path">{r}</span>
                <button type="button" class="remove" onclick={() => removeRoot(r)}>Remove</button>
              </li>
            {/each}
            {#if roots.length === 0}
              <li class="empty">No roots configured yet.</li>
            {/if}
          </ul>
        {:else if step === 1}
          <h3>Pick a global hotkey</h3>
          <p class="hint">Pressing this combo from anywhere brings Sourcerer to the front.</p>
          <input
            type="text"
            class="hotkey-input"
            bind:value={hotkey}
            placeholder="Win+Space"
          />
        {:else if step === 2}
          <h3>Pick your language</h3>
          <p class="hint">Sourcerer ships in 18 languages. You can switch later.</p>
          <select bind:value={locale}>
            {#each SUPPORTED_LOCALES as code (code)}
              <option value={code}>{code}</option>
            {/each}
          </select>
        {:else if step === 3}
          <h3>Pick a theme</h3>
          <p class="hint">System follows your OS appearance setting.</p>
          <div class="themes">
            {#each ["system", "light", "dark"] as id (id)}
              <button
                type="button"
                class="theme-card"
                class:active={themeChoice === id}
                onclick={() => (themeChoice = id as "system" | "light" | "dark")}
              >
                {id}
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <footer>
        <button type="button" onclick={back} disabled={step === 0}>Back</button>
        <span class="grow"></span>
        {#if step < 3}
          <button type="button" class="primary" onclick={next}>Next</button>
        {:else}
          <button type="button" class="primary" onclick={finish}>Finish</button>
        {/if}
      </footer>
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
    z-index: 200;
  }
  .modal {
    width: 540px;
    background: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 10px;
    box-shadow: 0 12px 48px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
  }
  header {
    display: flex;
    align-items: baseline;
    padding: 16px 20px 12px;
    border-bottom: 1px solid var(--border);
  }
  h2 {
    flex: 1;
    margin: 0;
    font-size: 15px;
    color: var(--text-primary);
  }
  .step {
    color: var(--text-secondary);
    font-size: 12px;
  }
  .body {
    padding: 16px 20px;
    min-height: 200px;
  }
  h3 {
    margin: 0 0 8px;
    font-size: 14px;
    color: var(--text-primary);
  }
  .hint {
    margin: 0 0 12px;
    color: var(--text-secondary);
    font-size: 12px;
  }
  .root-add {
    display: flex;
    gap: 6px;
    margin-bottom: 8px;
  }
  .root-add input {
    flex: 1;
    background: var(--bg-canvas);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    padding: 4px 8px;
    font-size: 13px;
  }
  .root-add button,
  footer button {
    padding: 4px 12px;
    background: var(--bg-surface-2);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    font-size: 13px;
    cursor: pointer;
  }
  footer button.primary {
    background: var(--accent-cyan);
    border-color: var(--accent-cyan);
    color: var(--bg-canvas);
  }
  footer button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  .roots {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  .roots li {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 0;
    border-top: 1px solid var(--border);
  }
  .roots li.empty {
    color: var(--text-secondary);
    font-style: italic;
  }
  .path {
    flex: 1;
    font-family: var(--font-mono);
    font-size: 12px;
  }
  .remove {
    background: transparent;
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--danger);
    padding: 2px 8px;
    cursor: pointer;
    font-size: 11px;
  }
  .hotkey-input {
    width: 100%;
    background: var(--bg-canvas);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    padding: 6px 10px;
    font-family: var(--font-mono);
    font-size: 13px;
  }
  select {
    width: 100%;
    background: var(--bg-canvas);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    padding: 6px 10px;
    font-size: 13px;
  }
  .themes {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }
  .theme-card {
    padding: 24px 12px;
    background: var(--bg-surface-2);
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text-primary);
    text-transform: capitalize;
    cursor: pointer;
    font-size: 13px;
  }
  .theme-card.active {
    border-color: var(--accent-cyan);
    background: color-mix(in srgb, var(--accent-cyan) 20%, transparent);
  }
  footer {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 20px;
    border-top: 1px solid var(--border);
  }
  .grow {
    flex: 1;
  }
</style>
