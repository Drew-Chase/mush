# wc

A cross-platform `wc` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Line, word, and byte counts** — displays all three by default when no flags are specified
- **Character counting** — count characters separately from bytes for multi-byte encodings
- **Max line length** — report the length of the longest line
- **Multi-file totals** — prints a total line when more than one file is given
- **Stdin support** — read from standard input when no file is given or when `-` is specified
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Count lines, words, and bytes
wc file.txt

# Count only lines
wc -l file.txt

# Count lines across multiple files with totals
wc -l file1.txt file2.txt

# Find the longest line
wc -L file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-l, --lines` | Print the newline counts |
| `-w, --words` | Print the word counts |
| `-c, --bytes` | Print the byte counts |
| `-m, --chars` | Print the character counts |
| `-L, --max-line-length` | Print the maximum display width |

## Building

```sh
cargo build --package wc --release
```
