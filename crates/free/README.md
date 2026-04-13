# free

A cross-platform `free` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Memory overview** — display total, used, free, shared, and available memory
- **Swap reporting** — show swap space usage alongside physical memory
- **Unit selection** — output in bytes, kilobytes, megabytes, gigabytes, or human-readable
- **Wide mode** — separate buffers and cache into distinct columns
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Show memory usage
free

# Human-readable output
free -h

# Include a total line
free -ht

# Wide output with separate buffers/cache columns
free -w
```

## Flags

| Flag | Description |
|------|-------------|
| `-b` | Display amounts in bytes |
| `-k` | Display amounts in kilobytes (default) |
| `-m` | Display amounts in megabytes |
| `-g` | Display amounts in gigabytes |
| `-h, --human` | Human-readable output with automatic unit selection |
| `--si` | Use powers of 1000 instead of 1024 |
| `-t, --total` | Append a total line summing RAM and swap |
| `-w, --wide` | Wide output with separate buffers and cache columns |

## Building

```sh
cargo build --package free --release
```
