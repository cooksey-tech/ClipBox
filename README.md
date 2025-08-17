# ClipBox

Experimental Windows utility that integrates with the foreground window (e.g., File Explorer) and the system clipboard using Win32 APIs.

Note: This application requires Administrator privileges at runtime. The included app.manifest requests elevation and is embedded into the binary during build.

## Requirements
- Windows 10/11 (x86_64)
- Rust (stable) with one of the following toolchains:
  - MSVC (Visual C++ Build Tools) – recommended
  - MinGW-w64
- Rustup installed
- Administrator privileges to run the built binary

## Setup
If using the MSVC toolchain (recommended):
1) Install Visual Studio Build Tools (C++ build tools) or Visual Studio with C++ workload.
2) Set Rust to use the MSVC toolchain:

```
rustup default stable-msvc
```

For MinGW-w64, install MinGW-w64 and ensure the toolchain is on PATH. Then use the default GNU target or explicitly set it if needed.

## Build
- Debug build:

```
cargo build
```

- Release build:

```
cargo build --release
```

The build script embeds the application manifest via embed-resource (see build.rs, app.rc, app.manifest).

## Run
Because the app requires elevation, run it from an elevated terminal, or run the built .exe as Administrator.

- Run (debug) from an elevated terminal:

```
cargo run
```

- Run the built executable as Administrator (example for release build):

```
./target/release/ClipBox.exe
```

Open File Explorer, right‑click the executable, and choose "Run as administrator" if you are not already in an elevated shell.

## Test
```
cargo test
```

## Project Structure
- Cargo.toml – crate metadata and dependencies
- build.rs – embeds Windows resources (manifest)
- app.rc – resource script that includes app.manifest
- app.manifest – requests Administrator privileges
- src/
  - main.rs – entry point; hooks into events, foreground window, and storage modules
  - constants.rs, enums/, events/, settings/, storage/, tools/, windows/, tests/ – internal modules

## Troubleshooting
- Linker or build tool errors on Windows: ensure MSVC Build Tools (or MinGW-w64) are installed and on PATH.
- Permission errors at runtime: confirm you launched the application with Administrator rights.
- Cross-compilation from non-Windows hosts is not supported; build on Windows.
