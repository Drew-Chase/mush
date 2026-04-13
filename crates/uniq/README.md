# uniq

A cross-platform `uniq` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Duplicate filtering** — filter or report adjacent matching lines
- **Count occurrences** — prefix lines with the number of consecutive duplicates
- **Flexible matching** — skip fields, skip characters, or limit comparison width
- **Case-insensitive** — optionally ignore case when comparing
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Remove adjacent duplicate lines
sort file.txt | uniq

# Count occurrences of each line
sort file.txt | uniq -c

# Show only duplicate lines
sort file.txt | uniq -d

# Show only unique lines (no duplicates)
sort file.txt | uniq -u

# Case-insensitive comparison
sort file.txt | uniq -i

# Skip the first field and compare
sort file.txt | uniq -f 1
```

## Flags

| Flag | Description |
|------|-------------|
| `-c, --count` | Prefix lines by the number of occurrences |
| `-d, --repeated` | Only print duplicate lines, one for each group |
| `-D, --all-repeated` | Print all duplicate lines |
| `-u, --unique` | Only print unique lines |
| `-i, --ignore-case` | Ignore differences in case when comparing |
| `-f NUM, --skip-fields=NUM` | Avoid comparing the first NUM fields |
| `-s NUM, --skip-chars=NUM` | Avoid comparing the first NUM characters |
| `-w NUM, --check-chars=NUM` | Compare no more than NUM characters in lines |

## Building

```sh
cargo build --package uniq --release
```
