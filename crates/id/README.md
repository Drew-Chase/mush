# id

A cross-platform `id` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **User and group info** — print UID, GID, and supplementary groups
- **Selective output** — show only user ID, group ID, or group list
- **Name display** — print names instead of numeric IDs
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# Print all user and group information
id

# Print only the effective user ID
id -u

# Print the user name instead of the numeric ID
id -un

# Print all supplementary groups
id -G
```

## Flags

| Flag | Description |
|------|-------------|
| `-u, --user` | Print only the effective user ID |
| `-g, --group` | Print only the effective group ID |
| `-G, --groups` | Print all group IDs |
| `-n, --name` | Print names instead of numeric IDs (use with -u, -g, or -G) |
| `-r, --real` | Print real IDs instead of effective IDs (use with -u, -g, or -G) |

## Building

```sh
cargo build --package id --release
```
