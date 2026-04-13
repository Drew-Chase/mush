# awk

A cross-platform `awk` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Pattern-action processing** — scan input and apply rules to matching lines
- **Field splitting** — automatic splitting of input records into fields
- **Custom separators** — configurable field separator via `-F`
- **Variables** — assign variables from the command line with `-v`
- **Script files** — load awk programs from a file with `-f`
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print the second field of each line
awk '{print $2}' data.txt

# Use colon as field separator
awk -F: '{print $1, $3}' /etc/passwd

# Sum a column
awk '{sum += $1} END {print sum}' numbers.txt

# Filter lines by pattern
awk '/error/ {print $0}' logfile.txt

# Set a variable from the command line
awk -v threshold=100 '$1 > threshold {print}' data.txt

# Load program from a file
awk -f script.awk input.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-F SEP, --field-separator=SEP` | Set the input field separator |
| `-v VAR=VAL, --assign=VAR=VAL` | Assign a value to a variable before execution |
| `-f FILE, --file=FILE` | Read the awk program from FILE |

## Building

```sh
cargo build --package awk --release
```
