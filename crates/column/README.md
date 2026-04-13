# column

A cross-platform `column` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Table formatting** — arrange input into neatly aligned columns
- **Custom delimiters** — specify input separator for table mode
- **Output separator** — control the string placed between columns
- **Named columns** — provide column headers with `-N`
- **Right-align** — selectively right-align columns
- **JSON output** — emit structured JSON from tabular data
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Fill columns (default)
column file.txt

# Create a table from delimiter-separated input
column -t -s, data.csv

# Specify column names
column -t -s, -N "Name,Age,City" data.csv

# Custom output separator
column -t -s: -o " | " /etc/passwd

# Right-align columns 2 and 3
column -t -s, -R 2,3 data.csv

# JSON output
column -t -s, -N "name,value" -J data.csv
```

## Flags

| Flag | Description |
|------|-------------|
| `-t, --table` | Create a table; determine the number of columns automatically |
| `-s, --separator=STRING` | Specify the input delimiter characters for table mode |
| `-o, --output-separator=STRING` | Specify the output column separator for table mode |
| `-c, --columns=NUM` | Output is formatted for a display NUM columns wide |
| `-N, --table-columns=NAMES` | Specify comma-separated column names for table header |
| `-R, --table-right=COLUMNS` | Right-align text in the specified columns |
| `-J, --json` | Use JSON output format for table mode |

## Building

```sh
cargo build --package column --release
```
