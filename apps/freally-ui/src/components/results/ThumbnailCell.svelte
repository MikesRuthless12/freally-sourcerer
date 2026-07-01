<script lang="ts">
  import * as files from "../../lib/ipc/files";

  interface Props {
    path: string;
    size: number;
  }
  let { path, size }: Props = $props();

  let dataUrl = $state<string | null>(null);
  let lastPath = "";

  $effect(() => {
    if (path === lastPath) return;
    lastPath = path;
    files.thumbnail(path, size).then(
      (url) => (dataUrl = url),
      () => (dataUrl = null)
    );
  });
</script>

<span class="thumb" style="width: {size}px; height: {size}px;">
  {#if dataUrl}
    <img src={dataUrl} alt="" />
  {:else}
    <span class="placeholder"></span>
  {/if}
</span>

<style>
  .thumb {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }
  img {
    width: 100%;
    height: 100%;
    border-radius: 3px;
    object-fit: cover;
  }
  .placeholder {
    width: 100%;
    height: 100%;
    background: var(--bg-surface-2);
    border-radius: 3px;
  }
</style>
