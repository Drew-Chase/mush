# pgrep

A cross-platform `pgrep` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Pattern matching** — find processes by name or command-line pattern
- **Flexible output** — show PIDs, names, full command lines, or counts
- **Filtering** — narrow results by user, newest, or oldest match
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Find processes by name
pgrep firefox

# List matching PIDs with process names
pgrep -l nginx

# Match against full command line
pgrep -f "python server.py"

# Count matching processes
pgrep -c sshd
```

## Flags

| Flag | Description |
|------|-------------|
| `-l` | List PID and process name |
| `-a` | List PID and full command line |
| `-c` | Print a count of matching processes |
| `-d` | Set the output delimiter (default newline) |
| `-f` | Match against the full command line |
| `-i` | Case-insensitive matching |
| `-x` | Require an exact match of the process name |
| `-u` | Match processes owned by the given user |
| `-n` | Select the newest (most recently started) matching process |
| `-o` | Select the oldest matching process |

## Building

```sh
cargo build --package pgrep --release
```
