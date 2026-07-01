<script lang="ts">
  import type { LensId } from "../../lib/ipc/types";
  import { formatLensTiming } from "../../lib/util/format";

  interface Props {
    lens: LensId;
    ms: number;
  }
  let { lens, ms }: Props = $props();

  const label = $derived(formatLensTiming(ms));
</script>

<span class="badge {lens}" title="{lens} lens">
  <span class="dot"></span>
  <span class="label">{label}</span>
</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 1px 6px;
    border-radius: 3px;
    background: var(--bg-surface-2);
    color: var(--text-secondary);
    font-size: 11px;
  }
  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
  }
  .badge.filename .dot { background: var(--lens-filename); }
  .badge.content .dot { background: var(--lens-content); }
  .badge.audio .dot { background: var(--lens-audio); }
  .badge.similarity .dot { background: var(--lens-similarity); }
</style>
