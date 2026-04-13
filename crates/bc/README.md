# bc

A cross-platform `bc` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Arbitrary precision arithmetic** — supports integers and floating-point numbers with configurable scale
- **Standard math library** — optional `-l` flag provides math functions (sine, cosine, arctangent, logarithm, exponential, bessel)
- **Interactive REPL** — reads expressions from stdin with immediate evaluation
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Interactive mode
bc

# With standard math library
bc -l

# Evaluate an expression from a pipe
echo "2 ^ 10" | bc

# Compute with decimal precision
echo "scale=4; 22/7" | bc -l
```

## Flags

| Flag | Description |
|------|-------------|
| `-l` | Define the standard math library and set default scale to 20 |

## Building

```sh
cargo build --package bc --release
```
