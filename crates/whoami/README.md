# whoami

A cross-platform `whoami` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **User identification** — print the effective user name of the current user
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print the current user name
whoami
```

## Flags

This command has no flags.

## Building

```sh
cargo build --package whoami --release
```
