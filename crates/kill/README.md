# kill

A cross-platform `kill` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Send signals** — deliver signals to processes by PID
- **Signal styles** — supports `-9`, `-KILL`, `-SIGTERM`, and `-TERM` notation
- **List signals** — display available signal names and numbers
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Terminate a process
kill 1234

# Send SIGKILL
kill -9 1234

# Send a named signal
kill -TERM 1234

# List all available signals
kill -l
```

## Flags

| Flag | Description |
|------|-------------|
| `-s, --signal` | Specify the signal to send (name or number) |
| `-l, --list` | List signal names |
| `-L, --table` | List signal names in a table format |

## Building

```sh
cargo build --package kill --release
```
