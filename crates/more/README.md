# more

A cross-platform `more` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Screen-at-a-time paging** — page through text one screenful at a time
- **Blank line squeezing** — collapse multiple adjacent blank lines into one
- **Configurable page size** — set the number of lines per screenful
- **Start at line** — begin displaying at a specific line number
- **Stdin support** — read from standard input when no file is given
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Page through a file
more file.txt

# Squeeze blank lines
more -s file.txt

# Set 30 lines per screenful
more -n 30 file.txt

# Start displaying at line 50
more --start-line 50 file.txt

# Pipe input
cat largefile.txt | more
```

## Flags

| Flag | Description |
|------|-------------|
| `-s` | Squeeze multiple adjacent blank lines into one |
| `-n NUM` | Lines per screenful (default: terminal height - 1) |
| `--start-line NUM` | Start displaying at line number NUM |

## Building

```sh
cargo build --package more --release
```
