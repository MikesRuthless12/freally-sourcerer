<script lang="ts">
  import { toastStore } from "../lib/stores/toast.svelte";
  import { t } from "../lib/i18n/t";
</script>

{#if toastStore.current}
  {#key toastStore.current.id}
    <div class="toast {toastStore.current.tone}" role="status" aria-live="polite">
      <span class="msg">{toastStore.current.message}</span>
      <button
        type="button"
        class="close"
        aria-label={t("toast-dismiss")}
        onclick={() => toastStore.dismiss()}
      >
        ×
      </button>
    </div>
  {/key}
{/if}

<style>
  .toast {
    position: fixed;
    right: 16px;
    bottom: 40px;
    z-index: 60;
    display: flex;
    align-items: center;
    gap: 12px;
    max-width: 480px;
    padding: 10px 12px;
    border-radius: 8px;
    border: 1px solid var(--border);
    background: var(--bg-surface-2);
    color: var(--text-primary);
    font-size: 12px;
    box-shadow: 0 6px 20px rgb(0 0 0 / 35%);
  }
  .toast.error {
    border-color: var(--accent-red, #e5484d);
  }
  .msg {
    flex: 1;
    overflow-wrap: anywhere;
  }
  .close {
    background: transparent;
    border: 0;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 16px;
    line-height: 1;
    padding: 0 2px;
  }
  .close:hover {
    color: var(--text-primary);
  }
</style>
