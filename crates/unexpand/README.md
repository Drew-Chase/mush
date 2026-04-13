# unexpand

A cross-platform `unexpand` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Space-to-tab conversion** — convert runs of spaces back to tabs
- **All-blanks mode** — convert spaces throughout each line, not just leading spaces
- **Custom tab stops** — set a uniform tab width or specify a comma-separated list of positions
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Convert leading spaces to tabs (default 8-column stops)
unexpand file.txt

# Convert all spaces, not just leading
unexpand -a file.txt

# Use 4-space tab stops
unexpand -t 4 file.txt

# Convert only leading spaces (explicit)
unexpand --first-only file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-a, --all` | Convert all blanks instead of just initial blanks |
| `-t, --tabs=N` | Set tab stops at every N columns, or a comma-separated list of positions |
| `--first-only` | Convert only leading sequences of blanks (overrides `-a`) |

## Building

```sh
cargo build --package unexpand --release
```
