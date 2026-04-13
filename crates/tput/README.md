# tput

A cross-platform `tput` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Terminal queries** — retrieve terminal dimensions, color support, and capabilities
- **Cursor control** — position the cursor at arbitrary locations
- **Text formatting** — enable bold, reset attributes, and set foreground colors
- **Screen control** — clear the terminal screen
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Get the number of columns
tput cols

# Get the number of lines
tput lines

# Get the number of supported colors
tput colors

# Enable bold text
tput bold

# Reset text attributes
tput sgr0

# Set foreground color (e.g., red = 1)
tput setaf 1

# Clear the screen
tput clear

# Move cursor to row 5, column 10
tput cup 5 10
```

## Capabilities

| Capability | Description |
|------------|-------------|
| `cols` | Print the number of terminal columns |
| `lines` | Print the number of terminal lines |
| `colors` | Print the number of supported colors |
| `bold` | Enable bold text mode |
| `sgr0` | Reset all text attributes |
| `setaf <N>` | Set foreground color to color N |
| `clear` | Clear the terminal screen |
| `cup <ROW> <COL>` | Move cursor to the specified position |

## Building

```sh
cargo build --package tput --release
```
