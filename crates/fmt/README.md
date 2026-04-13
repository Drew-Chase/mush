# fmt

A cross-platform `fmt` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Paragraph reformatting** — reflow text to fit a specified width
- **Split-only mode** — split long lines without joining short ones
- **Uniform spacing** — normalize spacing between words and sentences
- **Prefix-aware** — reformat only lines matching a given prefix
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Reformat to default width (75 columns)
fmt document.txt

# Reformat to 60 columns
fmt -w 60 document.txt

# Split long lines only (do not join short lines)
fmt -s file.txt

# Uniform spacing (one space between words, two after sentences)
fmt -u file.txt

# Reformat only lines starting with "# "
fmt -p "# " file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-w, --width=WIDTH` | Maximum line width (default 75) |
| `-s, --split-only` | Split long lines but do not refill |
| `-u, --uniform-spacing` | One space between words, two after sentences |
| `-p, --prefix=STRING` | Only reformat lines beginning with STRING |

## Building

```sh
cargo build --package fmt --release
```
