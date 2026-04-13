# stat

A cross-platform `stat` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **File status display** — show detailed file metadata including size, permissions, timestamps, and inode info
- **Custom format strings** — control output with printf-style format specifiers via `-c`
- **Terse output** — machine-readable single-line output with `-t`
- **Symlink dereferencing** — follow symbolic links to report on the target file
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# display status of a file
stat myfile.txt

# custom format showing size and permissions
stat -c "%s %a %n" myfile.txt

# follow symlinks
stat -L link_to_file

# terse machine-readable output
stat -t myfile.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-L, --dereference` | Follow symbolic links |
| `-c, --format FORMAT` | Use the specified FORMAT instead of the default |
| `-t, --terse` | Print the information in terse form |

## Building

```sh
cargo build --package stat --release
```
