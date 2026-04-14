# Contributing to Mush

Thank you for your interest in contributing to mush! This guide covers everything you need to get started.

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)
- [just](https://github.com/casey/just) task runner (optional, for convenience commands)
- Git

## Building

```sh
git clone https://github.com/Drew-Chase/mush.git
cd mush
cargo build --workspace
```

For a release build:

```sh
just build
# or: cargo build --workspace --release
```

The main shell binary is `mush`, and each bundled utility is a separate binary. All outputs go to `target/debug/` or `target/release/`.

## Running Tests

```sh
cargo test --workspace
```

## Code Style

- **Formatting:** `cargo fmt` (default rustfmt settings)
- **Linting:** `cargo clippy -- -D warnings` — all warnings are treated as errors
- **No `unsafe`** without a comment explaining why it's necessary
- **Error handling:** use `color-eyre` / `Result` in the main crate, `std::process::exit` in utilities

## Project Structure

```
mush/
├── mush/          # Main shell interpreter
│   └── src/
│       ├── config/    # TOML configuration, aliases, layout
│       ├── db/        # SQLite history database
│       ├── shell/     # Parser, AST, builtins, pipeline executor, expansion
│       └── widgets/   # TUI components (Ratatui)
├── crates/        # 78 bundled utility crates (ls, grep, find, etc.)
├── .wiki/         # Documentation wiki (git submodule)
├── nsis/          # Windows installer scripts
├── Cargo.toml     # Workspace configuration
└── ROADMAP.md     # Development roadmap
```

See the [ROADMAP](ROADMAP.md) for architecture details and the path to 1.0.0.

## Making Changes

### Pull Request Guidelines

1. **One logical change per PR** — don't mix features with bug fixes or refactoring
2. **Target the `master` branch**
3. **Include tests** for new features or bug fixes when applicable
4. **Update documentation** — if your change affects behavior, update the relevant `.wiki/` page and/or crate README
5. **Run checks before submitting:**
   ```sh
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test --workspace
   ```

### Commit Messages

Use descriptive commit messages that explain *what* and *why*:

```
feat(ls): add --tree flag for recursive tree output
fix(parser): handle unclosed quotes in variable expansion
docs(wiki): add sed backreference documentation
refactor(pipeline): extract redirect handling into separate module
```

### Adding a New Utility

1. Create the crate directory: `crates/myutil/`
2. Create `crates/myutil/Cargo.toml`:
   ```toml
   [package]
   name = "myutil"
   description = "Cross-platform myutil implementation for the mush shell"
   version.workspace = true
   edition.workspace = true
   authors.workspace = true
   license.workspace = true
   repository.workspace = true

   [dependencies]
   clap.workspace = true
   ```
3. Create `crates/myutil/src/main.rs` with a clap-derive CLI
4. Add `"crates/myutil"` to the `members` array in the root `Cargo.toml`
5. Create `crates/myutil/README.md` with usage examples and flag documentation
6. Create a wiki page at `.wiki/myutil.md`
7. Add the utility to the appropriate category table in `.wiki/` and `README.md`

## Reporting Bugs

Open an issue on [GitHub](https://github.com/Drew-Chase/mush/issues) with:

- Operating system and version
- Mush version (`mush --version`)
- Terminal emulator
- Steps to reproduce
- Expected vs. actual behavior

## Questions?

If you're unsure about an approach or want to discuss a feature before implementing it, open an issue to start a conversation.
