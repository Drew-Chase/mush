# reset

A cross-platform `reset` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Terminal reset** — restores the terminal to a sane default state
- **Clears screen** — resets scrollback and cursor position
- **Fixes broken state** — recovers from garbled output or misconfigured terminal modes
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Reset the terminal to a sane state
reset
```

## Building

```sh
cargo build --package reset --release
```
