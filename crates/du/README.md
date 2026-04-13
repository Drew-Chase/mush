# du

A cross-platform `du` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Disk usage estimation** — summarize space used by files and directories
- **Human-readable sizes** — display sizes in KB, MB, GB, etc.
- **Depth control** — limit how deep into the directory tree to report
- **Grand total** — optionally print a combined total
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Show disk usage of the current directory
du

# Human-readable summary
du -sh /path/to/dir

# Show all files, not just directories
du -ah

# Limit depth to 2 levels
du -h -d 2

# Print a grand total
du -ch /path/to/dir
```

## Flags

| Flag | Description |
|------|-------------|
| `-h, --human-readable` | Print sizes in powers of 1024 (e.g. 1.5G) |
| `-s, --summarize` | Display only a total for each argument |
| `-a, --all` | Show counts for all files, not just directories |
| `-c, --total` | Produce a grand total |
| `-d, --max-depth` | Print totals only for directories N or fewer levels deep |
| `--apparent-size` | Print apparent sizes rather than disk usage |
| `-b, --bytes` | Equivalent to `--apparent-size --block-size=1` |
| `-k` | Display sizes in kilobytes |
| `-m` | Display sizes in megabytes |

## Building

```sh
cargo build --package du --release
```
