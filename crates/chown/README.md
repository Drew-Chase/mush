# chown

A cross-platform `chown` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Change owner and group** — set file ownership with `OWNER[:GROUP]` syntax
- **Recursive** — apply changes to directories and their contents
- **Reference file** — copy ownership from another file
- **Symlink handling** — control whether symlinks are dereferenced
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Change owner of a file
chown alice file.txt

# Change owner and group
chown alice:staff file.txt

# Recursively change ownership
chown -R alice:staff project/

# Copy ownership from a reference file
chown --reference=other.txt file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-R, --recursive` | Operate on files and directories recursively |
| `-v, --verbose` | Output a diagnostic for every file processed |
| `-c, --changes` | Like verbose but report only when a change is made |
| `-f, --quiet, --silent` | Suppress most error messages |
| `-h, --no-dereference` | Affect symbolic links instead of their targets |
| `--reference FILE` | Use ownership of FILE instead of specifying values |

## Building

```sh
cargo build --package chown --release
```
