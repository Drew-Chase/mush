# cat

A cross-platform `cat` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **File concatenation** — concatenate and print one or more files to standard output
- **Line numbering** — number all lines or only non-blank lines
- **Whitespace visualization** — display tabs as `^I`, line endings as `$`, and non-printing characters with `^` and `M-` notation
- **Blank line squeezing** — suppress repeated empty output lines
- **Stdin support** — read from standard input when no file is given or when `-` is specified
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print a file
cat file.txt

# Concatenate multiple files with line numbers
cat -n file1.txt file2.txt

# Show all non-printing characters, tabs, and line endings
cat -A file.txt

# Squeeze repeated blank lines and number non-blank lines
cat -sb file.txt
```

## Flags

| Flag | Description |
|------|-------------|
| `-A, --show-all` | Equivalent to `-vET` |
| `-b, --number-nonblank` | Number nonempty output lines, overrides `-n` |
| `-e` | Equivalent to `-vE` |
| `-E, --show-ends` | Display `$` at end of each line |
| `-n, --number` | Number all output lines |
| `-s, --squeeze-blank` | Suppress repeated empty output lines |
| `-t` | Equivalent to `-vT` |
| `-T, --show-tabs` | Display TAB characters as `^I` |
| `-v, --show-nonprinting` | Use `^` and `M-` notation, except for LFD and TAB |

## Building

```sh
cargo build --package cat --release
```
