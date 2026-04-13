# chgrp

A cross-platform `chgrp` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Change group ownership** — set the group of files and directories
- **Recursive** — apply changes to directories and their contents
- **Reference file** — copy group ownership from another file
- **Symlink handling** — control whether symlinks are dereferenced
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Change group of a file
chgrp staff file.txt

# Recursively change group
chgrp -R developers project/

# Copy group from a reference file
chgrp --reference=other.txt file.txt

# Verbose output
chgrp -v staff *.conf
```

## Flags

| Flag | Description |
|------|-------------|
| `-R, --recursive` | Operate on files and directories recursively |
| `-v, --verbose` | Output a diagnostic for every file processed |
| `-c, --changes` | Like verbose but report only when a change is made |
| `-f, --quiet, --silent` | Suppress most error messages |
| `-h, --no-dereference` | Affect symbolic links instead of their targets |
| `--reference FILE` | Use group of FILE instead of specifying a group name |

## Building

```sh
cargo build --package chgrp --release
```
