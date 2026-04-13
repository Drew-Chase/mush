# cut

A cross-platform `cut` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Byte, character, and field selection** — extract portions of each line by bytes, characters, or delimited fields
- **Custom delimiters** — specify input delimiter for field mode
- **Output delimiter** — control how selected ranges are joined
- **Complement mode** — select everything except the specified ranges
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Cut fields 1 and 3 from a CSV
cut -d, -f1,3 data.csv

# Extract characters 1-10
cut -c1-10 file.txt

# Extract bytes 5 through end of line
cut -b5- file.txt

# Use a custom output delimiter
cut -d: -f1,3 --output-delimiter=' | ' /etc/passwd

# Select complement (everything except field 2)
cut -d, -f2 --complement data.csv

# Suppress lines without delimiters
cut -d: -f1 -s file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-b, --bytes=LIST` | Select only these bytes |
| `-c, --characters=LIST` | Select only these characters |
| `-f, --fields=LIST` | Select only these fields |
| `-d, --delimiter=DELIM` | Use DELIM instead of TAB as the field delimiter |
| `--output-delimiter=STRING` | Use STRING as the output delimiter |
| `-s, --only-delimited` | Do not print lines not containing delimiters |
| `--complement` | Complement the set of selected bytes, characters, or fields |

## Building

```sh
cargo build --package cut --release
```
