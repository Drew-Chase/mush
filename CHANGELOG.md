# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-04-13

### Added

- Script file execution: `mush script.mush` runs a script file and exits with its exit code
- Command string execution: `mush -c "echo hello"` executes a command string and exits
- Startup file sourcing: `~/.config/mush/init.mush` (or `~/.mushrc`) on interactive startup
- Non-interactive startup file: `~/.config/mush/env.mush` sourced in script mode
- Shebang support: `#!/usr/bin/env mush` works in script files (comment lines are skipped)
- `help` builtin: list all builtins or show detailed help for a specific builtin
- `echo` builtin: display text with `-n` (no newline) and `-e` (escape sequences) support
- `kill` builtin: send signals to processes by PID with `-9`, `-TERM`, `-s SIGNAL` syntax
- CI workflow: `cargo fmt`, `cargo clippy`, `cargo test` on Windows, macOS, and Linux
- Comprehensive documentation: README, wiki (97 pages), CONTRIBUTING, CHANGELOG, CODE_OF_CONDUCT, SECURITY
- GitHub issue/PR templates and CI documentation checks
- Workspace Cargo.toml metadata inheritance across all 78 utility crates
- Crate-level doc comments on all crates and doc comments on shell core public APIs
- Exit code visual feedback: failed commands show red border and exit code in TUI
- Stderr coloring: stderr output shown in red in streaming mode
- `2>&1` redirect properly merges stderr into stdout pipe in multi-command pipelines
- Builtin-in-pipeline streaming: builtins pipe output to next command instead of sync fallback

### Fixed

- `exit` builtin now respects the exit code argument (`exit 1` previously always exited with 0)
- Broken rustdoc links in `timeout` and `chown` crates

[0.2.0]: https://github.com/Drew-Chase/mush/releases/tag/v0.2.0

## [0.1.0] - 2026-04-11

### Added

- TUI-first interactive shell powered by Ratatui with live-streaming output, ANSI rendering, and mouse support
- Two-phase parser (lexer + AST) supporting pipes, chains (`&&`, `||`, `;`), background jobs (`&`), and subshells
- Variable expansion: `$VAR`, `${VAR:-default}`, `${VAR:=value}`, `${VAR:+alt}`, `${VAR:?err}`, `$?`, `$(cmd)`, `<(cmd)`
- I/O redirection: `>`, `>>`, `<`, `2>`, `2>>`, `2>&1`, `<<<`, `<<`
- Glob pattern expansion: `*`, `?`, `[abc]`
- 78 bundled cross-platform Unix utilities (ls, grep, find, sed, awk, cat, cp, mv, rm, diff, sort, tree, ps, and more)
- 30 builtin commands: cd, pushd/popd, export, alias, source, test, printf, read, jobs, fg, bg, and more
- Smart autocomplete that parses `--help` output from any command and caches suggestions in SQLite
- SQLite-backed persistent command history with search and deduplication
- TOML configuration file support (`~/.config/mush/config.toml`)
- TypeScript/JavaScript scripting extension via Bun
- Clipboard support (Ctrl+C/X/V) and mouse text selection
- GitHub-style diff view with dual line numbers and color coding
- Help menu for builtin commands
- CLI arguments for config, install-dir, github, version, and updates
- Windows NSIS installer
- Cross-platform release builds (Windows x64, macOS ARM64/x64, Linux x64)
- GitHub Actions release workflow for tag-triggered builds

[0.1.0]: https://github.com/Drew-Chase/mush/releases/tag/0.1.0
