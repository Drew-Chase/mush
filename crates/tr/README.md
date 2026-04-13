# tr

A cross-platform `tr` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Character translation** — map characters from one set to another
- **Character deletion** — remove characters from input
- **Squeeze repeats** — replace repeated characters with a single occurrence
- **Complement mode** — operate on characters not in the specified set
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Translate lowercase to uppercase
echo "hello" | tr 'a-z' 'A-Z'

# Delete all digits
echo "abc123" | tr -d '0-9'

# Squeeze repeated spaces into one
echo "hello    world" | tr -s ' '

# Delete complement (keep only digits)
echo "abc123def" | tr -cd '0-9'

# Translate and squeeze
echo "hello" | tr -s 'l' 'L'
```

## Flags

| Flag | Description |
|------|-------------|
| `-c, -C, --complement` | Use the complement of SET1 |
| `-d, --delete` | Delete characters in SET1, do not translate |
| `-s, --squeeze-repeats` | Replace each sequence of repeated characters with a single occurrence |
| `-t, --truncate-set1` | First truncate SET1 to the length of SET2 |

## Building

```sh
cargo build --package tr --release
```
