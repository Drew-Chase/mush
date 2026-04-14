# Mush Shell: Product Review & Roadmap to 1.0.0

**Date:** 2026-04-12
**Current Version:** 0.1.0 (117 commits, 1 tag)
**Codebase:** ~25,000 lines of Rust across 79 crates

---

## Part 1: Product Review

### What Mush Is Today

Mush is a cross-platform interactive shell with a Ratatui TUI, 78 bundled Unix utilities, a proper two-phase parser (lexer + AST), variable expansion, command pipelines, background jobs, SQLite-backed history, and a smart autocomplete system that parses `--help` output.

### What Mush Is Not (Yet)

Mush is not a scripting language. It cannot run shell scripts, define functions, or use control flow (if/for/while). It is an interactive command runner with good UX, not a bash replacement.

---

### Current Capabilities Assessment

| Area | Status | Score |
|------|--------|-------|
| **Interactive command execution** | Exit code feedback, stderr coloring, `2>&1` merge, streaming builtins | 10/10 |
| **Parsing & expansion** | Good (pipes, chains, globs, $VAR, $(cmd), <(cmd)) | 8/10 |
| **Autocomplete** | Excellent (commands, help options, pipe output) | 9/10 |
| **History** | Good (SQLite, search, dedup) | 8/10 |
| **TUI** | Good (live output, ANSI support, mouse, clipboard) | 8/10 |
| **Bundled utilities** | 78 crates, core utils solid | 7/10 |
| **Control flow** | None | 0/10 |
| **Scripting** | Script files, `-c` flag, startup files, shebang | 5/10 |
| **Job control** | Basic (bg/fg stubs, no Ctrl+Z) | 3/10 |
| **Shell options** | Stored but not enforced | 2/10 |
| **Documentation** | README, wiki, CHANGELOG, CONTRIBUTING, doc comments, Cargo metadata | 9/10 |
| **CI/CD** | Release workflow + PR checks (fmt, clippy, test, doc) on 3 platforms | 8/10 |
| **Packaging** | Windows NSIS only | 3/10 |
| **Config & customization** | Basic TOML, no themes, no prompt | 3/10 |
| **Cross-platform** | Architecture is good, gaps in practice | 6/10 |

**Overall: 4.5/10 for production use, 8/10 as an interactive prototype.**

---

### Critical Gaps (Blockers for 1.0)

1. **No control flow** — Cannot express `if`, `for`, `while`, `case`, or define functions. This is the single largest gap. Without it, mush cannot run real shell scripts or be a bash replacement.

2. **No script file execution** — `mush script.sh` doesn't work. There's no CLI argument to pass a script file. The `source` builtin is the only way to execute files, and it's line-by-line.

3. **No job control suspend/resume** — Ctrl+Z doesn't suspend processes. The `fg`/`bg` builtins are wired up for background jobs but there's no SIGTSTP/SIGCONT signal handling.

4. **Shell options not enforced** — `set -e` (errexit), `set -u` (nounset), `set -x` (xtrace) are stored but never checked during execution. Scripts relying on `set -e` will silently continue past failures.

5. **No startup file** — No `.mushrc` or equivalent is sourced on startup. Users can't set aliases, env vars, or PATH modifications that persist across sessions.

6. **No prompt customization** — The prompt is hardcoded. No PS1/PS2 equivalent.

7. **Documentation is effectively zero** — 2-line README, no man pages, no user guide, no CHANGELOG, no CONTRIBUTING guide.

8. **No CI for PRs** — Code can be merged without any automated checks (tests, clippy, fmt).

---

### Strengths Worth Preserving

- **TUI-first design** — The live-streaming output, inline autocomplete with `--help` parsing, and mouse support are genuinely novel. Most shells are line-mode; mush's TUI approach is a differentiator.
- **Cross-platform from day one** — The architecture handles Windows/macOS/Linux differences well (PATHEXT, path separators, config dirs).
- **Bundled utilities** — Shipping 78 self-contained utilities means mush works in environments where coreutils aren't available (fresh Windows installs, containers).
- **Clean Rust codebase** — Proper use of the type system, no unsafe outside of env var mutations, good module separation.
- **Parser quality** — The two-phase lexer-parser with a proper AST is the right foundation. Adding control flow means extending the AST, not rewriting it.

---

## Part 2: Roadmap to 1.0.0

### Version Strategy

| Version | Codename | Theme | Target |
|---------|----------|-------|--------|
| **0.2.0** | Foundation | CI, docs, startup files, script execution | Infrastructure |
| **0.3.0** | Control | if/else, for, while, functions | Scripting |
| **0.4.0** | Polish | Job control, prompt, themes, shell options | Usability |
| **0.5.0** | Completeness | Utility hardening, missing flags, edge cases | Reliability |
| **0.6.0-0.9.0** | Stabilization | Bug fixes, performance, packaging, community | Production |
| **1.0.0** | Release | Stable API, documented, packaged | General availability |

---

### 0.2.0 — Foundation

*Goal: Make the project credible. CI, docs, and basic scripting.*

#### CI/CD Pipeline
- [x] Add `.github/workflows/ci.yml` — run on every push and PR:
  - `cargo fmt --check`
  - `cargo clippy -- -D warnings`
  - `cargo test` (all workspace members)
  - Matrix: Windows, macOS, Linux
- [ ] Add branch protection rules on `master` requiring CI pass

#### Documentation
- [x] Rewrite `README.md`:
  - Project description and goals
  - Installation instructions (per platform)
  - Feature overview with screenshots
  - Configuration reference
  - Known limitations
  - Contributing section
- [x] Add `CHANGELOG.md` (retroactive for 0.1.0)
- [x] Add `CONTRIBUTING.md` (build instructions, PR guidelines, code style)
- [x] Add built-in `help` command listing all builtins with descriptions

#### Startup Files
- [x] Source `~/.config/mush/init.mush` (or `~/.mushrc`) on startup if it exists
- [x] Source `~/.config/mush/env.mush` for non-interactive (script) mode
- [ ] Document the startup file order

#### Script File Execution
- [x] Add `[file]` positional argument to CLI: `mush script.mush`
- [x] Add `-c "command"` flag: `mush -c "echo hello"`
- [x] Detect and skip shebang lines (`#!/usr/bin/env mush`)
- [x] Non-interactive mode: no TUI, execute and exit
- [x] Exit code propagation from scripts

#### Quick Wins
- [x] Implement `echo` as a builtin (currently falls through to PATH)
- [x] Implement `kill` as a builtin (currently falls through to the crate)
- [ ] Wire up `read` builtin to stdin in interactive mode (crossterm raw input)

---

### 0.3.0 — Control Flow

*Goal: Make mush a real scripting language. This is the hardest milestone.*

#### AST Extensions
- [ ] Add `If { condition, then_body, elseif_chains, else_body }` node
- [ ] Add `For { var, iterable, body }` node (for-in)
- [ ] Add `While { condition, body }` node
- [ ] Add `Until { condition, body }` node
- [ ] Add `Case { word, patterns: Vec<(Pattern, Body)> }` node
- [ ] Add `FunctionDef { name, body }` node
- [ ] Add `Return { exit_code }` node
- [ ] Add `Break` and `Continue` nodes

#### Parser Extensions
- [ ] Recognize keywords: `if`, `then`, `elif`, `else`, `fi`
- [ ] Recognize keywords: `for`, `in`, `do`, `done`
- [ ] Recognize keywords: `while`, `until`
- [ ] Recognize keywords: `case`, `esac`, `;;`
- [ ] Recognize keywords: `function`, `{`, `}`
- [ ] Handle multi-line input in interactive mode (detect incomplete input, show continuation prompt)
- [ ] Keyword protection: prevent keywords from being used as command names

#### Execution
- [ ] Implement `if` condition evaluation (run pipeline, check exit code)
- [ ] Implement `for var in ...` iteration (word list, glob expansion, command substitution)
- [ ] Implement `while`/`until` loop execution with `break`/`continue`
- [ ] Implement `case` pattern matching (glob patterns, `|` alternation)
- [ ] Implement function definition storage and invocation
- [ ] Function-local variables (or at least `local` keyword)
- [ ] Arithmetic expansion `$(( expr ))` for loop counters

#### Testing
- [ ] Parser tests for each control flow construct
- [ ] Execution tests: nested if/for/while, break/continue, function recursion
- [ ] Script file tests: create test `.mush` scripts and verify output
- [ ] Edge cases: empty bodies, single-line forms, deeply nested structures

---

### 0.4.0 — Polish

*Goal: Make mush pleasant to use daily.*

#### Job Control
- [ ] Implement Ctrl+Z (SIGTSTP) to suspend foreground process
- [ ] Implement SIGCONT delivery for `fg`/`bg`
- [ ] Track job state: Running, Stopped, Done
- [ ] Display job state changes in TUI
- [ ] Unix-only: use process groups for proper job control (`setpgid`, `tcsetpgrp`)

#### Prompt Customization
- [ ] Add `prompt` config option in `config.toml`
- [ ] Support escape sequences: `\u` (user), `\h` (host), `\w` (cwd), `\W` (basename), `\$` (# for root), `\t` (time), `\n` (newline)
- [ ] Support color codes in prompt
- [ ] Support command substitution in prompt (e.g., git branch)
- [ ] PS2-equivalent for continuation lines

#### Theme System
- [ ] Define theme schema (JSON or TOML): colors for prompt, errors, commands, output, autocomplete, history
- [ ] Load themes from `~/.config/mush/themes/`
- [ ] Ship 3-5 built-in themes (dark, light, solarized, monokai, nord)
- [ ] Apply theme to all TUI widgets

#### Shell Options Enforcement
- [ ] `set -e` (errexit): abort script on non-zero exit code (skip in interactive mode)
- [ ] `set -u` (nounset): error on expansion of unset variables
- [ ] `set -x` (xtrace): print commands before execution (prefix with `+`)
- [ ] `set -o pipefail`: pipeline exit code is rightmost non-zero

#### Multi-line Input
- [ ] Detect incomplete input (unclosed quotes, trailing `\`, open control structures)
- [ ] Show continuation prompt (PS2) and accumulate lines
- [ ] Submit on complete input, not just Enter

#### Keybindings
- [ ] Make keybindings configurable via `config.toml`
- [ ] Document all default keybindings
- [ ] Support Emacs-style (default) and Vi-mode input

---

### 0.5.0 — Completeness

*Goal: Harden utilities and fill feature gaps.*

#### Utility Hardening
- [ ] `cp`: Add `-p` (preserve timestamps/permissions), `-a` (archive)
- [ ] `mv`: Handle cross-filesystem moves (copy + delete)
- [ ] `grep`: Binary file detection, `-a` flag
- [ ] `sed`: Address groups `{}`, backreferences `\1`, write to file `w`
- [ ] `awk`: Complete the program evaluator (currently basic/stub)
- [ ] `find`: Add `-user`, `-group`, `-uid`, `-gid`, `-depth`
- [ ] All utilities: Consistent `--version` output

#### Missing Utilities (Priority)
- [ ] `tar` — archive creation/extraction (use `tar` crate or implement)
- [ ] `gzip`/`gunzip` — compression (use `flate2` crate)
- [ ] `which` — standalone (currently a builtin alias for `type`)
- [ ] `dd` — block copy
- [ ] `split` — file splitting
- [ ] `nl` — line numbering
- [ ] `od` — octal/hex dump (complement `xxd`)

#### Test Coverage
- [ ] Integration test suite: shell scripts that test end-to-end behavior
- [ ] Each utility: at least 10 test cases covering flags, edge cases, errors
- [ ] Fuzzing: fuzz the parser with `cargo-fuzz`
- [ ] Benchmark suite: measure startup time, pipeline throughput, large-file handling

#### Error Messages
- [ ] Consistent error format across all builtins: `mush: <builtin>: <message>`
- [ ] Helpful suggestions on common mistakes (e.g., "did you mean...?")
- [ ] `RUST_BACKTRACE` equivalent for debugging: `MUSH_DEBUG=1`

---

### 0.6.0-0.9.0 — Stabilization

*Goal: Production hardening.*

#### Packaging (0.6.0)
- [ ] Homebrew formula for macOS/Linux
- [ ] Chocolatey package for Windows
- [ ] `.deb` package for Debian/Ubuntu
- [ ] `.rpm` package for Fedora/RHEL
- [ ] Docker image: `ghcr.io/drew-chase/mush`
- [ ] AUR package for Arch Linux

#### Performance (0.7.0)
- [ ] Benchmark against bash/zsh for common operations
- [ ] Profile startup time — target < 50ms
- [ ] Profile pipeline throughput — target parity with bash
- [ ] Lazy-load help cache (don't load all on startup)
- [ ] Reduce memory footprint of bundled utilities (shared binary with subcommand dispatch?)

#### Update Mechanism (0.7.0)
- [ ] Implement `--check-updates`: check GitHub releases API
- [ ] Implement `--install-updates`: download and replace binaries
- [ ] Signature verification for downloaded binaries
- [ ] Auto-update prompt on startup (configurable)

#### Security (0.8.0)
- [ ] Add `SECURITY.md` with vulnerability reporting process
- [ ] Audit all `unsafe` blocks (currently ~15, all env var mutations)
- [ ] Consider sandboxing for user scripts (seccomp-bpf on Linux, App Sandbox on macOS)
- [ ] Validate PATH entries for obvious injection attempts
- [ ] Rate-limit command substitution depth (already done: 64)

#### Accessibility (0.8.0)
- [ ] Non-TUI batch mode: `mush --no-tui` for piped/scripted use
- [ ] High-contrast theme
- [ ] Configurable color scheme for colorblind users
- [ ] Keyboard-only navigation documentation

#### Compatibility (0.9.0)
- [ ] POSIX sh compliance mode: `mush --posix` or `set -o posix`
- [ ] Bash compatibility shims for common bashisms
- [ ] Test with common shell scripts (dotfiles, build scripts, CI scripts)
- [ ] `.bashrc`/`.zshrc` migration guide

---

### 1.0.0 — General Availability

*Goal: Stable, documented, packaged, trustworthy.*

#### Release Criteria
- [ ] All Critical and High review findings fixed (done as of 0.1.0+)
- [ ] Control flow complete: if/for/while/case/functions with tests
- [ ] Script execution: `mush script.mush` and shebang support
- [ ] Job control: Ctrl+Z suspend/resume on Unix
- [ ] Shell options enforced: set -e, -u, -x, -o pipefail
- [ ] Startup file: `.mushrc` sourced on launch
- [ ] Prompt customization working
- [ ] CI passing on all 3 platforms
- [ ] Documentation: README, user guide, man page, CHANGELOG
- [ ] Packaged for: Homebrew, Chocolatey, .deb, .rpm
- [ ] Zero known Critical/High bugs
- [ ] At least 200 integration tests
- [ ] Startup time < 100ms on modern hardware
- [ ] Semantic versioning commitment: 1.x.y API stability

---

## Part 3: Architecture Recommendations

### Parser Evolution Strategy

The current parser handles `CommandLine -> Chain -> Pipeline -> SimpleCommand`. Control flow requires extending this hierarchy:

```
CommandLine
  Chain
    Pipeline
      SimpleCommand        (existing)
      IfStatement          (new)
      ForStatement         (new)
      WhileStatement       (new)
      CaseStatement        (new)
      FunctionDef          (new)
      Block { commands }   (new — replaces subshell for function bodies)
```

**Recommendation:** Add a `Statement` enum that wraps both `SimpleCommand` and control flow nodes. The pipeline stage would contain `Vec<Statement>` instead of `Vec<SimpleCommand>`. This is the minimal disruption to the existing architecture.

### Script Execution Strategy

Non-interactive mode should:
1. Parse the entire file into a `CommandLine` (not line-by-line)
2. Expand and execute the full AST
3. Respect `set -e` and other options
4. Support `exit N` to terminate early

The current `source` builtin processes line-by-line. For 0.3.0, the parser must handle multi-line constructs, which means the `source` builtin should feed the entire file to the parser as a single unit.

### Multi-line Interactive Input

When the parser returns `ParseError::UnexpectedEof` after an `if` or `for` keyword, the TUI should:
1. Show a continuation prompt (e.g., `> `)
2. Accumulate input lines
3. Re-parse the accumulated input on each Enter
4. Execute when parsing succeeds

This requires the TUI to distinguish between "syntax error" and "incomplete input" — add a `ParseError::Incomplete` variant.

### Bundled Utilities Strategy

Currently each utility is a separate binary. For 1.0, consider:
- **Keep separate binaries** for correctness and isolation
- **Optionally** build a BusyBox-style multi-call binary for minimal deployments
- **Share** common code via a `mush-common` crate (argument parsing patterns, error formatting, file I/O helpers)

---

## Part 4: Effort Estimates

| Milestone | Estimated Effort | Key Risk |
|-----------|-----------------|----------|
| 0.2.0 (Foundation) | 40-60 hours | Scope creep on docs |
| 0.3.0 (Control Flow) | 120-180 hours | Parser complexity, multi-line input |
| 0.4.0 (Polish) | 60-80 hours | Job control (Unix signal handling) |
| 0.5.0 (Completeness) | 80-120 hours | awk implementation depth |
| 0.6.0-0.9.0 (Stabilization) | 100-150 hours | Packaging for each platform |
| 1.0.0 (Release) | 20-40 hours | Final testing, docs review |
| **Total** | **420-630 hours** | |

**Calendar estimate:** 6-12 months with a single developer working part-time, or 3-4 months with focused full-time effort.

---

## Part 5: What NOT to Do

1. **Don't chase POSIX compliance first.** Mush's value proposition is being *better* than bash, not identical. POSIX mode can come later as a compatibility layer.

2. **Don't rewrite the parser.** The current lexer+parser+AST architecture is sound. Extend it, don't replace it.

3. **Don't add a plugin system yet.** The TypeScript/Bun script system is sufficient for extensibility. A proper plugin API is a post-1.0 concern.

4. **Don't ship a language server.** IDE integration is nice-to-have but irrelevant for 1.0.

5. **Don't implement every GNU coreutils flag.** Target the 80% use case. If someone needs `find -xdev -cnewer -samefile`, they can install GNU findutils.

6. **Don't break the TUI.** The TUI is the differentiator. Every feature should work *within* the TUI, not require escaping from it.
