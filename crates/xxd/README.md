# xxd

A cross-platform `xxd` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Hex dump** — display file contents in hexadecimal with ASCII sidebar
- **Binary dump** — display file contents as binary digits
- **Plain hex output** — output a continuous hex string without formatting
- **C include style** — output as a C array definition for embedding in source code
- **Reverse operation** — convert a hex dump back into binary
- **Configurable layout** — control columns, grouping, offset, and length
- **Stdin support** — read from standard input when no file is given
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Hex dump a file
xxd file.bin

# Dump first 64 bytes with uppercase hex
xxd -l 64 -u file.bin

# Plain hex output
xxd -p file.bin

# Output as C include array
xxd -i file.bin

# Binary digit dump
xxd -b file.bin

# Reverse a hex dump back to binary
xxd -r dump.hex output.bin

# Start at offset 128, show 4 bytes per group
xxd -s 128 -g 4 file.bin
```

## Flags

| Flag | Description |
|------|-------------|
| `-c NUM` | Number of octets per line (default: 16) |
| `-g NUM` | Group size in bytes (default: 2) |
| `-l NUM` | Stop after NUM octets |
| `-s NUM` | Start at byte offset NUM |
| `-u` | Use upper case hex letters |
| `-p` | Output in plain hex dump style |
| `-r` | Reverse: convert hex dump to binary |
| `-i` | Output in C include file style |
| `-b` | Binary digit dump |

## Building

```sh
cargo build --package xxd --release
```
