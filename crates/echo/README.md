# echo

A cross-platform `echo` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **GNU-compatible** — follows GNU coreutils echo behavior
- **Cross-platform** — Windows, macOS, and Linux
- **Escape sequences** — full support with `-e` flag: `\n`, `\t`, `\0NNN`, `\xHH`, `\c`, and more
- **Zero dependencies** — no external crates at runtime
- **Library crate** — use programmatically via the `echo` library API

## Usage

```sh
# Simple output
echo hello world

# Suppress trailing newline
echo -n "no newline"

# Enable escape sequences
echo -e "col1\tcol2\tcol3"
echo -e "line1\nline2"

# Stop output mid-string
echo -e "stop here\cignored"

# Hex and octal
echo -e "\x48\x65\x6c\x6c\x6f"
echo -e "\0110\0145\0154\0154\0157"
```

## Flags

| Flag | Description |
|------|-------------|
| `-n` | Do not output the trailing newline |
| `-e` | Enable interpretation of backslash escapes |
| `-E` | Disable interpretation of backslash escapes (default) |
| `--help` | Print help information |
| `--version` | Print version information |

Flags can be combined: `-ne` enables both no-newline and escape interpretation.

Only `-n`, `-e`, `-E` and combinations thereof are recognized as flags. Any other `-` prefixed argument is treated as a literal string, matching GNU echo behavior.

## Escape Sequences (with `-e`)

| Sequence | Description |
|----------|-------------|
| `\\` | Backslash |
| `\a` | Alert (BEL, 0x07) |
| `\b` | Backspace (0x08) |
| `\c` | Stop output immediately (no newline, no further text) |
| `\e` | Escape character (0x1B) |
| `\f` | Form feed (0x0C) |
| `\n` | Newline (0x0A) |
| `\r` | Carriage return (0x0D) |
| `\t` | Horizontal tab (0x09) |
| `\v` | Vertical tab (0x0B) |
| `\0NNN` | Byte with octal value NNN (0 to 3 digits) |
| `\xHH` | Byte with hexadecimal value HH (1 to 2 digits) |

## Building

```sh
cargo build --package echo --release
```

## Library Usage

```rust
use echo::cli::EchoConfig;
use echo::escape::process_escapes;

let args: Vec<String> = vec!["-e".into(), "hello\\tworld".into()];
if let Some(config) = EchoConfig::from_args(&args) {
    for arg in &config.args {
        if config.interpret_escapes {
            let (text, stop) = process_escapes(arg);
            print!("{text}");
            if stop { break; }
        } else {
            print!("{arg}");
        }
    }
}
```

See `examples/basic.rs` and `examples/escapes.rs` for more.

## Architecture

```
EchoConfig::from_args()   Manual flag parsing (no clap)
    -> process_escapes()  Optional escape interpretation per arg
    -> stdout             Direct output
```

| Module | Purpose |
|--------|---------|
| `cli` | Manual argument parsing with GNU echo semantics |
| `escape` | Backslash escape sequence processing |
