<script lang="ts">
  interface Props {
    title: string;
    children: import("svelte").Snippet;
  }
  let { title, children }: Props = $props();
</script>

<section class="sect">
  <h3>{title}</h3>
  <div class="body">
    {@render children()}
  </div>
</section>

<style>
  .sect {
    margin-bottom: 18px;
  }
  h3 {
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-secondary);
    margin: 0 0 6px;
  }
  .body {
    border-left: 2px solid var(--border);
    padding: 4px 0 4px 12px;
    margin-left: 4px;
  }
</style>
