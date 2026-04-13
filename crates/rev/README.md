# rev

A cross-platform `rev` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Line reversal** — reverse each line of input character by character
- **Unicode-aware** — correctly handles multi-byte characters
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Reverse each line of a file
rev file.txt

# Reverse piped input
echo "hello world" | rev

# Reverse lines from multiple files
rev file1.txt file2.txt
```

## Flags

No flags. `rev` reads from the specified files or standard input and reverses each line.

## Building

```sh
cargo build --package rev --release
```
