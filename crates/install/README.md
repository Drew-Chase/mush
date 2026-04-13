# install

A cross-platform `install` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Copy with permissions** — copy files and set permission mode in one step
- **Directory creation** — create destination directories and all leading components
- **Compare mode** — skip copying when source and destination are identical
- **Target directory** — install multiple files into a single directory with `-t`
- **Verbose output** — print the name of each file as it is installed
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# install a binary with specific permissions
install -m 755 myapp /usr/local/bin/myapp

# create directories
install -d /opt/myapp/config /opt/myapp/data

# install with leading directory creation
install -D config.toml /opt/myapp/config/config.toml

# install multiple files into a target directory
install -t /usr/local/bin/ tool1 tool2 tool3

# verbose install, skip if identical
install -Cv src/app dest/app
```

## Flags

| Flag | Description |
|------|-------------|
| `-d` | Create all leading directories; treat DEST as a directory |
| `-m, --mode MODE` | Set permission mode (as in chmod) |
| `-v, --verbose` | Print the name of each file as it is installed |
| `-C, --compare` | Compare each pair of files; skip copy if identical |
| `-D` | Create all leading components of DEST |
| `-t, --target-directory DIR` | Copy all SOURCE arguments into DIR |

## Building

```sh
cargo build --package install --release
```
