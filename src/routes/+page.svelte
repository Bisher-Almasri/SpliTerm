<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { FitAddon } from "@xterm/addon-fit";
  import { Terminal } from "@xterm/xterm";
  import { onMount } from "svelte";

  let container: HTMLDivElement;

  let charactersWrote = 0;

  onMount(() => {
    const terminal = new Terminal({
      cursorBlink: true,
      fontSize: 14,
      fontFamily: "monospace",
      theme: {
        background: "#101116",
        foreground: "#e5e7eb",
        cursor: "#a78bfa",
      },
    });

    const fitAddon = new FitAddon();

    terminal.loadAddon(fitAddon);
    terminal.open(container);
    fitAddon.fit();

    const observer = new ResizeObserver(() => {
      fitAddon.fit();
    });

    observer.observe(container);

    let unlistenOutput: (() => void) | undefined;
    let unlistenError: (() => void) | undefined;

    const startShell = async () => {
      try {
        unlistenOutput = await listen<string>("pty-output", (event) => {
          terminal.write(event.payload);
        });
        unlistenError = await listen<string>("pty-error", (event) => {
          terminal.writeln(`\r\nPTY error: ${event.payload}`);
        });

        invoke<string>("init");
      } catch (error) {
        terminal.writeln(`\r\nError: ${error}`);
      }
    };

    startShell();

    terminal.onData((data) => {
      invoke("write_to_pty", { data }).catch((error) => {
        terminal.writeln(`\r\nPTY write error: ${error}`);
      });
    });

    return () => {
      observer.disconnect();
      unlistenOutput?.();
      unlistenError?.();
      // terminal.dispose();
    };
  });
</script>

<main>
  <div bind:this={container} class="terminal"></div>
</main>

<style>
  .terminal {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

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