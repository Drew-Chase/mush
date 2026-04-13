# seq

A cross-platform `seq` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Flexible sequences** — specify first, increment, and last values
- **Custom separators** — use any string to separate output numbers
- **Format control** — printf-style format strings for output
- **Equal width** — pad numbers with leading zeros to equal width
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print numbers 1 through 10
seq 10

# Print numbers 5 through 15
seq 5 15

# Print even numbers from 2 to 20
seq 2 2 20

# Use a custom separator
seq -s ", " 1 5

# Pad with leading zeros
seq -w 1 100
```

## Flags

| Flag | Description |
|------|-------------|
| `-s, --separator <STRING>` | Use STRING to separate numbers (default: newline) |
| `-f, --format <FORMAT>` | Use printf-style floating-point format string |
| `-w, --equal-width` | Pad with leading zeros to equal width |

## Building

```sh
cargo build --package seq --release
```
