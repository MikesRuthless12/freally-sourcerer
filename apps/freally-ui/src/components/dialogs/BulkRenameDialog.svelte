<script lang="ts">
  // SRC-M15 — Rename the selected results.
  //
  // The table is a view of the backend's plan, not an input to it: Apply
  // sends the same rule again and the backend re-derives every name. A
  // tampered preview therefore changes nothing.
  import { renameStore } from "../../lib/stores/rename.svelte";
  import { toastStore } from "../../lib/stores/toast.svelte";
  import { t } from "../../lib/i18n/t";
  import type { RenameItem } from "../../lib/ipc/types";

  interface Props {
    open: boolean;
    onClose: () => void;
  }
  let { open, onClose }: Props = $props();

  const preview = $derived(renameStore.preview);

  function edit<K extends keyof typeof renameStore.rule>(
    key: K,
    value: (typeof renameStore.rule)[K]
  ) {
    renameStore.rule = { ...renameStore.rule, [key]: value };
    renameStore.schedule();
  }

  function statusText(item: RenameItem): string {
    if (item.status === "invalid" && item.reason) {
      return t(`rename-invalid-${item.reason.replace(/_/g, "-")}`);
    }
    return t(`rename-status-${item.status}`);
  }

  async function apply() {
    const n = await renameStore.apply();
    if (n !== null) {
      toastStore.show(t("rename-toast-done", { count: n }));
      close();
    }
  }

  function close() {
    renameStore.reset();
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
      aria-labelledby="bulk-rename-title"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <h2 id="bulk-rename-title">
        {t("rename-title", { count: renameStore.paths.length })}
      </h2>

      <div class="rule">
        <label>
          <span>{t("rename-find")}</span>
          <input
            type="text"
            value={renameStore.rule.find}
            oninput={(e) => edit("find", e.currentTarget.value)}
            autocomplete="off"
            spellcheck="false"
          />
        </label>
        <label>
          <span>{t("rename-replace")}</span>
          <input
            type="text"
            value={renameStore.rule.replace}
            oninput={(e) => edit("replace", e.currentTarget.value)}
            autocomplete="off"
            spellcheck="false"
          />
        </label>
        <div class="toggles">
          <label class="inline">
            <input
              type="checkbox"
              checked={renameStore.rule.use_regex}
              onchange={(e) => edit("use_regex", e.currentTarget.checked)}
            />
            <span>{t("rename-use-regex")}</span>
          </label>
          <label class="inline">
            <input
              type="checkbox"
              checked={renameStore.rule.ignore_case}
              onchange={(e) => edit("ignore_case", e.currentTarget.checked)}
            />
            <span>{t("rename-ignore-case")}</span>
          </label>
          <label class="inline">
            <span>{t("rename-part")}</span>
            <select
              value={renameStore.rule.part}
              onchange={(e) => edit("part", e.currentTarget.value as "stem" | "full")}
            >
              <option value="stem">{t("rename-part-stem")}</option>
              <option value="full">{t("rename-part-full")}</option>
            </select>
          </label>
          <label class="inline">
            <span>{t("rename-case")}</span>
            <select
              value={renameStore.rule.case}
              onchange={(e) => edit("case", e.currentTarget.value as "none")}
            >
              <option value="none">{t("rename-case-none")}</option>
              <option value="lower">{t("rename-case-lower")}</option>
              <option value="upper">{t("rename-case-upper")}</option>
              <option value="title">{t("rename-case-title")}</option>
            </select>
          </label>
          <label class="inline">
            <span>{t("rename-counter-start")}</span>
            <input
              type="number"
              min="0"
              value={renameStore.rule.counter_start}
              oninput={(e) => edit("counter_start", Number(e.currentTarget.value) || 0)}
            />
          </label>
        </div>
        <p class="hint">{t("rename-hint")}</p>
      </div>

      {#if renameStore.error}
        <p class="error">{renameStore.error}</p>
      {/if}

      {#if preview}
        <div class="table-wrap">
          <table>
            <thead>
              <tr>
                <th>{t("rename-col-before")}</th>
                <th>{t("rename-col-after")}</th>
                <th>{t("rename-col-status")}</th>
              </tr>
            </thead>
            <tbody>
              {#each preview.items as item (item.from)}
                <tr class={item.status}>
                  <td title={item.from}>{item.from_name}</td>
                  <td>{item.to_name}</td>
                  <td>{statusText(item)}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
        <p class="summary" class:blocked={preview.blocked}>
          {#if preview.blocked}
            {t("rename-blocked")}
          {:else}
            {t("rename-summary", { count: preview.will_apply })}
          {/if}
        </p>
      {/if}

      <footer>
        <button
          type="button"
          class="primary"
          disabled={!renameStore.canApply}
          onclick={() => void apply()}
        >
          {t("rename-apply")}
        </button>
        <button type="button" onclick={close}>{t("settings-cancel")}</button>
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
    z-index: 100;
  }
  .modal {
    width: 720px;
    max-width: 92vw;
    max-height: 84vh;
    display: flex;
    flex-direction: column;
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
  }
  .rule {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  label {
    display: grid;
    grid-template-columns: 90px 1fr;
    align-items: center;
    gap: 10px;
    font-size: 13px;
  }
  label > span {
    color: var(--text-secondary);
  }
  .toggles {
    display: flex;
    flex-wrap: wrap;
    gap: 14px;
    align-items: center;
  }
  label.inline {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
  }
  label.inline input[type="number"] {
    width: 70px;
  }
  .hint {
    margin: 2px 0 0;
    font-size: 11px;
    color: var(--text-secondary);
  }
  .error {
    color: var(--danger, #d94040);
    font-size: 13px;
  }
  .table-wrap {
    margin-top: 12px;
    overflow-y: auto;
    flex: 1;
    min-height: 120px;
    border: 1px solid var(--border);
    border-radius: 6px;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 12px;
  }
  th {
    position: sticky;
    top: 0;
    text-align: left;
    font-weight: 600;
    color: var(--text-secondary);
    background: var(--bg-surface);
    padding: 5px 8px;
    border-bottom: 1px solid var(--border);
  }
  td {
    padding: 4px 8px;
    border-bottom: 1px solid var(--border);
  }
  tr.unchanged {
    color: var(--text-secondary);
  }
  tr.invalid,
  tr.collision,
  tr.exists {
    color: var(--danger, #d94040);
  }
  .summary {
    margin: 8px 0 0;
    font-size: 12px;
    color: var(--text-secondary);
  }
  .summary.blocked {
    color: var(--danger, #d94040);
    font-weight: 600;
  }
  footer {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 16px;
  }
</style>
