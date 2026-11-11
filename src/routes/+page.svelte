
<script lang="ts">
  import Terminal from "../components/Terminal.svelte";
  import TerminalGroup from "../components/TerminalGroup.svelte";
</script>

<main>
  <!-- <Terminal /> -->
   <TerminalGroup />
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
    background: #1c1c1c;
  }
</style>
