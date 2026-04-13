# sort

A cross-platform `sort` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Multiple sort modes** — numeric, human-numeric, dictionary, and case-insensitive sorting
- **Key-based sorting** — sort by specific fields with `-k`
- **Unique filtering** — output only the first of equal runs
- **Sorted check** — verify that input is already sorted without modifying it
- **Merge mode** — merge pre-sorted files efficiently
- **Stable sort** — preserve original order of equal elements
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Sort a file alphabetically
sort file.txt

# Reverse numeric sort
sort -rn numbers.txt

# Sort by the second field, colon-delimited
sort -t: -k2 data.txt

# Remove duplicate lines
sort -u file.txt

# Sort human-readable sizes (2K, 1G, etc.)
sort -h sizes.txt

# Check if a file is already sorted
sort -c file.txt

# Write output to a file
sort -o sorted.txt file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-r, --reverse` | Reverse the result of comparisons |
| `-n, --numeric-sort` | Compare according to string numerical value |
| `-h, --human-numeric-sort` | Compare human-readable numbers (e.g., 2K 1G) |
| `-f, --ignore-case` | Fold lower case to upper case characters |
| `-d, --dictionary-order` | Consider only blanks and alphanumeric characters |
| `-b, --ignore-leading-blanks` | Ignore leading blanks |
| `-k, --key=KEYDEF` | Sort via a key; KEYDEF gives location and type |
| `-t, --field-separator=SEP` | Use SEP instead of non-blank to blank transition |
| `-u, --unique` | Output only the first of an equal run |
| `-s, --stable` | Stabilize sort by disabling last-resort comparison |
| `-o, --output=FILE` | Write result to FILE instead of standard output |
| `-c, --check` | Check for sorted input; do not sort |
| `-m, --merge` | Merge already sorted files; do not sort |

## Building

```sh
cargo build --package sort --release
```
