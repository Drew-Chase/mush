# uname

A cross-platform `uname` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **System identification** — print kernel name, version, architecture, and more
- **Selective output** — display individual fields or all at once
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print the kernel name
uname

# Print all system information
uname -a

# Print the machine architecture
uname -m

# Print kernel name and release
uname -sr
```

## Flags

| Flag | Description |
|------|-------------|
| `-a, --all` | Print all available system information |
| `-s, --kernel-name` | Print the kernel name |
| `-n, --nodename` | Print the network node hostname |
| `-r, --kernel-release` | Print the kernel release |
| `-v, --kernel-version` | Print the kernel version |
| `-m, --machine` | Print the machine hardware name |
| `-p, --processor` | Print the processor type |
| `-o, --operating-system` | Print the operating system |

## Building

```sh
cargo build --package uname --release
```
