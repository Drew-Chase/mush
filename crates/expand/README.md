# expand

A cross-platform `expand` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Tab expansion** — convert tabs to the appropriate number of spaces
- **Custom tab stops** — set a uniform tab width or specify a comma-separated list of positions
- **Initial-only mode** — convert only leading tabs, leaving tabs within text unchanged
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Expand tabs to spaces (default 8-column stops)
expand file.txt

# Use 4-space tab stops
expand -t 4 file.txt

# Expand only initial (leading) tabs
expand -i file.txt

# Custom tab stop positions
expand -t 4,8,12 file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-t, --tabs=N` | Set tab stops at every N columns, or a comma-separated list of positions |
| `-i, --initial` | Do not convert tabs after non-blanks |

## Building

```sh
cargo build --package expand --release
```
