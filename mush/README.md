# mush (shell crate)

The core shell interpreter for [mush](../README.md) — a cross-platform TUI shell written in Rust.

This crate contains the parser, variable expander, pipeline executor, builtin commands, autocomplete engine, and the Ratatui-based terminal UI.

## Architecture

```
Input → Lexer → Parser → AST → Expander → Pipeline Executor → TUI Output
```

### Modules

| Module | Purpose |
|--------|---------|
| `shell/parser.rs` | Two-phase lexer + parser producing an AST |
| `shell/ast.rs` | AST node definitions (CommandLine, Chain, Pipeline, SimpleCommand) |
| `shell/expand.rs` | Variable, glob, command substitution, and process substitution expansion |
| `shell/pipeline.rs` | Synchronous and streaming pipeline execution with I/O redirection |
| `shell/builtins.rs` | 30 builtin commands (cd, export, alias, test, printf, etc.) |
| `shell/mod.rs` | Command resolution: alias → builtin → script → PATH lookup |
| `shell/path_resolver.rs` | PATH search with cross-platform executable detection |
| `shell/help_parser.rs` | Parses `--help` output to extract flags for autocomplete |
| `shell/script_registry.rs` | TypeScript/Bun script discovery and management |
| `config/` | TOML configuration with aliases, layout, and startup settings |
| `db/` | SQLite-backed command history and help cache |
| `widgets/` | Ratatui TUI: command input, history display, autocomplete, history popover |

### Parser Hierarchy

```
CommandLine
  └─ Chain (connected by && or ||, optional & for background)
       └─ Pipeline (connected by |)
            └─ SimpleCommand (words + redirects)
                 ├─ Word → Literal | Variable | CommandSubstitution | Glob | ...
                 └─ Redirect → >, >>, <, 2>, 2>>, 2>&1, <<<, <<
```

### Command Resolution Order

1. Aliases (from config)
2. Builtin commands
3. User scripts (TypeScript via Bun)
4. Direct paths (`./prog`, `/usr/bin/prog`)
5. PATH executables

## Dependencies

| Crate | Purpose |
|-------|---------|
| `ratatui` | TUI framework |
| `sqlx` | SQLite database (history, help cache) |
| `tokio` | Async runtime for database operations |
| `serde` + `toml` | Configuration serialization |
| `glob` | Filesystem glob pattern matching |
| `arboard` | Clipboard access |
| `dirs` | Platform-specific config/data directories |
| `notify` | Filesystem watching (config reload, script discovery) |
| `clap` | CLI argument parsing |
| `color-eyre` | Error reporting |

## Building

```sh
cargo build --package mush --release
```

The binary is output to `target/release/mush`.
