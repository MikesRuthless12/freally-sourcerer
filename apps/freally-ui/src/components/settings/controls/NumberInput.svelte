<script lang="ts">
  interface Props {
    label?: string;
    value: number;
    onChange: (next: number) => void;
    min?: number;
    max?: number;
    step?: number;
    suffix?: string;
    disabled?: boolean;
    id?: string;
  }
  let {
    label,
    value,
    onChange,
    min,
    max,
    step = 1,
    suffix,
    disabled = false,
    id
  }: Props = $props();

  function clamp(n: number) {
    if (typeof min === "number" && n < min) n = min;
    if (typeof max === "number" && n > max) n = max;
    return n;
  }
</script>

<div class="ni" class:disabled>
  {#if label}<span class="lbl" id={id ? `${id}-lbl` : undefined}>{label}</span>{/if}
  <input
    type="number"
    {id}
    {disabled}
    {min}
    {max}
    {step}
    aria-labelledby={id && label ? `${id}-lbl` : undefined}
    aria-label={!label ? "Setting" : undefined}
    value={value}
    oninput={(e) => {
      const n = Number((e.currentTarget as HTMLInputElement).value);
      if (!Number.isNaN(n)) onChange(clamp(n));
    }}
  />
  {#if suffix}<span class="sfx">{suffix}</span>{/if}
</div>

<style>
  .ni {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 4px 0;
    color: var(--text-primary);
    font-size: 13px;
  }
  .ni.disabled {
    opacity: 0.55;
  }
  .lbl {
    flex: 1;
  }
  input {
    width: 110px;
    padding: 4px 6px;
    background: var(--bg-canvas);
    color: var(--text-primary);
    border: 1px solid var(--border);
    border-radius: 4px;
    font: inherit;
  }
  .sfx {
    color: var(--text-secondary);
    min-width: 40px;
  }
</style>
