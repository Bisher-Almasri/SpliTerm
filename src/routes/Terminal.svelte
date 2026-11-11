
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { FitAddon } from "@xterm/addon-fit";
  import { Terminal } from "@xterm/xterm";
  import { onMount } from "svelte";
  import "@xterm/xterm/css/xterm.css";

  interface Config {
    font_family: string;
  }

  let container: HTMLDivElement;

  onMount(() => {
    let terminal: Terminal | undefined;
    let id: number | undefined;
    let destroyed = false;

    let unlistenOutput: (() => void) | undefined;
    let observer: ResizeObserver | undefined;

    const start = async () => {
      try {
        const config = await invoke<Config>("get_config");

        if (destroyed) return;

        terminal = new Terminal({
          cursorBlink: true,
          cursorStyle: "bar",
          fontSize: 16,
          fontFamily: config.font_family,
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

        id = await invoke<number>("init", {
          rows: terminal.rows,
          cols: terminal.cols,
        });

        if (destroyed) {
          return;
        }

        unlistenOutput = await listen<string>(
          `pty-output-${id}`,
          (event) => {
            terminal?.write(event.payload);
          }
        );

        terminal.onData((data) => {
          if (id === undefined || destroyed) return;

          invoke("write_to_pty", {
            data,
            index: id,
          }).catch(console.error);
        });

        observer = new ResizeObserver(() => {
          if (!terminal) return;

          fitAddon.fit();

          if (id !== undefined) {
            invoke("resize_terminal", {
              index: id,
              rows: terminal.rows,
              cols: terminal.cols,
            }).catch(console.error);
          }
        });

        observer.observe(container);

      } catch (error) {
        terminal?.writeln(`\r\nError: ${error}`);
      }
    };

    start();

    return () => {
      destroyed = true;
      observer?.disconnect();
      unlistenOutput?.();
      terminal?.dispose();
    };
  });
</script>

 <div bind:this={container} class="terminal"></div>

<style>
  .terminal {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }
</style>
