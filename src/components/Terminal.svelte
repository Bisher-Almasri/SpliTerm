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
    font: {
      font_family?: string | null;
      font_size?: number | null;
    };
    theme: {
      background?: string | null;
      foreground?: string | null;

      cursor?: string | null;
      cursor_accent?: string | null;
      cursor_style?: "bar" | "block" | "underline" | null;
      cursor_blink?: boolean | null;

      selection_background?: string | null;
      selection_foreground?: string | null;

      black?: string | null;
      red?: string | null;
      green?: string | null;
      yellow?: string | null;
      blue?: string | null;
      magenta?: string | null;
      cyan?: string | null;
      white?: string | null;

      bright_black?: string | null;
      bright_red?: string | null;
      bright_green?: string | null;
      bright_yellow?: string | null;
      bright_blue?: string | null;
      bright_magenta?: string | null;
      bright_cyan?: string | null;
      bright_white?: string | null;
    };
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

        const style = config.theme.cursor_style?.toLowerCase();

        if (style === "bar" || style === "block" || style === "underline") {
          config.theme.cursor_style = style;
        }
        
        terminal = new Terminal({
          cursorBlink: config.theme.cursor_blink ?? true,
          cursorStyle: config.theme.cursor_style ?? "bar",
          fontSize: config.font.font_size ?? 16,
          fontFamily: config.font.font_family ?? "monospace",
          allowTransparency: true,

          theme: {
            background: "rgba(0,0,0,0)",
            foreground: config.theme.foreground ?? "#ddeedd",

            cursor: config.theme.cursor ?? "#e2bbef",
            cursorAccent: config.theme.cursor_accent ?? "#000000",

            selectionBackground: config.theme.selection_background ?? "#4d4d4d",

            selectionForeground: config.theme.selection_foreground ?? "#ffffff",

            black: config.theme.black ?? "#3d352a",
            red: config.theme.red ?? "#cd5c5c",
            green: config.theme.green ?? "#86af80",
            yellow: config.theme.yellow ?? "#e8ae5b",
            blue: config.theme.blue ?? "#6495ed",
            magenta: config.theme.magenta ?? "#deb887",
            cyan: config.theme.cyan ?? "#b0c4de",
            white: config.theme.white ?? "#bbaa99",

            brightBlack: config.theme.bright_black ?? "#554444",
            brightRed: config.theme.bright_red ?? "#cc5533",
            brightGreen: config.theme.bright_green ?? "#88aa22",
            brightYellow: config.theme.bright_yellow ?? "#ffa75d",
            brightBlue: config.theme.bright_blue ?? "#87ceeb",
            brightMagenta: config.theme.bright_magenta ?? "#996600",
            brightCyan: config.theme.bright_cyan ?? "#b0c4de",
            brightWhite: config.theme.bright_white ?? "#ddccbb",
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

<div class="terminal">
  <div bind:this={container} class="terminal-content"></div>
</div>

<style>
  .terminal {
    position: relative;
    width: 100%;
    height: 100%;
    padding: 7.5px;
    box-sizing: border-box;
    overflow: hidden;
    border-radius: 10px;
    background-color: rgba(0, 0, 0, 0);
  }

  .terminal-content {
    width: 100%;
    height: 100%;
    min-width: 0;
    min-height: 0;
    overflow: hidden;
  }

  .terminal :global(.xterm),
  .terminal :global(.xterm-viewport) {
    background-color: transparent !important;
  }
</style>
