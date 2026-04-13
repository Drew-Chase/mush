# dirname

A cross-platform `dirname` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Strip last component** — output the directory portion of a pathname
- **Multiple arguments** — process several paths at once
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Get the directory of a file path
dirname /usr/local/bin/tool
# => /usr/local/bin

# Handles trailing slashes
dirname /usr/local/bin/
# => /usr/local

# Multiple arguments
dirname /home/user/file.txt /etc/config.conf
# => /home/user
# => /etc
```

## Flags

| Flag | Description |
|------|-------------|
| `-z, --zero` | End each output line with NUL instead of newline |

## Building

```sh
cargo build --package dirname --release
```
