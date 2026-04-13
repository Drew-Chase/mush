# tree

A cross-platform `tree` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Tree-format listing** — display directory contents as an indented tree
- **Depth limiting** — control traversal depth with `-L`
- **Pattern filtering** — include or exclude files by glob pattern
- **File metadata** — show file sizes, human-readable sizes, and modification dates
- **JSON output** — structured output with `-J` for programmatic use
- **Colorized output** — syntax-highlighted file names with `-C`
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# list current directory as a tree
tree

# show only 2 levels deep, directories first
tree -L 2 --dirsfirst

# include hidden files with human-readable sizes
tree -a -h

# exclude node_modules, output as JSON
tree -I node_modules -J

# directories only with modification dates
tree -d -D /var/log
```

## Flags

| Flag | Description |
|------|-------------|
| `-a` | Include hidden files |
| `-d, --dirs-only` | List directories only |
| `-f, --full-path` | Print the full path prefix for each file |
| `-L, --level NUM` | Descend only NUM directories deep |
| `-I, --exclude PATTERN` | Exclude files matching pattern |
| `-P, --pattern PATTERN` | List only files matching pattern |
| `-s, --size` | Print the size of each file |
| `-h, --human-readable` | Print size in human-readable format |
| `-D, --date` | Print the date of last modification |
| `--dirsfirst` | List directories before files |
| `--noreport` | Omit the file and directory report at end |
| `-C, --color` | Turn colorization on always |
| `-J, --json` | Output as JSON |

## Building

```sh
cargo build --package tree --release
```
