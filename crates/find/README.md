# find

A cross-platform `find` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Flexible search predicates** — filter by name, type, size, permissions, modification time, and more
- **Glob and regex matching** — match file names with shell globs or full regular expressions
- **Size filtering** — find files by size with support for byte, kilobyte, megabyte, and gigabyte suffixes
- **Depth control** — limit traversal depth with `-maxdepth` and `-mindepth`
- **Boolean logic** — combine predicates with `-not`, `-o` (OR), and `-a` (AND)
- **Actions** — print, print0, delete, or execute commands on matched files
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# find all .rs files in current directory
find . -name "*.rs"

# find empty directories
find /tmp -type d -empty

# find files larger than 10MB modified in the last day
find . -size +10M -mtime -1

# find and delete all .tmp files
find . -name "*.tmp" -delete

# execute a command on each match
find . -name "*.log" -exec gzip {} ;
```

## Flags

| Flag | Description |
|------|-------------|
| `-maxdepth N` | Descend at most N levels |
| `-mindepth N` | Do not apply tests at levels less than N |
| `-name PATTERN` | Base of file name matches shell glob PATTERN |
| `-iname PATTERN` | Like `-name` but case-insensitive |
| `-type TYPE` | File type: `f` (file), `d` (directory), `l` (symlink) |
| `-size SPEC` | File size (`+N`, `-N`, or `N` with suffix `c`/`k`/`M`/`G`) |
| `-empty` | File is empty and is a regular file or directory |
| `-newer FILE` | File was modified more recently than FILE |
| `-path PATTERN` | File path matches shell glob PATTERN |
| `-regex PATTERN` | File path matches regular expression PATTERN |
| `-mtime N` | File was modified N*24 hours ago (`+N`, `-N`, or `N`) |
| `-mmin N` | File was modified N minutes ago (`+N`, `-N`, or `N`) |
| `-perm MODE` | File permission bits are exactly MODE (octal) |
| `-not EXPR`, `! EXPR` | Negate the following expression |
| `-o` | OR: combine with previous expression |
| `-a` | AND: combine with previous expression (implicit) |
| `-print` | Print full file name (default action) |
| `-print0` | Print full file name followed by NUL |
| `-delete` | Delete matched files |
| `-exec CMD {} ;` | Execute CMD with `{}` replaced by file path |
| `-exec CMD {} +` | Execute CMD with all matched paths appended |

## Building

```sh
cargo build --package find --release
```
