<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { FitAddon } from "@xterm/addon-fit";
  import { Terminal } from "@xterm/xterm";
  import { onMount } from "svelte";
  import "@xterm/xterm/css/xterm.css";

  interface Props {
    active?: boolean;
  }

  interface Config {
    font_family: string;
  }

  let { active = false }: Props = $props();
  let container: HTMLDivElement;
  let termInstance = $state<Terminal | undefined>(undefined);

  $effect(() => {
    if (!termInstance) return;
    if (active) {
      termInstance.focus();
    } else {
      termInstance.blur();
    }
  });

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
          fontFamily: `${config.font_family}`,
          theme: {
            background: "#1c1c1c",
            foreground: "#ddeedd",
            cursor: "#e2bbef",
            cursorAccent: "#000000",
            selectionBackground: "#4d4d4d",
            selectionForeground: "#ffffff",
            black: "#3d352a",
            red: "#cd5c5c",
            green: "#86af80",
            yellow: "#e8ae5b",
            blue: "#6495ed",
            magenta: "#deb887",
            cyan: "#b0c4de",
            white: "#bbaa99",
            brightBlack: "#554444",
            brightRed: "#cc5533",
            brightGreen: "#88aa22",
            brightYellow: "#ffa75d",
            brightBlue: "#87ceeb",
            brightMagenta: "#996600",
            brightCyan: "#b0c4de",
            brightWhite: "#ddccbb",
          },
        });

        const fitAddon = new FitAddon();
        terminal.loadAddon(fitAddon);
        terminal.open(container);
        fitAddon.fit();

        termInstance = terminal;
        if (active) terminal.focus();

        id = await invoke<number>("init", {
          rows: terminal.rows,
          cols: terminal.cols,
        });

        if (destroyed) return;

        unlistenOutput = await listen<string>(`pty-output-${id}`, (event) => {
          terminal?.write(event.payload);
        });

        terminal.onData((data) => {
          if (id === undefined || destroyed) return;
          invoke("write_to_pty", { data, index: id }).catch(console.error);
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
    box-sizing: border-box;
    overflow: hidden;
    padding: 15px;
    letter-spacing: 1px;
  }
</style>
