# mv

A cross-platform `mv` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **GNU-compatible** — follows GNU coreutils mv behavior
- **Cross-platform** — Windows, macOS, and Linux
- **Cross-device support** — falls back to copy+delete when rename crosses filesystems
- **Interactive prompts** — `-i` prompts before overwriting
- **No-clobber mode** — `-n` never overwrites existing files
- **Update mode** — `-u` only moves when source is newer
- **Target directory** — `-t DIR` moves all sources into a directory
- **Verbose output** — `-v` prints each operation
- **Zero runtime dependencies** — no external crates
- **Library crate** — use programmatically via the `mv` library API

## Usage

```sh
# Rename a file
mv old.txt new.txt

# Move file into a directory
mv file.txt dir/

# Move multiple files into a directory
mv a.txt b.txt c.txt dest_dir/

# Move with explicit target directory
mv -t dest_dir/ a.txt b.txt

# Interactive mode (prompt before overwriting)
mv -i file.txt existing.txt

# No-clobber (never overwrite)
mv -n file.txt existing.txt

# Update (only if source is newer)
mv -u file.txt dest.txt

# Verbose output
mv -v old.txt new.txt

# Combined flags
mv -nv a.txt b.txt

# Strip trailing slashes from source
mv --strip-trailing-slashes dir/ dest/
```

## Flags

| Flag | Description |
|------|-------------|
| `-f, --force` | Do not prompt before overwriting (default) |
| `-i, --interactive` | Prompt before overwriting |
| `-n, --no-clobber` | Do not overwrite existing files |
| `-u, --update` | Move only when source is newer or dest missing |
| `-v, --verbose` | Explain what is being done |
| `-t, --target-directory=DIR` | Move all sources into DIR |
| `-T, --no-target-directory` | Treat DEST as a normal file |
| `--strip-trailing-slashes` | Remove trailing slashes from SOURCE |
| `--help` | Print help information |
| `--version` | Print version information |

If you specify more than one of `-i`, `-f`, `-n`, only the final one takes effect.

## Building

```sh
cargo build --package mv --release
```

## Library Usage

```rust
use std::io;
use std::path::Path;
use mv::cli::MvConfig;
use mv::ops::move_path;

let config = MvConfig {
    verbose: true,
    ..Default::default()
};

let stdin = io::stdin();
let mut reader = stdin.lock();
let stderr = io::stderr();
let mut writer = stderr.lock();

move_path(
    Path::new("source.txt"),
    Path::new("dest.txt"),
    &config,
    &mut reader,
    &mut writer,
)?;
```

See `examples/basic.rs` for more.

## Architecture

```
MvConfig::from_args()     Manual flag parsing (no clap)
    -> resolve_targets()  Determine sources and target directory
    -> move_path()        fs::rename with cross-device copy+delete fallback
    -> stderr/stdout      Error messages / verbose output / prompts
```

| Module | Purpose |
|--------|---------|
| `cli` | Manual argument parsing with GNU mv semantics and last-wins override |
| `ops` | Cross-platform file/directory move with interactive prompts and cross-device fallback |
