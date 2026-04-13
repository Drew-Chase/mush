# sleep

A cross-platform `sleep` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Duration suffixes** — supports seconds (s), minutes (m), hours (h), and days (d)
- **Default to seconds** — bare numbers are treated as seconds
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Sleep for 5 seconds
sleep 5

# Sleep for 2 minutes
sleep 2m

# Sleep for 1 hour
sleep 1h

# Sleep for half a second
sleep 0.5s
```

## Building

```sh
cargo build --package sleep --release
```
