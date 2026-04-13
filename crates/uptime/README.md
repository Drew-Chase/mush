# uptime

A cross-platform `uptime` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **System uptime** — show how long the system has been running
- **Boot time** — display the date and time the system was started
- **Pretty format** — human-friendly uptime representation
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Show uptime, users, and load averages
uptime

# Pretty-print the uptime duration
uptime -p

# Show the date and time of last boot
uptime -s
```

## Flags

| Flag | Description |
|------|-------------|
| `-p` | Show uptime in a human-readable (pretty) format |
| `-s` | Print the date and time the system was booted |

## Building

```sh
cargo build --package uptime --release
```
