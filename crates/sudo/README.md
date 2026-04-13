# sudo

A cross-platform `sudo` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Elevated execution** — run commands with another user's privileges
- **User selection** — execute as any specified user
- **Login shell** — start a login shell as the target user
- **Shell mode** — run the target user's shell with a command
- **Environment preservation** — optionally keep the current environment
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Run a command as root
sudo ls /root

# Run a command as a specific user
sudo -u www-data whoami

# Start a login shell as root
sudo -i

# Run root's shell with a command
sudo -s whoami

# Preserve environment variables
sudo -E env
```

## Flags

| Flag | Description |
|------|-------------|
| `-u, --user <USER>` | Run the command as the specified user (default: root) |
| `-i, --login` | Start a login shell as the target user |
| `-s, --shell` | Run the target user's shell |
| `-E, --preserve-env` | Preserve the current environment when running the command |

## Building

```sh
cargo build --package sudo --release
```
