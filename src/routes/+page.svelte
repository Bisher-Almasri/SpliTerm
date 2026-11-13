
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Terminal from "../components/Terminal.svelte";
  import TerminalGroup from "../components/TerminalGroup.svelte";

  interface Config {
    font: {
      font_family?: string | null;
      font_size?: number | null;
    };
    theme?: {
      background?: string | null;
    } | null;
  }

  let termGroup: TerminalGroup;


  $effect(() => {
    invoke<Config>("get_config")
      .then((config) => {
        let term = document.getElementById("term");
        if (term) {
          if (config.font.font_family) term.style.fontFamily = config.font.font_family;
          if (config.font.font_size) term.style.fontSize = `${config.font.font_size}`;
        } 
      })
      .catch((error) => {
        console.error("Failed to load theme background:", error);
      });
  });

  function handleKeydown(event: KeyboardEvent) {
    console.log(event);
    
    
    if ((event.ctrlKey || event.metaKey) && event.shiftKey && event.code == "KeyX") {
      event.preventDefault();
      console.log("closing");
      
      termGroup?.closeTerminal();
    }


    if ((event.ctrlKey || event.metaKey) && event.shiftKey && event.code == "KeyT") {
      event.preventDefault();

      termGroup.spawnTerminal()
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<main>
  <TerminalGroup id="term" bind:this={termGroup} />
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
  }
</style>
