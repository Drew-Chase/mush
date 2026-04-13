# touch

A cross-platform `touch` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Create files** — create new empty files if they do not exist
- **Update timestamps** — set access and modification times to the current time
- **Selective updates** — change only access time or only modification time
- **Reference file** — copy timestamps from another file with `-r`
- **Custom date** — set timestamps to a specific date string with `-d`
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# create a new file or update timestamps
touch newfile.txt

# update only the modification time
touch -m existing.txt

# do not create the file if it doesn't exist
touch -c maybe_missing.txt

# set timestamps from a reference file
touch -r reference.txt target.txt

# set a specific date
touch -d "2025-01-15 12:00:00" file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-a` | Change only the access time |
| `-m` | Change only the modification time |
| `-c, --no-create` | Do not create any files |
| `-r, --reference FILE` | Use this file's times instead of current time |
| `-d, --date STRING` | Parse STRING and use it instead of current time |

## Building

```sh
cargo build --package touch --release
```
