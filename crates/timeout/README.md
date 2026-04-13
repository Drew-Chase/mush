# timeout

A cross-platform `timeout` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Time-limited execution** — terminate a command after a specified duration
- **Configurable signal** — choose which signal to send on timeout
- **Escalation** — optionally send SIGKILL after a grace period
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Run a command with a 30-second time limit
timeout 30 ./slow-command

# Send SIGKILL instead of SIGTERM
timeout -s KILL 10 ./stuck-process

# Send SIGTERM, then SIGKILL after 5 seconds if still running
timeout -k 5 30 ./server
```

## Flags

| Flag | Description |
|------|-------------|
| `-s, --signal` | Specify the signal to send on timeout (default SIGTERM) |
| `-k, --kill-after` | Send SIGKILL after the given duration if the command is still running |
| `--preserve-status` | Exit with the same status as the command, even on timeout |
| `-v, --verbose` | Print a diagnostic message when the signal is sent |

## Building

```sh
cargo build --package timeout --release
```
