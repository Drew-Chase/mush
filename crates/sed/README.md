# sed

A cross-platform `sed` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Stream editing** — filter and transform text line by line
- **In-place editing** — modify files directly with optional backup suffix
- **Multiple scripts** — chain multiple expressions with `-e`
- **Script files** — load editing commands from a file with `-f`
- **Extended regex** — support for extended regular expressions
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Substitute first occurrence on each line
sed 's/foo/bar/' input.txt

# Global substitution
sed 's/foo/bar/g' input.txt

# In-place edit with backup
sed -i.bak 's/old/new/g' file.txt

# Delete lines matching a pattern
sed '/^#/d' config.txt

# Print only matching lines
sed -n '/error/p' logfile.txt

# Multiple expressions
sed -e 's/foo/bar/' -e 's/baz/qux/' input.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-n, --quiet, --silent` | Suppress automatic printing of pattern space |
| `-e SCRIPT, --expression=SCRIPT` | Add the script to the commands to be executed |
| `-f FILE, --file=FILE` | Add the contents of FILE to the commands to be executed |
| `-i[SUFFIX], --in-place[=SUFFIX]` | Edit files in place, optionally creating a backup with SUFFIX |
| `-r, -E, --regexp-extended` | Use extended regular expressions |
| `-s, --separate` | Consider files as separate rather than a single continuous stream |

## Building

```sh
cargo build --package sed --release
```
