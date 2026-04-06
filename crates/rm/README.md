# rm

A cross-platform `rm` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **GNU-compatible** — follows GNU coreutils rm behavior
- **Cross-platform** — Windows, macOS, and Linux
- **Recursive removal** — `-r` removes directories and contents
- **Interactive prompts** — `-i` (every file) and `-I` (once for bulk/recursive)
- **Force mode** — `-f` ignores nonexistent files, never prompts
- **Root protection** — `--preserve-root` prevents accidental removal of `/` (default)
- **Verbose output** — `-v` prints each removed path
- **Zero runtime dependencies** — no external crates
- **Library crate** — use programmatically via the `rm` library API

## Usage

```sh
# Remove a file
rm file.txt

# Remove multiple files
rm file1.txt file2.txt file3.txt

# Force remove (ignore nonexistent, no prompts)
rm -f missing_file.txt

# Remove directory recursively
rm -r directory/

# Remove recursively with verbose output
rm -rv directory/

# Remove empty directory
rm -d empty_dir/

# Interactive mode (prompt for each file)
rm -i file.txt

# Combined flags
rm -rfv directory/

# Remove file starting with dash
rm -- -foo
```

## Flags

| Flag | Description |
|------|-------------|
| `-f, --force` | Ignore nonexistent files, never prompt |
| `-i` | Prompt before every removal |
| `-I` | Prompt once before removing >3 files or recursive |
| `--interactive[=WHEN]` | Prompt: `never`, `once` (-I), `always` (-i) |
| `-r, -R, --recursive` | Remove directories and contents recursively |
| `-d, --dir` | Remove empty directories |
| `-v, --verbose` | Explain what is being done |
| `--no-preserve-root` | Do not treat `/` specially |
| `--preserve-root[=all]` | Do not remove `/` (default) |
| `--help` | Print help information |
| `--version` | Print version information |

Flags can be combined: `-rf` enables recursive and force. Last flag wins for conflicts: `-fi` means interactive, `-if` means force.

## Building

```sh
cargo build --package rm --release
```

## Library Usage

```rust
use std::io;
use std::path::Path;
use rm::cli::RmConfig;
use rm::ops::remove_path;

let config = RmConfig {
    recursive: true,
    force: true,
    ..Default::default()
};

let stdin = io::stdin();
let mut reader = stdin.lock();
let stderr = io::stderr();
let mut writer = stderr.lock();

remove_path(Path::new("some/dir"), &config, &mut reader, &mut writer)?;
```

See `examples/basic.rs` for more.

## Architecture

```
RmConfig::from_args()   Manual flag parsing (no clap)
    -> remove_path()    fs::remove_file / remove_dir / remove_dir_all
    -> stderr/stdout    Error messages / verbose output / prompts
```

| Module | Purpose |
|--------|---------|
| `cli` | Manual argument parsing with GNU rm semantics and flag override rules |
| `ops` | Cross-platform file/directory removal with interactive prompts and root protection |
