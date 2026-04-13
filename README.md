# mush - Multi-Unified Shell

A fully cross-platform shell interpreter with a modern TUI, 78 bundled Unix utilities, and smart autocomplete — built in Rust.

Mush is a drop-in replacement for PowerShell, Bash, and Zsh that works identically on Windows, macOS, and Linux. It ships its own coreutils so you get a consistent Unix-like experience everywhere, even on a fresh Windows install.

## Features

- **TUI-first design** — live-streaming command output, inline autocomplete, mouse support, and clipboard integration powered by [Ratatui](https://ratatui.rs)
- **Cross-platform** — native binaries for Windows (x64), macOS (ARM64/x64), and Linux (x64)
- **78 bundled utilities** — `ls`, `grep`, `find`, `sed`, `awk`, `cat`, `cp`, `mv`, `rm`, `diff`, `sort`, `tree`, `ps`, and many more — no system coreutils required
- **Proper parser** — two-phase lexer + AST supporting pipes, chains (`&&`, `||`), redirects, background jobs, and subshells
- **Variable expansion** — `$VAR`, `${VAR:-default}`, `${VAR:=value}`, `${VAR:+alt}`, `${VAR:?err}`, `$?`, `$(cmd)`, `<(cmd)`
- **Glob patterns** — `*`, `?`, `[abc]` with filesystem expansion
- **I/O redirection** — `>`, `>>`, `<`, `2>`, `2>>`, `2>&1`, `<<<`, `<<`
- **Smart autocomplete** — parses `--help` output from any command and caches suggestions in SQLite
- **SQLite-backed history** — persistent, searchable, deduplicated command history
- **30 builtin commands** — `cd`, `pushd`/`popd`, `export`, `alias`, `source`, `test`, `printf`, `read`, and more
- **TypeScript scripting** — extend mush with TypeScript/JavaScript scripts via Bun
- **TOML configuration** — customizable layout, aliases, and startup behavior

## Installation

### From GitHub Releases

Download the latest release for your platform from [GitHub Releases](https://github.com/Drew-Chase/mush/releases):

| Platform | Download |
|----------|----------|
| Windows (x64) | NSIS installer (`.exe`) or portable (`.zip`) |
| macOS (ARM64) | `.tar.gz` archive |
| macOS (x64) | `.tar.gz` archive |
| Linux (x64) | `.tar.gz` archive |

### Building from Source

Requires [Rust](https://rustup.rs/) (edition 2024).

```sh
git clone https://github.com/Drew-Chase/mush.git
cd mush
cargo build --release
```

Binaries are output to `target/release/`. The main shell binary is `mush`, and each bundled utility is a separate binary.

## Shell Features

### Pipelines and Chains

```sh
ls -la | grep ".rs" | sort -k5 -n
cargo build && cargo test || echo "failed"
long-running-task &
```

### Variable Expansion

```sh
echo $HOME
echo ${USER:-unknown}
echo "Files: $(ls | wc -l)"
diff <(sort file1.txt) <(sort file2.txt)
```

### I/O Redirection

```sh
echo "hello" > output.txt
cat file.txt >> log.txt
command 2>/dev/null
command > out.txt 2>&1
grep pattern <<< "search this string"
```

### Glob Patterns

```sh
ls *.rs
cp src/**/*.toml backup/
rm temp/??_*.log
```

## Builtin Commands

| Category | Commands |
|----------|----------|
| **Navigation** | `cd`, `pwd`, `pushd`, `popd`, `dirs` |
| **Environment** | `export`, `unset`, `env`, `printenv`, `set` |
| **Aliases** | `alias`, `unalias` |
| **I/O** | `echo`, `printf`, `read` |
| **History** | `history`, `source` (`.`), `scripts` |
| **Control** | `exit`, `true`, `false`, `test` (`[`) |
| **Info** | `type`, `which`, `expr`, `umask` |
| **Jobs** | `jobs`, `fg`, `bg`, `wait` |
| **Terminal** | `clear` (`cls`) |

## Bundled Utilities

All 78 utilities are standalone Rust binaries that work on every supported platform.

<details>
<summary>Full list (click to expand)</summary>

| Category | Utilities |
|----------|-----------|
| **Text Processing** | `awk`, `column`, `comm`, `cut`, `expand`, `fmt`, `fold`, `grep`, `join`, `paste`, `rev`, `sed`, `sort`, `tr`, `unexpand`, `uniq` |
| **File Viewing** | `cat`, `head`, `less`, `more`, `strings`, `tail`, `wc`, `xxd` |
| **File Comparison** | `diff`, `patch` |
| **File Operations** | `cp`, `find`, `install`, `ln`, `mkdir`, `mktemp`, `mv`, `rm`, `stat`, `file`, `tee`, `touch`, `tree` |
| **File Attributes** | `basename`, `chmod`, `chgrp`, `chown`, `dirname`, `readlink`, `realpath` |
| **Hashing & Encoding** | `base64`, `md5sum`, `sha256sum` |
| **Process Management** | `kill`, `nohup`, `pgrep`, `pkill`, `ps`, `timeout` |
| **System Info** | `arch`, `date`, `df`, `du`, `free`, `hostname`, `id`, `nproc`, `uname`, `uptime`, `whoami` |
| **Utility** | `bc`, `echo`, `ls`, `reset`, `seq`, `shuf`, `sleep`, `su`, `sudo`, `tput`, `xargs`, `xxd`, `yes` |

</details>

## Configuration

Mush stores its configuration in TOML format at:

- **Linux/macOS:** `~/.config/mush/config.toml`
- **Windows:** `%APPDATA%\mush\config.toml`
- **Override:** set `MUSH_APPDATA` environment variable

```toml
[application]
default_cwd = "~"
theme = "dark.joker"
interactive_commands = ["less", "nano", "vim"]

[alias]
ll = "ls -la"
la = "ls -A"
gs = "git status"
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Up` / `Down` | Navigate command history |
| `Tab` | Accept autocomplete suggestion |
| `Ctrl+C` | Copy selected text |
| `Ctrl+V` | Paste from clipboard |
| `Ctrl+A` | Select all |
| `Ctrl+Arrow` | Word navigation |
| `Ctrl+Backspace` | Delete word |
| `Ctrl+O` | Open current directory in file explorer |

## Project Structure

```
mush/
├── mush/          # Main shell interpreter (TUI, parser, builtins, pipeline executor)
├── crates/        # 78 bundled utility crates
│   ├── ls/
│   ├── grep/
│   ├── find/
│   └── ...
├── Cargo.toml     # Workspace configuration
└── ROADMAP.md     # Development roadmap to 1.0.0
```

## Current Status

**Version 0.1.0** — Mush is an interactive prototype. It works well as a daily-driver command runner but does not yet support control flow (`if`/`for`/`while`/`case`), function definitions, or script file execution. See the [ROADMAP](ROADMAP.md) for the path to 1.0.0.

## License

[MIT](LICENSE)
