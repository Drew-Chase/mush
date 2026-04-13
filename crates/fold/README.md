# fold

A cross-platform `fold` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Line wrapping** — wrap input lines to a specified width
- **Byte or character counting** — count width in bytes or characters
- **Break at spaces** — optionally break lines at the last space within the width
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Wrap lines at 80 columns (default)
fold file.txt

# Wrap at 40 columns
fold -w 40 file.txt

# Break at the last space within the width
fold -s -w 60 file.txt

# Count width in bytes instead of columns
fold -b file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-w, --width=WIDTH` | Use WIDTH columns instead of 80 |
| `-b, --bytes` | Count width in bytes rather than columns |
| `-s, --spaces` | Break at spaces |

## Building

```sh
cargo build --package fold --release
```
