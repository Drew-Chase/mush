# ln

A cross-platform `ln` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Hard and symbolic links** — create both hard links and symbolic links
- **Force overwrite** — replace existing destination files with `-f`
- **Interactive mode** — prompt before removing existing destinations
- **Verbose output** — print the name of each linked file
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# create a hard link
ln target.txt link_name.txt

# create a symbolic link
ln -s /path/to/target link_name

# force overwrite existing link
ln -sf new_target existing_link

# verbose symbolic link creation
ln -sv target.txt link.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-s, --symbolic` | Make symbolic links instead of hard links |
| `-f, --force` | Remove existing destination files |
| `-i, --interactive` | Prompt whether to remove destinations |
| `-v, --verbose` | Print name of each linked file |
| `-n, --no-dereference` | Treat LINK_NAME as a normal file if it is a symbolic link to a directory |

## Building

```sh
cargo build --package ln --release
```
