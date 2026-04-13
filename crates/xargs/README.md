# xargs

A cross-platform `xargs` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Build commands from stdin** — read arguments from standard input and execute commands
- **Custom delimiters** — split input on null bytes or any custom delimiter
- **Argument batching** — control how many arguments are passed per command invocation
- **Placeholder replacement** — insert arguments at specific positions in the command
- **Parallel execution** — run multiple commands concurrently
- **Interactive mode** — prompt before executing each command
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Remove all .tmp files
find . -name "*.tmp" | xargs rm

# Use null-delimited input (handles filenames with spaces)
find . -name "*.log" -print0 | xargs -0 rm

# Run at most 2 arguments per command
echo "a b c d" | xargs -n 2 echo

# Replace placeholder in command
ls *.txt | xargs -I {} cp {} backup/

# Run 4 commands in parallel
cat urls.txt | xargs -P 4 -n 1 curl -O

# Prompt before each execution
echo "file1 file2" | xargs -p rm
```

## Flags

| Flag | Description |
|------|-------------|
| `-0, --null` | Input items are terminated by null character instead of whitespace |
| `-d, --delimiter <CHAR>` | Use CHAR as input delimiter |
| `-n, --max-args <N>` | Use at most N arguments per command line |
| `-I, --replace <REPLACE>` | Replace occurrences of REPLACE in the command with input |
| `-L, --max-lines <N>` | Use at most N input lines per command line |
| `-P, --max-procs <N>` | Run up to N processes concurrently |
| `-t, --verbose` | Print the command line to stderr before executing |
| `-p, --interactive` | Prompt the user before executing each command |
| `-r, --no-run-if-empty` | Do not run the command if stdin is empty |
| `-s, --max-chars <N>` | Limit the command line length to N characters |

## Building

```sh
cargo build --package xargs --release
```
