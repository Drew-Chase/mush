# comm

A cross-platform `comm` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Three-column output** — lines unique to file 1, lines unique to file 2, and lines common to both
- **Column suppression** — selectively hide any of the three columns
- **Case-insensitive** — optionally ignore case when comparing lines
- **Custom delimiter** — specify the string used to separate columns
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Compare two sorted files (three-column output)
comm file1.txt file2.txt

# Show only lines common to both files
comm -12 file1.txt file2.txt

# Show only lines unique to file 1
comm -23 file1.txt file2.txt

# Show only lines unique to file 2
comm -13 file1.txt file2.txt

# Case-insensitive comparison
comm -i file1.txt file2.txt

# Custom output delimiter
comm --output-delimiter="| " file1.txt file2.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-1` | Suppress column 1 (lines unique to file 1) |
| `-2` | Suppress column 2 (lines unique to file 2) |
| `-3` | Suppress column 3 (lines common to both files) |
| `-i, --ignore-case` | Ignore case when comparing lines |
| `--output-delimiter=STRING` | Separate columns with STRING |

## Building

```sh
cargo build --package comm --release
```
