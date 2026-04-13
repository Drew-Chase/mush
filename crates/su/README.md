# su

A cross-platform `su` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Switch user** — change the current user identity
- **Run commands** — execute a single command as another user
- **Login shell** — start a login shell as the target user
- **Custom shell** — specify which shell to use
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Switch to root
su

# Switch to a specific user
su username

# Run a command as another user
su -c "whoami" username

# Start a login shell as another user
su -l username

# Use a specific shell
su -s /bin/zsh username
```

## Flags

| Flag | Description |
|------|-------------|
| `-c, --command <COMMAND>` | Pass a single command to the shell |
| `-l, --login` | Start a login shell for the target user |
| `-s, --shell <SHELL>` | Use SHELL instead of the target user's default shell |

## Building

```sh
cargo build --package su --release
```
