<script lang="ts" generics="T extends string">
  interface Option {
    value: T;
    label: string;
  }
  interface Props {
    label?: string;
    value: T;
    options: Option[];
    onChange: (next: T) => void;
    disabled?: boolean;
    id?: string;
  }
  let { label, value, options, onChange, disabled = false, id }: Props = $props();
</script>

<div class="dropdown" class:disabled>
  {#if label}
    <span class="lbl" id={id ? `${id}-lbl` : undefined}>{label}</span>
  {/if}
  <select
    {id}
    {disabled}
    aria-labelledby={id && label ? `${id}-lbl` : undefined}
    aria-label={!label ? "Setting" : undefined}
    onchange={(e) => onChange((e.currentTarget as HTMLSelectElement).value as T)}
    bind:value
  >
    {#each options as o (o.value)}
      <option value={o.value}>{o.label}</option>
    {/each}
  </select>
</div>

<style>
  .dropdown {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 4px 0;
    color: var(--text-primary);
    font-size: 13px;
  }
  .dropdown.disabled {
    opacity: 0.55;
  }
  .lbl {
    flex: 1;
  }
  select {
    min-width: 180px;
    padding: 4px 6px;
    background: var(--bg-canvas);
    color: var(--text-primary);
    border: 1px solid var(--border);
    border-radius: 4px;
    font: inherit;
  }
</style>
