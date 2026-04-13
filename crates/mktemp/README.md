# mktemp

A cross-platform `mktemp` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Safe temp file creation** — create temporary files with unique names atomically
- **Temporary directories** — create temporary directories with `-d`
- **Custom templates** — control the naming pattern with a template argument
- **Custom suffix** — append a suffix to the generated name
- **Custom base directory** — specify the parent directory with `-p`
- **Dry-run mode** — print the name without creating the file
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# create a temporary file and print its path
mktemp

# create a temporary directory
mktemp -d

# use a custom template (X's are replaced with random characters)
mktemp /tmp/myapp.XXXXXX

# create temp file with a specific suffix
mktemp --suffix .log

# create temp file in a specific directory
mktemp -p /var/tmp

# dry-run: print the name without creating the file
mktemp -u
```

## Flags

| Flag | Description |
|------|-------------|
| `-d, --directory` | Create a directory instead of a file |
| `-u, --dry-run` | Print the name without creating the file |
| `-q, --quiet` | Suppress error messages |
| `-p, --tmpdir DIR` | Use DIR as the base directory |
| `--suffix SUFFIX` | Append SUFFIX to the template |

## Building

```sh
cargo build --package mktemp --release
```
