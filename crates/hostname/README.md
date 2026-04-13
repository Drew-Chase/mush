# hostname

A cross-platform `hostname` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Display hostname** — print the system's current host name
- **Short name** — show just the name up to the first dot
- **FQDN** — display the fully qualified domain name
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print the hostname
hostname

# Print the short hostname
hostname -s

# Print the fully qualified domain name
hostname -f
```

## Flags

| Flag | Description |
|------|-------------|
| `-s, --short` | Print the short hostname (up to the first dot) |
| `-f, --fqdn, --long` | Print the fully qualified domain name |

## Building

```sh
cargo build --package hostname --release
```
