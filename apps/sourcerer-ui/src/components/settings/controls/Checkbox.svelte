<script lang="ts">
  interface Props {
    label: string;
    checked: boolean;
    onChange: (next: boolean) => void;
    disabled?: boolean;
    id?: string;
    description?: string;
  }
  let { label, checked, onChange, disabled = false, id, description }: Props = $props();
</script>

<label class="checkbox" class:disabled>
  <input
    type="checkbox"
    {id}
    {disabled}
    checked={checked}
    aria-label={label}
    aria-describedby={description ? `${id}-desc` : undefined}
    onchange={(e) => onChange((e.currentTarget as HTMLInputElement).checked)}
  />
  <span class="text">{label}</span>
  {#if description}
    <span id={`${id}-desc`} class="desc">{description}</span>
  {/if}
</label>

<style>
  .checkbox {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 4px 0;
    cursor: pointer;
    color: var(--text-primary);
    font-size: 13px;
  }
  .checkbox.disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }
  input[type="checkbox"] {
    margin-top: 2px;
    cursor: inherit;
  }
  .text {
    flex: 1;
    line-height: 1.4;
  }
  .desc {
    color: var(--text-secondary);
    font-size: 12px;
    margin-left: 24px;
    display: block;
  }
</style>
