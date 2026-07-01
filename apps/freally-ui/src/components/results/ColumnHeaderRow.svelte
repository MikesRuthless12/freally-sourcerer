<script lang="ts">
  import { columnsStore } from "../../lib/stores/columns.svelte";
  import ColumnHeader from "./ColumnHeader.svelte";
</script>

<div class="header-row" role="row">
  {#each columnsStore.visible as col (col.id)}
    <ColumnHeader id={col.id} width={col.width_px} />
  {/each}
</div>

<style>
  .header-row {
    display: flex;
    align-items: center;
    height: 28px;
    background: var(--bg-surface-2);
    border-bottom: 1px solid var(--border);
    overflow: hidden;
  }
</style>
