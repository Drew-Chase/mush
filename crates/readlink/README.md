# readlink

A cross-platform `readlink` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Print symlink target** — resolve where a symbolic link points
- **Canonicalize paths** — resolve all symlinks and produce absolute paths
- **Flexible canonicalization** — require all components to exist, or allow missing ones
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print the target of a symlink
readlink mylink

# Get the fully resolved canonical path
readlink -f ./some/relative/../path

# Canonicalize, requiring all components to exist
readlink -e /usr/local/bin/tool

# Print without trailing newline
readlink -n mylink
```

## Flags

| Flag | Description |
|------|-------------|
| `-f, --canonicalize` | Canonicalize by following every symlink; all but the last component must exist |
| `-e, --canonicalize-existing` | Canonicalize, requiring all components to exist |
| `-m, --canonicalize-missing` | Canonicalize without requiring any component to exist |
| `-n, --no-newline` | Do not output a trailing newline |
| `-z, --zero` | End each output line with NUL instead of newline |

## Building

```sh
cargo build --package readlink --release
```
