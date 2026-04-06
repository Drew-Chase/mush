# cp

A cross-platform `cp` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **GNU-compatible** — follows GNU coreutils cp behavior
- **Cross-platform** — Windows, macOS, and Linux
- **Recursive copying** — `-r` copies directories and contents
- **Interactive prompts** — `-i` prompts before overwriting
- **No-clobber mode** — `-n` never overwrites existing files
- **Update mode** — `-u` only copies when source is newer
- **Target directory** — `-t DIR` copies all sources into a directory
- **Verbose output** — `-v` prints each operation
- **Zero runtime dependencies** — no external crates
- **Library crate** — use programmatically via the `cp` library API

## Usage

```sh
# Copy a file
cp file.txt copy.txt

# Copy a directory recursively
cp -r src_dir/ dest_dir/

# Copy multiple files into a directory
cp a.txt b.txt c.txt dest_dir/

# Copy with explicit target directory
cp -t dest_dir/ a.txt b.txt

# Interactive mode (prompt before overwriting)
cp -i file.txt existing.txt

# No-clobber (never overwrite)
cp -n file.txt existing.txt

# Update (only if source is newer)
cp -u file.txt dest.txt

# Verbose output
cp -v file.txt copy.txt

# Combined flags
cp -rfv src_dir/ dest_dir/
```

## Flags

| Flag | Description |
|------|-------------|
| `-f, --force` | Do not prompt before overwriting (default) |
| `-i, --interactive` | Prompt before overwriting |
| `-n, --no-clobber` | Do not overwrite existing files |
| `-r, -R, --recursive` | Copy directories recursively |
| `-u, --update` | Copy only when source is newer or dest missing |
| `-v, --verbose` | Explain what is being done |
| `-t, --target-directory=DIR` | Copy all sources into DIR |
| `-T, --no-target-directory` | Treat DEST as a normal file |
| `--help` | Print help information |
| `--version` | Print version information |

If you specify more than one of `-i`, `-f`, `-n`, only the final one takes effect.

## Building

```sh
cargo build --package cp --release
```

## Library Usage

```rust
use std::io;
use std::path::Path;
use cp::cli::CpConfig;
use cp::ops::copy_path;

let config = CpConfig {
    recursive: true,
    verbose: true,
    ..Default::default()
};

let stdin = io::stdin();
let mut reader = stdin.lock();
let stderr = io::stderr();
let mut writer = stderr.lock();

copy_path(
    Path::new("source_dir"),
    Path::new("dest_dir"),
    &config,
    &mut reader,
    &mut writer,
)?;
```

See `examples/basic.rs` for more.

## Architecture

```
CpConfig::from_args()     Manual flag parsing (no clap)
    -> resolve_targets()  Determine sources and target directory
    -> copy_path()        fs::copy / recursive copy_dir_recursive
    -> stderr/stdout      Error messages / verbose output / prompts
```

| Module | Purpose |
|--------|---------|
| `cli` | Manual argument parsing with GNU cp semantics and last-wins override |
| `ops` | Cross-platform file/directory copy with interactive prompts and recursive support |
