# file

A cross-platform `file` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **File type detection** — identify files by inspecting content and metadata
- **MIME type output** — display MIME type strings with `-i` or `--mime-type`
- **Brief mode** — suppress filenames in output for scripting use
- **Symlink dereferencing** — follow symbolic links to classify the target file
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# determine the type of a file
file document.pdf

# output MIME type only
file --mime-type image.png

# brief output without filenames
file -b *.txt

# follow symlinks
file -L symlink_to_file
```

## Flags

| Flag | Description |
|------|-------------|
| `-b, --brief` | Do not prepend filenames to output lines |
| `-i, --mime` | Output MIME type strings |
| `--mime-type` | Output the MIME type only |
| `-L, --dereference` | Follow symbolic links |

## Building

```sh
cargo build --package file --release
```
