# base64

A cross-platform `base64` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Encode and decode** — convert data to and from Base64 representation
- **Line wrapping** — configurable column width for encoded output
- **Garbage tolerance** — optionally ignore non-alphabet characters when decoding
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Encode a file
base64 file.bin

# Decode Base64 input
base64 -d encoded.txt

# Encode with custom line wrap width
base64 -w 60 file.bin
```

## Flags

| Flag | Description |
|------|-------------|
| `-d, --decode` | Decode Base64 input |
| `-i, --ignore-garbage` | Ignore non-alphabet characters when decoding |
| `-w, --wrap COLS` | Wrap encoded lines after COLS characters (default 76, 0 disables) |

## Building

```sh
cargo build --package base64 --release
```
