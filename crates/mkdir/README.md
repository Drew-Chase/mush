# mkdir

A cross-platform `mkdir` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **GNU-compatible** — follows GNU coreutils mkdir behavior
- **Cross-platform** — Windows, macOS, and Linux
- **Parent directory creation** — `-p` creates intermediate directories
- **File mode support** — `-m` sets Unix permissions (accepted but ignored on Windows)
- **Zero runtime dependencies** — no external crates
- **Library crate** — use programmatically via the `mkdir` library API

## Usage

```sh
# Create a single directory
mkdir new_dir

# Create nested directories (parents)
mkdir -p path/to/deep/dir

# Create with verbose output
mkdir -pv a/b/c

# Set Unix permissions (octal)
mkdir -m 755 my_dir

# Create multiple directories
mkdir dir1 dir2 dir3

# Combined flags
mkdir -pm 700 secret/nested/dir
```

## Flags

| Flag | Description |
|------|-------------|
| `-p, --parents` | No error if existing, make parent directories as needed |
| `-v, --verbose` | Print a message for each created directory |
| `-m, --mode=MODE` | Set file mode (octal, as in chmod) |
| `--help` | Print help information |
| `--version` | Print version information |

Flags can be combined: `-pv` enables both parents and verbose. `-pm 755` enables parents and sets mode.

## Building

```sh
cargo build --package mkdir --release
```

## Library Usage

```rust
use std::path::Path;
use mkdir::cli::MkdirConfig;
use mkdir::ops::create_directory;

let config = MkdirConfig {
    parents: true,
    verbose: true,
    ..Default::default()
};

create_directory(Path::new("my/nested/dir"), &config)?;
```

See `examples/basic.rs` for more.

## Architecture

```
MkdirConfig::from_args()   Manual flag parsing (no clap)
    -> create_directory()  fs::create_dir / create_dir_all + mode setting
    -> stderr/stdout       Error messages / verbose output
```

| Module | Purpose |
|--------|---------|
| `cli` | Manual argument parsing with GNU mkdir semantics |
| `ops` | Cross-platform directory creation with mode support |
