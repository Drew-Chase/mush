# head

A cross-platform `head` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **First N lines** — print the first 10 lines of each file by default, configurable with `-n`
- **Byte mode** — print the first N bytes instead of lines
- **Multi-file headers** — automatically prints filename headers when multiple files are given
- **Stdin support** — read from standard input when no file is given or when `-` is specified
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print the first 10 lines
head file.txt

# Print the first 20 lines
head -n 20 file.txt

# Print the first 100 bytes
head -c 100 file.txt

# Print first 5 lines of multiple files with headers
head -n 5 file1.txt file2.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-n, --lines NUM` | Print the first NUM lines instead of first 10 |
| `-c, --bytes NUM` | Print the first NUM bytes of each file |
| `-q, --quiet` | Never print headers giving file names |
| `-v, --verbose` | Always print headers giving file names |

## Building

```sh
cargo build --package head --release
```
