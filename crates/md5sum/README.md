# md5sum

A cross-platform `md5sum` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Compute MD5 digests** — hash files or standard input
- **Verify checksums** — check files against a previously generated checksum file
- **BSD-style tags** — output checksums in BSD tag format
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Compute MD5 hash of files
md5sum file1.txt file2.txt

# Check files against saved checksums
md5sum -c checksums.md5

# Output in BSD tag format
md5sum --tag file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-b, --binary` | Read files in binary mode |
| `-c, --check` | Verify checksums read from the input files |
| `--tag` | Output BSD-style checksums |
| `-q, --quiet` | Suppress OK messages when checking |
| `--status` | Exit with non-zero status on mismatch, produce no output |
| `-w, --warn` | Warn about improperly formatted checksum lines |

## Building

```sh
cargo build --package md5sum --release
```
