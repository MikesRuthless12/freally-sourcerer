<script lang="ts">
  interface Props {
    label?: string;
    value: string;
    onChange: (next: string) => void;
    placeholder?: string;
    disabled?: boolean;
    id?: string;
    width?: string;
  }
  let { label, value, onChange, placeholder, disabled = false, id, width = "260px" }: Props =
    $props();
</script>

<div class="ti" class:disabled style:--w={width}>
  {#if label}<span class="lbl" id={id ? `${id}-lbl` : undefined}>{label}</span>{/if}
  <input
    type="text"
    {id}
    {disabled}
    {placeholder}
    aria-labelledby={id && label ? `${id}-lbl` : undefined}
    aria-label={!label ? "Setting" : undefined}
    value={value}
    oninput={(e) => onChange((e.currentTarget as HTMLInputElement).value)}
  />
</div>

<style>
  .ti {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 4px 0;
    color: var(--text-primary);
    font-size: 13px;
  }
  .ti.disabled {
    opacity: 0.55;
  }
  .lbl {
    flex: 1;
  }
  input {
    width: var(--w);
    padding: 4px 6px;
    background: var(--bg-canvas);
    color: var(--text-primary);
    border: 1px solid var(--border);
    border-radius: 4px;
    font: inherit;
  }
</style>
