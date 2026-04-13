# date

A cross-platform `date` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Flexible formatting** — display dates and times in custom formats via `+FORMAT`
- **Standard formats** — ISO 8601, RFC 2822 (email), and RFC 3339 output
- **Date parsing** — display a specific date string instead of the current time
- **File timestamps** — show the last modification time of a file
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print the current date and time
date

# Custom format
date +"%Y-%m-%d %H:%M:%S"

# ISO 8601 output
date -I

# Display date in UTC
date -u

# Show modification time of a file
date -r file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-d, --date` | Display the given date string instead of the current time |
| `-I, --iso-8601` | Output in ISO 8601 format (optionally specify precision: date, hours, minutes, seconds, ns) |
| `-R, --rfc-email` | Output in RFC 2822 format |
| `--rfc-3339` | Output in RFC 3339 format (date, seconds, or ns) |
| `-r, --reference` | Display the last modification time of the given file |
| `-u, --utc` | Use Coordinated Universal Time (UTC) |
| `+FORMAT` | Format string controlling the output |

## Building

```sh
cargo build --package date --release
```
