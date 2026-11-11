
<script lang="ts">
  import Terminal from "./Terminal.svelte";
</script>

<main>
  <Terminal />
</main>

<style>
  :global(html),
  :global(body),
  :global(#app) {
    width: 100%;
    height: 100%;
    margin: 0;
    overflow: hidden;
  }

  main {
    width: 100%;
    height: 100%;
    background: #101116;
  }
</style>
