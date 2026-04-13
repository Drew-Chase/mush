# join

A cross-platform `join` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Relational join** — join lines from two files on a common field
- **Custom join fields** — specify which field to join on for each file
- **Unpairable lines** — optionally print lines that have no match
- **Custom output format** — select and reorder output fields
- **Case-insensitive** — optionally ignore case when comparing join fields
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Join two sorted files on their first field
join file1.txt file2.txt

# Join on field 2 of file1 and field 3 of file2
join -1 2 -2 3 file1.txt file2.txt

# Use comma as field separator
join -t, file1.csv file2.csv

# Also print unpairable lines from file 1
join -a 1 file1.txt file2.txt

# Replace missing fields with a placeholder
join -e "N/A" -o 1.1,2.2 file1.txt file2.txt

# Case-insensitive join
join -i file1.txt file2.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-1 FIELD` | Join on this field of file 1 |
| `-2 FIELD` | Join on this field of file 2 |
| `-t CHAR` | Use CHAR as the input and output field separator |
| `-a FILENUM` | Also print unpairable lines from file FILENUM |
| `-v FILENUM` | Like `-a`, but suppress joined output lines |
| `-e STRING` | Replace missing input fields with STRING |
| `-o FORMAT` | Obey FORMAT while constructing output line |
| `-i, --ignore-case` | Ignore differences in case when comparing fields |

## Building

```sh
cargo build --package join --release
```
