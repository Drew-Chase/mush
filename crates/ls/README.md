# ls

A cross-platform `ls` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **30+ flags** compatible with GNU `ls` conventions
- **Cross-platform** — Windows, macOS, and Linux with platform-appropriate metadata handling
- **Multiple output formats** — long (`-l`), grid/columns (`-C`), single-column (`-1`), comma-separated (`-m`)
- **Colored output** — ANSI colors by file type with `--color=auto|always|never`
- **Sorting** — by name, size, time, or extension with reverse and group-directories-first
- **Recursive listing** — `-R` with symlink loop detection
- **Classify indicators** — `-F` appends `*/=>@|` by file type
- **Human-readable sizes** — `-h` shows K/M/G/T suffixes
- **Library crate** — use programmatically via the `ls` library API

## Usage

```sh
# Default grid listing
ls

# Long format, all files, human-readable sizes
ls -lah

# Sort by size, largest first, directories grouped
ls -lS --group-directories-first

# Recursive with color
ls -R --color=always

# Comma-separated format
ls -m

# Single column, classify indicators
ls -1F
```

## Flags

| Flag | Description |
|------|-------------|
| `-a, --all` | Show hidden entries (starting with `.`) |
| `-A, --almost-all` | Show hidden entries except `.` and `..` |
| `-B, --ignore-backups` | Skip entries ending with `~` |
| `-b, --escape` | Print C-style escapes for nongraphic characters |
| `-C` | List entries by columns |
| `-c` | Sort by and show change time |
| `--color[=WHEN]` | Colorize output: `always`, `auto`, `never` |
| `-d, --directory` | List directories themselves, not contents |
| `-F, --classify` | Append indicator (`*/=>@\|`) to entries |
| `--file-type` | Like `--classify` but without `*` |
| `-f` | Do not sort, show all, disable color |
| `--format=WORD` | Output format: `long`, `commas`, `single-column`, etc. |
| `--full-time` | Long format with full ISO timestamps |
| `-g` | Long format without owner |
| `--group-directories-first` | Group directories before files |
| `-h, --human-readable` | Print sizes in K/M/G/T |
| `-i, --inode` | Print index number of each file |
| `-l` | Long listing format |
| `-m` | Comma-separated list |
| `-n, --numeric-uid-gid` | Numeric user and group IDs |
| `-o` | Long format without group |
| `-p` | Append `/` to directories |
| `-R, --recursive` | List subdirectories recursively |
| `-r, --reverse` | Reverse sort order |
| `-S` | Sort by file size |
| `-s, --size` | Print allocated size |
| `--sort=WORD` | Sort by: `none`, `size`, `time`, `extension` |
| `-t` | Sort by modification time |
| `--time=WORD` | Timestamp: `atime`, `ctime`, `mtime` |
| `--time-style=STYLE` | Time format: `full-iso`, `long-iso`, `iso` |
| `-U` | Do not sort |
| `-w, --width=COLS` | Set output width |
| `-X` | Sort by extension |
| `-1` | One entry per line |

## Building

```sh
cargo build --package ls --release
```

## Library Usage

The `ls` crate exposes its internals as a library for programmatic use:

```rust
use ls::cli::ResolvedConfig;
use ls::color::ColorScheme;
use ls::{format, read, sort};

let config = ResolvedConfig::default();
let colors = ColorScheme::new(&config);

let mut entries = read::read_entries(".".as_ref(), &config)?;
sort::sort_entries(&mut entries, &config);
format::write_output(&entries, &config, &colors, &mut std::io::stdout())?;
```

See `examples/basic.rs` and `examples/custom_format.rs` for more.

## Architecture

```
Cli::parse() → ResolvedConfig
    → read::read_entries()    Read directory + filter hidden/backups
    → sort::sort_entries()    Sort by key + reverse + group-dirs-first
    → format::write_output()  Dispatch to Long/Grid/Single/Comma formatter
    → BufWriter<Stdout>       Buffered output
```

| Module | Purpose |
|--------|---------|
| `cli` | Argument parsing (clap) and flag conflict resolution |
| `entry` | `FileEntry` struct with per-file metadata |
| `read` | Directory reading and entry filtering |
| `sort` | Sort strategies |
| `color` | ANSI color by file type |
| `format` | Output formatters (long, grid, single, comma) |
| `platform` | Platform-specific metadata (Unix/Windows) |
