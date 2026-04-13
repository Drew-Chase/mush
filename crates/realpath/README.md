# realpath

A cross-platform `realpath` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Resolve absolute paths** — print the fully resolved pathname for any file
- **Symlink resolution control** — optionally skip symlink resolution
- **Flexible canonicalization** — require all components to exist, or allow missing ones
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print the resolved absolute path
realpath ./relative/path

# Resolve without following symlinks
realpath -s ./some/link

# Require all path components to exist
realpath -e /usr/local/bin/tool

# Allow missing path components
realpath -m /future/path/that/does/not/exist
```

## Flags

| Flag | Description |
|------|-------------|
| `-e, --canonicalize-existing` | All components of the path must exist |
| `-m, --canonicalize-missing` | No component of the path needs to exist |
| `-s, --strip, --no-symlinks` | Do not resolve symlinks |
| `-q, --quiet` | Suppress most error messages |
| `-z, --zero` | End each output line with NUL instead of newline |

## Building

```sh
cargo build --package realpath --release
```
