# patch

A cross-platform `patch` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Apply unified diffs** — apply standard unified diff patches to files
- **Strip path components** — remove leading path components with `-p` for flexible patching
- **Reverse patches** — unapply a patch with `-R`
- **Dry-run mode** — verify a patch applies cleanly without modifying files
- **Backup files** — create `.orig` backup files before patching
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# apply a patch from a file
patch -i fix.patch

# apply a patch from stdin
patch < fix.patch

# strip one leading path component
patch -p1 -i fix.patch

# reverse a previously applied patch
patch -R -i fix.patch

# dry run to check if patch applies cleanly
patch --dry-run -i fix.patch

# create backup files before patching
patch -b -i fix.patch
```

## Flags

| Flag | Description |
|------|-------------|
| `-p NUM` | Strip NUM leading path components from file names |
| `-R` | Reverse the patch |
| `--dry-run` | Do not actually modify any files |
| `-b` | Create backup files (`.orig`) |
| `-i FILE` | Read patch from FILE instead of stdin |

## Building

```sh
cargo build --package patch --release
```
