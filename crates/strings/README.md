# strings

A cross-platform `strings` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Printable string extraction** — find and print sequences of printable characters in binary files
- **Minimum length threshold** — only print strings at least N characters long (default: 4)
- **Offset display** — show the byte offset of each string in octal, hex, or decimal
- **Full file scanning** — scan the entire file for printable sequences
- **Stdin support** — read from standard input when no file is given
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print strings of 4+ characters from a binary
strings binary_file

# Print strings of at least 8 characters
strings -n 8 binary_file

# Show hex offsets for each string
strings -t x binary_file

# Show decimal offsets
strings -t d binary_file
```

## Flags

| Flag | Description |
|------|-------------|
| `-n, --bytes NUM` | Print sequences of at least NUM characters (default: 4) |
| `-a, --all` | Scan the whole file (default behavior) |
| `-t, --radix CHAR` | Print the offset with radix: `o` (octal), `x` (hex), or `d` (decimal) |

## Building

```sh
cargo build --package strings --release
```
