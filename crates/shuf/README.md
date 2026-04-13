# shuf

A cross-platform `shuf` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Random permutations** — shuffle lines from stdin or files
- **Echo mode** — shuffle command-line arguments directly
- **Input ranges** — generate and shuffle a range of integers
- **Head count** — output only a specified number of lines
- **Repeat mode** — allow output values to be repeated
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Shuffle lines from a file
shuf input.txt

# Shuffle command-line arguments
shuf -e apple banana cherry

# Generate a random number from 1 to 100
shuf -i 1-100 -n 1

# Pick 5 random lines from stdin
cat data.txt | shuf -n 5

# Generate 10 random values with repetition
shuf -i 1-6 -r -n 10
```

## Flags

| Flag | Description |
|------|-------------|
| `-e, --echo` | Treat each argument as an input line |
| `-i, --input-range <LO-HI>` | Treat each number from LO through HI as an input line |
| `-n, --head-count <COUNT>` | Output at most COUNT lines |
| `-r, --repeat` | Allow output values to be repeated |

## Building

```sh
cargo build --package shuf --release
```
