# less

A cross-platform `less` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Backward scrolling** — scroll both forward and backward through files, unlike `more`
- **Line numbers** — optionally display line numbers alongside content
- **Long line handling** — chop (truncate) long lines instead of wrapping
- **Case-insensitive search** — search through file content ignoring case
- **One-screen quit** — automatically exit if the entire file fits on one screen
- **Raw control characters** — pass through ANSI escape sequences for colored output
- **Start at line** — jump directly to a specific line number on open
- **Stdin support** — read from standard input when no file is given
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# View a file
less file.txt

# View with line numbers
less -N file.txt

# Chop long lines and show line numbers
less -SN file.txt

# Start at line 100
less -n 100 file.txt

# Quit immediately if the file fits on one screen
less -F file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-N, --line-numbers` | Show line numbers |
| `-S, --chop-long-lines` | Chop (truncate) long lines instead of wrapping |
| `-i, --ignore-case` | Ignore case in searches |
| `-F, --quit-if-one-screen` | Quit if entire file fits on one screen |
| `-R, --RAW-CONTROL-CHARS` | Output raw control characters (e.g. ANSI color) |
| `-X, --no-init` | Don't clear the screen on init/exit |
| `-n NUM` | Start displaying at line number NUM |

## Building

```sh
cargo build --package less --release
```
