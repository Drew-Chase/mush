# pkill

A cross-platform `pkill` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Signal by pattern** — send signals to processes matching a name pattern
- **Full command matching** — match against the entire command line
- **Filtering** — narrow targets by user, newest, or oldest match
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Kill processes by name
pkill firefox

# Send SIGHUP to matching processes
pkill -s HUP nginx

# Match against full command line
pkill -f "python server.py"

# Kill the newest matching process
pkill -n myapp
```

## Flags

| Flag | Description |
|------|-------------|
| `-s, --signal` | Specify the signal to send (name or number, default SIGTERM) |
| `-f` | Match against the full command line |
| `-i` | Case-insensitive matching |
| `-x` | Require an exact match of the process name |
| `-u` | Match processes owned by the given user |
| `-n` | Select the newest (most recently started) matching process |
| `-o` | Select the oldest matching process |

## Building

```sh
cargo build --package pkill --release
```
