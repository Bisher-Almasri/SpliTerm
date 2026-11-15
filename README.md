# SpliTerm

SpliTerm is a cross-platform terminal application built with [Tauri 2](https://tauri.app/), [SvelteKit](https://svelte.dev/), TypeScript, and Rust.

Install the macOS command-line tools if needed:

```bash
xcode-select --install
```

On Windows, install the **Desktop development with C++** workload from the [Visual Studio Installer](https://visualstudio.microsoft.com/downloads/). WebView2 is included with current Windows versions; install the Evergreen WebView2 Runtime if it is missing.

## Install

Clone the repository and install the JavaScript dependencies:

```bash
git clone https://github.com/Bisher-Almasri/SpliTerm.git
cd SpliTerm
bun install
```

## Development

Start the Tauri development application:

```bash
bun run tauri dev
```

## Build for macOS

Run this on a Mac:

```bash
bun run tauri build
```

The generated application and installer are written under:

```text
src-tauri/target/release/bundle/
```

Typical outputs include a `.app` and a `.dmg`. The build targets the architecture of the Mac running the command. For distribution outside your own machine, configure Apple code signing and notarization in the Tauri build configuration.

## Build for Windows

Run this on Windows in PowerShell:

```powershell
bun run tauri build
```

The generated Windows installers are written under:

```text
src-tauri\target\release\bundle\
```

Typical outputs include an `.msi` and/or a setup `.exe`. Build on Windows for Windows, and configure code signing before distributing the installer.

## Cross-platform builds

Tauri normally builds for the operating system where the command is run. The simplest and most reliable release process is:

1. Build macOS releases on macOS.
2. Build Windows releases on Windows.
3. Upload the installers from each platform's `src-tauri/target/release/bundle/` directory.

## Project structure

- `src/` - SvelteKit frontend
- `src-tauri/` - Rust/Tauri desktop application
- `static/` - Static frontend assets
- `build/` - Generated frontend output
