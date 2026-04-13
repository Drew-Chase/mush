# df

A cross-platform `df` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Disk space reporting** — show usage for all mounted filesystems
- **Human-readable sizes** — display sizes in KB, MB, GB, etc.
- **Filesystem filtering** — limit output to specific filesystem types
- **Grand total** — optionally append a summary row
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Show disk space for all filesystems
df

# Human-readable output
df -h

# Show filesystem type
df -T

# Only show ext4 filesystems
df -t ext4

# Include a total line
df -h --total
```

## Flags

| Flag | Description |
|------|-------------|
| `-h, --human-readable` | Print sizes in powers of 1024 (e.g. 1.5G) |
| `-H, --si` | Print sizes in powers of 1000 |
| `-T, --print-type` | Include the filesystem type in the output |
| `-t TYPE` | Limit output to filesystems of the given type |
| `-a, --all` | Include pseudo, duplicate, and inaccessible filesystems |
| `--total` | Append a grand total row |

## Building

```sh
cargo build --package df --release
```
