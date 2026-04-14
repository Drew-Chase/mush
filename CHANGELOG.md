# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
