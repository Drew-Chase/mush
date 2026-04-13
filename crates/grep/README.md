# grep

A cross-platform `grep` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Pattern matching** — search for lines matching regular expressions or fixed strings
- **Recursive search** — search directories recursively with include/exclude filters
- **Context lines** — show lines before, after, or around matches
- **Multiple output modes** — print matches, counts, filenames, or suppress output for exit-code usage
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Search for a pattern in a file
grep "error" logfile.txt

# Case-insensitive recursive search
grep -ri "todo" src/

# Show line numbers and 2 lines of context
grep -n -C 2 "panic" src/main.rs

# Count matches per file
grep -rc "use std" src/

# List files that do NOT match
grep -rL "unsafe" src/
```

## Flags

| Flag | Description |
|------|-------------|
| `-i, --ignore-case` | Case-insensitive matching |
| `-v, --invert-match` | Select non-matching lines |
| `-w, --word-regexp` | Match whole words only |
| `-x, --line-regexp` | Match whole lines only |
| `-c, --count` | Print a count of matching lines per file |
| `-l, --files-with-matches` | Print only names of files with matches |
| `-L, --files-without-match` | Print only names of files without matches |
| `-n, --line-number` | Prefix each line with its line number |
| `-H, --with-filename` | Print the filename for each match |
| `-h, --no-filename` | Suppress the filename prefix |
| `-o, --only-matching` | Print only the matched parts of a line |
| `-q, --quiet` | Suppress all output; exit with status 0 on match |
| `-r, -R, --recursive` | Recursively search directories |
| `-A NUM` | Print NUM lines of trailing context after matches |
| `-B NUM` | Print NUM lines of leading context before matches |
| `-C NUM` | Print NUM lines of context before and after matches |
| `-m NUM, --max-count` | Stop reading a file after NUM matches |
| `--color[=WHEN]` | Highlight matches: `always`, `auto`, `never` |
| `-F, --fixed-strings` | Interpret pattern as a fixed string |
| `-E, --extended-regexp` | Interpret pattern as an extended regex |
| `--include=GLOB` | Search only files matching GLOB |
| `--exclude=GLOB` | Skip files matching GLOB |
| `--exclude-dir=GLOB` | Skip directories matching GLOB |

## Building

```sh
cargo build --package grep --release
```
