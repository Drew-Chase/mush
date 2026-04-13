# nproc

A cross-platform `nproc` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **CPU count** — print the number of available processing units
- **All processors** — optionally report all installed processors, ignoring cgroup/affinity limits
- **Subtract cores** — reduce the reported count by a specified number
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print available processing units
nproc

# Print all installed processors
nproc --all

# Report available processors minus 2
nproc --ignore 2
```

## Flags

| Flag | Description |
|------|-------------|
| `--all` | Print the number of installed processors, ignoring restrictions |
| `--ignore NUM` | Subtract NUM from the count of available processors |

## Building

```sh
cargo build --package nproc --release
```
