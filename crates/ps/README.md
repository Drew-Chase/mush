# ps

A cross-platform `ps` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Process listing** — display information about running processes
- **Flexible output** — full, long, and user-defined format modes
- **Filtering** — select processes by user, PID, or command name
- **Sortable** — order output by CPU, memory, PID, or other keys
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# List all processes
ps -e

# Full-format listing
ps -ef

# Show processes for a specific user
ps -u drew

# Custom output columns sorted by memory
ps -eo pid,user,%mem,comm --sort %mem
```

## Flags

| Flag | Description |
|------|-------------|
| `-e, -A` | Select all processes |
| `-f, --full` | Full-format listing |
| `-l, --long` | Long-format listing |
| `-u USER` | Show processes for the specified user |
| `-p PIDS` | Select processes by PID |
| `-C NAME` | Select processes by command name |
| `--sort KEY` | Sort output by the given key (e.g. pid, %cpu, %mem) |
| `-o FORMAT` | User-defined output format |
| `--no-headers` | Suppress the header line |
| `-a` | Show processes with a terminal |
| `-x` | Include processes without a controlling terminal |

## Building

```sh
cargo build --package ps --release
```
