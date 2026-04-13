# diff

A cross-platform `diff` implementation in Rust, built as part of the [mush](../../README.md) shell.

## Features

- **Unified and context output** — standard unified (`-u`) and context (`-c`) diff formats
- **Side-by-side view** — two-column comparison with configurable width
- **Whitespace control** — ignore case, spacing changes, blank lines, or all whitespace
- **Recursive directory diffs** — compare entire directory trees with `-r`
- **Colorized output** — syntax-highlighted diffs with `--color`
- **GitHub-style format** — line-numbered diff output with `--github`
- **Cross-platform** — Windows, macOS, and Linux

## Usage

```sh
# unified diff between two files
diff -u file1.txt file2.txt

# side-by-side comparison
diff -y -W 160 old.txt new.txt

# recursive directory diff with color
diff -r --color src/ src-backup/

# brief report of differences only
diff -q dir1/ dir2/
```

## Flags

| Flag | Description |
|------|-------------|
| `-u, --unified[=NUM]` | Output NUM (default 3) lines of unified context |
| `-c, --context[=NUM]` | Output NUM (default 3) lines of copied context |
| `-y, --side-by-side` | Output in two columns |
| `-W, --width NUM` | Output at most NUM (default 130) print columns |
| `-i, --ignore-case` | Ignore case differences in file contents |
| `-b, --ignore-space-change` | Ignore changes in the amount of white space |
| `-w, --ignore-all-space` | Ignore all white space |
| `-B, --ignore-blank-lines` | Ignore changes where lines are all blank |
| `-r, --recursive` | Recursively compare any subdirectories found |
| `-q, --brief` | Report only when files differ |
| `-s, --report-identical-files` | Report when two files are identical |
| `--color` | Colorize the output |
| `--github` | Output in GitHub-style diff format with line numbers |

## Building

```sh
cargo build --package diff --release
```
