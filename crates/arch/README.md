# arch

A cross-platform `arch` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Architecture detection** — prints the machine hardware architecture (e.g. x86_64, aarch64)
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print the machine architecture
arch
```

## Flags

This command has no flags.

## Building

```sh
cargo build --package arch --release
```
