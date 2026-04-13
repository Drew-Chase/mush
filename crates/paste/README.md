# paste

A cross-platform `paste` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Merge lines** — combine corresponding lines from multiple files side by side
- **Serial mode** — paste all lines of each file on a single line
- **Custom delimiters** — specify one or more delimiters to cycle through
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Merge lines from two files side by side (tab-delimited)
paste file1.txt file2.txt

# Use a comma as the delimiter
paste -d, file1.txt file2.txt

# Serialize: merge all lines of each file into one line
paste -s file.txt

# Combine stdin with a file
cat names.txt | paste - ages.txt

# Use multiple delimiters (cycles through them)
paste -d',;' file1.txt file2.txt file3.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-d, --delimiters=LIST` | Reuse characters from LIST instead of TABs |
| `-s, --serial` | Paste one file at a time instead of in parallel |

## Building

```sh
cargo build --package paste --release
```
