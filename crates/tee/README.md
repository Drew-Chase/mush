# tee

A cross-platform `tee` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Stdin duplication** — copy standard input to both standard output and one or more files
- **Append mode** — append to files instead of overwriting with `-a`
- **Multiple outputs** — write to any number of files simultaneously
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# write stdin to a file and stdout
echo "hello" | tee output.txt

# append to a file instead of overwriting
echo "another line" | tee -a output.txt

# write to multiple files at once
echo "data" | tee file1.txt file2.txt file3.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-a, --append` | Append to the given FILEs, do not overwrite |

## Building

```sh
cargo build --package tee --release
```
