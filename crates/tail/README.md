# tail

A cross-platform `tail` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Last N lines** — print the last 10 lines of each file by default, configurable with `-n`
- **Byte mode** — print the last N bytes instead of lines
- **Follow mode** — monitor a file for appended data in real time with `-f`
- **Multi-file headers** — automatically prints filename headers when multiple files are given
- **Stdin support** — read from standard input when no file is given or when `-` is specified
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print the last 10 lines
tail file.txt

# Print the last 20 lines
tail -n 20 file.txt

# Follow a log file as it grows
tail -f /var/log/app.log

# Print the last 256 bytes
tail -c 256 file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-n, --lines NUM` | Output the last NUM lines instead of last 10 |
| `-c, --bytes NUM` | Output the last NUM bytes |
| `-f, --follow` | Output appended data as the file grows |
| `-q, --quiet` | Never output headers giving file names |
| `-v, --verbose` | Always output headers giving file names |

## Building

```sh
cargo build --package tail --release
```
