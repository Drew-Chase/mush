# sha256sum

A cross-platform `sha256sum` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Multiple algorithms** — supports SHA-224, SHA-256, SHA-384, and SHA-512
- **Compute checksums** — hash files or standard input
- **Verify checksums** — check files against a previously generated checksum file
- **BSD-style tags** — output checksums in BSD tag format
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Compute SHA-256 hash of files
sha256sum file1.txt file2.txt

# Use a different SHA-2 algorithm
sha256sum -a 512 file.bin

# Check files against saved checksums
sha256sum -c checksums.sha256
```

## Flags

| Flag | Description |
|------|-------------|
| `-a, --algorithm` | Select the hash algorithm (224, 256, 384, 512) |
| `-b, --binary` | Read files in binary mode |
| `-c, --check` | Verify checksums read from the input files |
| `--tag` | Output BSD-style checksums |
| `-q, --quiet` | Suppress OK messages when checking |
| `--status` | Exit with non-zero status on mismatch, produce no output |
| `-w, --warn` | Warn about improperly formatted checksum lines |

## Building

```sh
cargo build --package sha256sum --release
```
