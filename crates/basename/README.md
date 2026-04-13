# basename

A cross-platform `basename` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Strip directory components** — extract the filename from a path
- **Suffix removal** — optionally strip a file extension or suffix
- **Multiple arguments** — process several paths at once
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Get the filename from a path
basename /usr/local/bin/tool
# => tool

# Strip a suffix
basename document.tar.gz .tar.gz
# => document

# Process multiple paths
basename -a /usr/bin/env /usr/bin/test
# => env
# => test

# Strip suffix from multiple files
basename -s .rs src/main.rs src/lib.rs
# => main
# => lib
```

## Flags

| Flag | Description |
|------|-------------|
| `-a, --multiple` | Support multiple arguments; treat each as a path |
| `-s, --suffix` | Remove a trailing suffix (e.g. `.txt`, `.tar.gz`) |
| `-z, --zero` | End each output line with NUL instead of newline |

## Building

```sh
cargo build --package basename --release
```
