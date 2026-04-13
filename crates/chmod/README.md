# chmod

A cross-platform `chmod` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Octal modes** — set permissions with numeric notation (e.g. `755`, `644`)
- **Symbolic modes** — use human-readable expressions (e.g. `u+x`, `g-w`, `o=r`, `a+rw`)
- **Recursive** — apply changes to directories and their contents
- **Verbose and changes output** — see what gets modified
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Make a script executable
chmod +x script.sh

# Set exact permissions with octal notation
chmod 755 mydir

# Remove write permission for group and others
chmod go-w file.txt

# Recursively set permissions
chmod -R 644 docs/
```

## Flags

| Flag | Description |
|------|-------------|
| `-R, --recursive` | Change files and directories recursively |
| `-v, --verbose` | Output a diagnostic for every file processed |
| `-c, --changes` | Like verbose but report only when a change is made |
| `-f, --quiet, --silent` | Suppress most error messages |

## Building

```sh
cargo build --package chmod --release
```
