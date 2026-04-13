# nohup

A cross-platform `nohup` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Hangup immunity** — run a command that persists after the terminal closes
- **Output redirection** — automatically redirects stdout and stderr to `nohup.out`
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Run a command immune to hangups
nohup long-running-job

# Combine with background execution
nohup ./server &
```

## Flags

This command has no flags. It takes a command and its arguments as operands.

## Building

```sh
cargo build --package nohup --release
```
