# yes

A cross-platform `yes` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Default output** — outputs "y" repeatedly when called with no arguments
- **Custom string** — outputs any specified string indefinitely
- **High throughput** — buffered output for efficient piping
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Output "y" indefinitely
yes

# Output a custom string
yes "I agree"

# Pipe to a command that expects confirmation
yes | rm -i *.tmp
```

## Building

```sh
cargo build --package yes --release
```
