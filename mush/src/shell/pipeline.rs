use super::ast::*;
use super::{builtins, path_resolver, CommandKind};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::{Child, Command, Stdio};

/// The result of executing a single simple command synchronously.
pub struct SyncExecResult {
    pub output: Vec<String>,
    pub exit_code: i32,
    pub exit_app: bool,
    pub change_dir: bool,
}

/// Information needed to spawn a streaming (async) pipeline in the TUI.
pub struct StreamingSpawn {
    /// All child processes in the pipeline (for cleanup).
    pub children: Vec<Child>,
    /// The final child whose stdout/stderr should be streamed to the TUI.
    pub last_child: Child,
    /// Data to write to the last child's stdin (for here-string/here-doc).
    pub stdin_data: Option<Vec<u8>>,
}

/// The result of attempting to execute a pipeline.
pub enum PipelineResult {
    /// The pipeline completed synchronously.
    Sync(SyncExecResult),
    /// The pipeline spawned async processes; the last child should be streamed.
    Streaming(StreamingSpawn),
    /// The pipeline requires interactive terminal access.
    Interactive {
        path: std::path::PathBuf,
        args: Vec<String>,
    },
}

/// Resolve a SimpleCommand's first word to a CommandKind + full args list.
fn resolve_simple(cmd: &SimpleCommand) -> (CommandKind, Vec<String>) {
    let words: Vec<String> = cmd.words.iter().map(|w| w.to_plain_string()).collect();
    let name = &words[0];
    let args = if words.len() > 1 {
        words[1..].to_vec()
    } else {
        Vec::new()
    };

    let kind = super::resolve_command(name);
    (kind, args)
}

/// Build redirect configuration for a command's I/O.
struct RedirectConfig {
    stdin: StdioConfig,
    stdout: StdioConfig,
    stderr: StdioConfig,
    merge_stderr_to_stdout: bool,
}

#[allow(dead_code)]
enum StdioConfig {
    Inherit,
    Piped,
    FromFile(File),
    FromStdio(Stdio),
}

impl Default for RedirectConfig {
    fn default() -> Self {
        Self {
            stdin: StdioConfig::Inherit,
            stdout: StdioConfig::Piped,
            stderr: StdioConfig::Piped,
            merge_stderr_to_stdout: false,
        }
    }
}

fn build_redirect_config(
    redirects: &[Redirect],
    is_piped_stdin: bool,
) -> Result<RedirectConfig, String> {
    let mut config = RedirectConfig::default();

    if is_piped_stdin {
        config.stdin = StdioConfig::Piped;
    }

    for redir in redirects {
        let target_path = redir.target.to_plain_string();
        match redir.kind {
            RedirectKind::StdoutOverwrite => {
                let file = File::create(&target_path)
                    .map_err(|e| format!("redirect: {target_path}: {e}"))?;
                config.stdout = StdioConfig::FromFile(file);
            }
            RedirectKind::StdoutAppend => {
                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&target_path)
                    .map_err(|e| format!("redirect: {target_path}: {e}"))?;
                config.stdout = StdioConfig::FromFile(file);
            }
            RedirectKind::StdinRead => {
                let file = File::open(&target_path)
                    .map_err(|e| format!("redirect: {target_path}: {e}"))?;
                config.stdin = StdioConfig::FromFile(file);
            }
            RedirectKind::StderrOverwrite => {
                let file = File::create(&target_path)
                    .map_err(|e| format!("redirect: {target_path}: {e}"))?;
                config.stderr = StdioConfig::FromFile(file);
            }
            RedirectKind::StderrAppend => {
                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&target_path)
                    .map_err(|e| format!("redirect: {target_path}: {e}"))?;
                config.stderr = StdioConfig::FromFile(file);
            }
            RedirectKind::StderrToStdout => {
                config.merge_stderr_to_stdout = true;
            }
            RedirectKind::HereString | RedirectKind::HereDoc => {
                // These are handled separately during execution (need piped stdin + write)
                config.stdin = StdioConfig::Piped;
            }
        }
    }

    Ok(config)
}

/// Extract the here-string or here-doc content from redirects, if any.
fn get_stdin_data(redirects: &[Redirect]) -> Option<Vec<u8>> {
    for redir in redirects {
        match redir.kind {
            RedirectKind::HereString => {
                let mut content = redir.target.to_plain_string();
                content.push('\n');
                return Some(content.into_bytes());
            }
            RedirectKind::HereDoc => {
                // For here-doc in a single-line shell, the target is the content
                // (since we can't do multi-line input in TUI yet).
                // The delimiter-based multi-line form would need multi-line input support.
                let content = redir.target.to_plain_string();
                return Some(content.into_bytes());
            }
            _ => {}
        }
    }
    None
}

fn apply_stdio(cmd: &mut Command, config: RedirectConfig) {
    match config.stdin {
        StdioConfig::Inherit => {}
        StdioConfig::Piped => {
            cmd.stdin(Stdio::piped());
        }
        StdioConfig::FromFile(f) => {
            cmd.stdin(Stdio::from(f));
        }
        StdioConfig::FromStdio(s) => {
            cmd.stdin(s);
        }
    }

    match config.stdout {
        StdioConfig::Inherit => {}
        StdioConfig::Piped => {
            cmd.stdout(Stdio::piped());
        }
        StdioConfig::FromFile(f) => {
            cmd.stdout(Stdio::from(f));
        }
        StdioConfig::FromStdio(s) => {
            cmd.stdout(s);
        }
    }

    if config.merge_stderr_to_stdout {
        // We need stdout to be piped first to clone it for stderr.
        // If stdout is a file, we need to duplicate the file handle.
        cmd.stderr(Stdio::piped()); // will be merged in post-processing
    } else {
        match config.stderr {
            StdioConfig::Inherit => {}
            StdioConfig::Piped => {
                cmd.stderr(Stdio::piped());
            }
            StdioConfig::FromFile(f) => {
                cmd.stderr(Stdio::from(f));
            }
            StdioConfig::FromStdio(s) => {
                cmd.stderr(s);
            }
        }
    }
}

/// Collect stdout + stderr from a completed process into a SyncExecResult.
fn collect_output(out: std::process::Output) -> SyncExecResult {
    let mut lines: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(String::from)
        .collect();
    let stderr_lines: Vec<String> = String::from_utf8_lossy(&out.stderr)
        .lines()
        .map(String::from)
        .collect();
    lines.extend(stderr_lines);
    SyncExecResult {
        output: lines,
        exit_code: out.status.code().unwrap_or(-1),
        exit_app: false,
        change_dir: false,
    }
}

/// Execute a single-command pipeline synchronously (for builtins, or sync external).
pub fn execute_simple_sync(cmd: &SimpleCommand) -> SyncExecResult {
    let (kind, args) = resolve_simple(cmd);

    match kind {
        CommandKind::Builtin(builtin) => {
            let result = builtins::execute(builtin, &args);
            SyncExecResult {
                output: result.output,
                exit_code: result.exit_code,
                exit_app: result.exit_app,
                change_dir: result.change_dir.is_some(),
            }
        }
        CommandKind::External(path) => {
            let stdin_data = get_stdin_data(&cmd.redirects);
            let redir = match build_redirect_config(&cmd.redirects, false) {
                Ok(r) => r,
                Err(e) => {
                    return SyncExecResult {
                        output: vec![e],
                        exit_code: 1,
                        exit_app: false,
                        change_dir: false,
                    };
                }
            };

            let mut proc = Command::new(&path);
            super::super::widgets::force_color_env(proc.args(&args));
            apply_stdio(&mut proc, redir);

            if stdin_data.is_some() {
                // Need to spawn + write stdin instead of just .output()
                match proc.spawn() {
                    Ok(mut child) => {
                        if let Some(data) = stdin_data
                            && let Some(mut stdin) = child.stdin.take()
                        {
                            let _ = stdin.write_all(&data);
                        }
                        match child.wait_with_output() {
                            Ok(out) => collect_output(out),
                            Err(e) => SyncExecResult {
                                output: vec![format!("error: {e}")],
                                exit_code: -1,
                                exit_app: false,
                                change_dir: false,
                            },
                        }
                    }
                    Err(e) => SyncExecResult {
                        output: vec![format!("error: {e}")],
                        exit_code: -1,
                        exit_app: false,
                        change_dir: false,
                    },
                }
            } else {
                match proc.output() {
                    Ok(out) => collect_output(out),
                    Err(e) => SyncExecResult {
                        output: vec![format!("error: {e}")],
                        exit_code: -1,
                        exit_app: false,
                        change_dir: false,
                    },
                }
            }
        }
        CommandKind::Script(entry) => {
            let bun_path = match path_resolver::find_in_path("bun") {
                Some(p) => p,
                None => {
                    return SyncExecResult {
                        output: vec![
                            format!(
                                "error: script '{}' requires Bun but 'bun' was not found on PATH.",
                                entry.name
                            ),
                            "Install Bun from https://bun.sh or re-run the Mush installer with the Bun option."
                                .to_string(),
                        ],
                        exit_code: 1,
                        exit_app: false,
                        change_dir: false,
                    };
                }
            };

            let stdin_data = get_stdin_data(&cmd.redirects);
            let redir = match build_redirect_config(&cmd.redirects, false) {
                Ok(r) => r,
                Err(e) => {
                    return SyncExecResult {
                        output: vec![e],
                        exit_code: 1,
                        exit_app: false,
                        change_dir: false,
                    };
                }
            };

            let mut proc = Command::new(&bun_path);
            super::super::widgets::force_color_env(
                proc.arg("run")
                    .arg(&entry.entry_point)
                    .args(&args)
                    .current_dir(&entry.script_dir),
            );
            apply_stdio(&mut proc, redir);

            if stdin_data.is_some() {
                match proc.spawn() {
                    Ok(mut child) => {
                        if let Some(data) = stdin_data
                            && let Some(mut stdin) = child.stdin.take()
                        {
                            let _ = stdin.write_all(&data);
                        }
                        match child.wait_with_output() {
                            Ok(out) => collect_output(out),
                            Err(e) => SyncExecResult {
                                output: vec![format!("error: {e}")],
                                exit_code: -1,
                                exit_app: false,
                                change_dir: false,
                            },
                        }
                    }
                    Err(e) => SyncExecResult {
                        output: vec![format!("error: {e}")],
                        exit_code: -1,
                        exit_app: false,
                        change_dir: false,
                    },
                }
            } else {
                match proc.output() {
                    Ok(out) => collect_output(out),
                    Err(e) => SyncExecResult {
                        output: vec![format!("error: {e}")],
                        exit_code: -1,
                        exit_app: false,
                        change_dir: false,
                    },
                }
            }
        }
        CommandKind::Alias { command, extra_args } => {
            // Append any extra arguments the user typed after the alias name
            let full_cmd = if extra_args.is_empty() {
                command
            } else {
                format!("{} {}", command, extra_args.join(" "))
            };
            match super::parser::parse(&full_cmd) {
                Ok(cl) => {
                    let mut all_output = Vec::new();
                    let mut exit_app = false;
                    let mut exit_code = 0;
                    for chain in &cl.chains {
                        let result = execute_chain_sync(chain);
                        all_output.extend(result.output);
                        exit_code = result.exit_code;
                        if result.exit_app {
                            exit_app = true;
                            break;
                        }
                    }
                    SyncExecResult {
                        output: all_output,
                        exit_code,
                        exit_app,
                        change_dir: false,
                    }
                }
                Err(e) => SyncExecResult {
                    output: vec![format!("parse error: {e}")],
                    exit_code: 1,
                    exit_app: false,
                    change_dir: false,
                },
            }
        }
        CommandKind::NotFound => {
            let name = cmd.words.first().map(|w| w.to_plain_string()).unwrap_or_default();
            SyncExecResult {
                output: vec![format!("command not found: {name}")],
                exit_code: 127,
                exit_app: false,
                change_dir: false,
            }
        }
    }
}

/// Execute a chain synchronously (for use in alias expansion, etc.).
pub fn execute_chain_sync(chain: &Chain) -> SyncExecResult {
    let mut result = execute_pipeline_sync(&chain.first);

    for (op, pipeline) in &chain.rest {
        let should_run = match op {
            ChainOp::And => result.exit_code == 0,
            ChainOp::Or => result.exit_code != 0,
        };
        if should_run {
            result = execute_pipeline_sync(pipeline);
        }
    }

    result
}

/// Execute a subshell synchronously with env isolation.
fn execute_subshell_sync(inner: &super::ast::CommandLine) -> SyncExecResult {
    // Save current directory
    let saved_cwd = std::env::current_dir().ok();
    // Save current environment
    let saved_env: Vec<(String, String)> = std::env::vars().collect();

    // Execute inner command line
    let mut all_output = Vec::new();
    let mut exit_code = 0;
    let mut exit_app = false;
    for chain in &inner.chains {
        let result = execute_chain_sync(chain);
        all_output.extend(result.output);
        exit_code = result.exit_code;
        if result.exit_app {
            exit_app = true;
            break;
        }
    }

    // Restore environment: remove any new vars, restore old values
    let current_env: std::collections::HashSet<String> =
        std::env::vars().map(|(k, _)| k).collect();
    let saved_keys: std::collections::HashSet<String> =
        saved_env.iter().map(|(k, _)| k.clone()).collect();

    // Remove vars that were added in the subshell
    for key in &current_env {
        if !saved_keys.contains(key) {
            // SAFETY: mush is single-threaded for command execution
            unsafe { std::env::remove_var(key) };
        }
    }
    // Restore vars that may have been changed
    for (key, value) in &saved_env {
        if std::env::var(key).ok().as_ref() != Some(value) {
            // SAFETY: mush is single-threaded for command execution
            unsafe { std::env::set_var(key, value) };
        }
    }

    // Restore working directory
    if let Some(cwd) = saved_cwd {
        let _ = std::env::set_current_dir(cwd);
    }

    SyncExecResult {
        output: all_output,
        exit_code,
        exit_app,
        change_dir: false,
    }
}

/// Execute a pipeline synchronously.
pub fn execute_pipeline_sync(pipeline: &Pipeline) -> SyncExecResult {
    // Handle subshell pipelines
    if let Some(ref inner) = pipeline.subshell {
        return execute_subshell_sync(inner);
    }

    if pipeline.commands.len() == 1 {
        return execute_simple_sync(&pipeline.commands[0]);
    }

    // Multi-command pipeline: use OS pipes for external-to-external stages
    // to avoid buffering entire outputs in memory. Only buffer when a
    // builtin feeds into an external command.
    enum PrevOutput {
        /// Data from a builtin or alias (must be written to next stdin).
        Bytes(Vec<u8>),
        /// OS pipe from a spawned child process (zero-copy streaming).
        Pipe(Stdio),
    }

    let mut prev: Option<PrevOutput> = None;
    let mut spawned_children: Vec<Child> = Vec::new();
    let mut last_result = SyncExecResult {
        output: Vec::new(),
        exit_code: 0,
        exit_app: false,
        change_dir: false,
    };

    let err_result = |msg: String| SyncExecResult {
        output: vec![msg],
        exit_code: -1,
        exit_app: false,
        change_dir: false,
    };

    let kill_children = |children: &mut Vec<Child>| {
        for c in children.iter_mut() {
            let _ = c.kill();
            let _ = c.wait();
        }
    };

    for (i, cmd) in pipeline.commands.iter().enumerate() {
        let (kind, args) = resolve_simple(cmd);
        let is_last = i == pipeline.commands.len() - 1;

        match kind {
            CommandKind::Builtin(builtin) => {
                let stdin_bytes = match &prev {
                    Some(PrevOutput::Bytes(data)) => Some(data.as_slice()),
                    _ => None,
                };
                let result = builtins::execute_with_stdin(builtin, &args, stdin_bytes);
                prev = None; // consumed
                if is_last {
                    last_result = SyncExecResult {
                        output: result.output,
                        exit_code: result.exit_code,
                        exit_app: result.exit_app,
                        change_dir: result.change_dir.is_some(),
                    };
                } else {
                    prev = Some(PrevOutput::Bytes(result.output.join("\n").into_bytes()));
                }
            }
            CommandKind::External(path) => {
                let mut proc = Command::new(&path);
                super::super::widgets::force_color_env(proc.args(&args));

                let stdin_bytes = match prev.take() {
                    Some(PrevOutput::Pipe(pipe)) => { proc.stdin(pipe); None }
                    Some(PrevOutput::Bytes(data)) => { proc.stdin(Stdio::piped()); Some(data) }
                    None => None,
                };
                proc.stdout(Stdio::piped()).stderr(Stdio::piped());

                if let Err(e) = apply_redirects_to_command(&mut proc, &cmd.redirects) {
                    kill_children(&mut spawned_children);
                    return SyncExecResult { output: vec![e], exit_code: 1, exit_app: false, change_dir: false };
                }

                match proc.spawn() {
                    Ok(mut child) => {
                        if let Some(data) = stdin_bytes
                            && let Some(mut stdin) = child.stdin.take() {
                                let _ = stdin.write_all(&data);
                            }
                        if is_last {
                            // Wait for all earlier children, then collect final output
                            for mut c in spawned_children.drain(..) { let _ = c.wait(); }
                            match child.wait_with_output() {
                                Ok(out) => { last_result = collect_output(out); }
                                Err(e) => return err_result(format!("error: {e}")),
                            }
                        } else {
                            // Pass stdout as OS pipe to next stage
                            if let Some(stdout) = child.stdout.take() {
                                prev = Some(PrevOutput::Pipe(Stdio::from(stdout)));
                            }
                            spawned_children.push(child);
                        }
                    }
                    Err(e) => {
                        kill_children(&mut spawned_children);
                        return err_result(format!("error: {e}"));
                    }
                }
            }
            CommandKind::Script(entry) => {
                let bun_path = match path_resolver::find_in_path("bun") {
                    Some(p) => p,
                    None => {
                        kill_children(&mut spawned_children);
                        return SyncExecResult {
                            output: vec![format!("error: script '{}' requires Bun", entry.name)],
                            exit_code: 1, exit_app: false, change_dir: false,
                        };
                    }
                };

                let mut proc = Command::new(&bun_path);
                super::super::widgets::force_color_env(
                    proc.arg("run").arg(&entry.entry_point).args(&args)
                        .current_dir(&entry.script_dir),
                );

                let stdin_bytes = match prev.take() {
                    Some(PrevOutput::Pipe(pipe)) => { proc.stdin(pipe); None }
                    Some(PrevOutput::Bytes(data)) => { proc.stdin(Stdio::piped()); Some(data) }
                    None => None,
                };
                proc.stdout(Stdio::piped()).stderr(Stdio::piped());

                match proc.spawn() {
                    Ok(mut child) => {
                        if let Some(data) = stdin_bytes
                            && let Some(mut stdin) = child.stdin.take() {
                                let _ = stdin.write_all(&data);
                            }
                        if is_last {
                            for mut c in spawned_children.drain(..) { let _ = c.wait(); }
                            match child.wait_with_output() {
                                Ok(out) => { last_result = collect_output(out); }
                                Err(e) => return err_result(format!("error: {e}")),
                            }
                        } else {
                            if let Some(stdout) = child.stdout.take() {
                                prev = Some(PrevOutput::Pipe(Stdio::from(stdout)));
                            }
                            spawned_children.push(child);
                        }
                    }
                    Err(e) => {
                        kill_children(&mut spawned_children);
                        return err_result(format!("error: {e}"));
                    }
                }
            }
            CommandKind::Alias { .. } | CommandKind::NotFound => {
                let sync = execute_simple_sync(cmd);
                if is_last {
                    last_result = sync;
                } else {
                    prev = Some(PrevOutput::Bytes(sync.output.join("\n").into_bytes()));
                    last_result.exit_code = sync.exit_code;
                }
            }
        }
    }

    // Clean up any remaining children
    for mut c in spawned_children {
        let _ = c.wait();
    }

    last_result
}

/// Try to execute a pipeline, potentially spawning streaming (async) processes.
/// Returns `PipelineResult` indicating how the pipeline should be handled by the TUI.
pub fn execute_pipeline(pipeline: &Pipeline, force_interactive: bool) -> PipelineResult {
    // Subshell: always synchronous
    if let Some(ref inner) = pipeline.subshell {
        return PipelineResult::Sync(execute_subshell_sync(inner));
    }

    // Single command — check if it's a builtin, interactive, or streamable external
    if pipeline.commands.len() == 1 {
        let cmd = &pipeline.commands[0];
        let (kind, args) = resolve_simple(cmd);

        match &kind {
            CommandKind::External(path) => {
                let words: Vec<String> = cmd.words.iter().map(|w| w.to_plain_string()).collect();
                if force_interactive || super::is_interactive(&words[0], &args) {
                    return PipelineResult::Interactive {
                        path: path.clone(),
                        args,
                    };
                }

                // Spawn streaming
                let redir = match build_redirect_config(&cmd.redirects, false) {
                    Ok(r) => r,
                    Err(e) => {
                        return PipelineResult::Sync(SyncExecResult {
                            output: vec![e],
                            exit_code: 1,
                            exit_app: false,
                            change_dir: false,
                        });
                    }
                };

                let stdin_data = get_stdin_data(&cmd.redirects);
                let mut proc = Command::new(path);
                super::super::widgets::force_color_env(proc.args(&args));
                apply_stdio(&mut proc, redir);

                match proc.spawn() {
                    Ok(child) => PipelineResult::Streaming(StreamingSpawn {
                        children: Vec::new(),
                        last_child: child,
                        stdin_data,
                    }),
                    Err(e) => PipelineResult::Sync(SyncExecResult {
                        output: vec![format!("error: {e}")],
                        exit_code: -1,
                        exit_app: false,
                        change_dir: false,
                    }),
                }
            }
            CommandKind::Script(entry) => {
                let bun_path = match path_resolver::find_in_path("bun") {
                    Some(p) => p,
                    None => {
                        return PipelineResult::Sync(SyncExecResult {
                            output: vec![
                                format!(
                                    "error: script '{}' requires Bun but 'bun' was not found on PATH.",
                                    entry.name
                                ),
                                "Install Bun from https://bun.sh".to_string(),
                            ],
                            exit_code: 1,
                            exit_app: false,
                            change_dir: false,
                        });
                    }
                };

                let redir = match build_redirect_config(&cmd.redirects, false) {
                    Ok(r) => r,
                    Err(e) => {
                        return PipelineResult::Sync(SyncExecResult {
                            output: vec![e],
                            exit_code: 1,
                            exit_app: false,
                            change_dir: false,
                        });
                    }
                };

                if force_interactive {
                    let mut script_args = vec![
                        "run".to_string(),
                        entry.entry_point.to_string_lossy().to_string(),
                    ];
                    script_args.extend(args);
                    return PipelineResult::Interactive {
                        path: bun_path,
                        args: script_args,
                    };
                }

                let mut proc = Command::new(&bun_path);
                super::super::widgets::force_color_env(
                    proc.arg("run")
                        .arg(&entry.entry_point)
                        .args(&args)
                        .current_dir(&entry.script_dir),
                );
                apply_stdio(&mut proc, redir);

                let stdin_data_script = get_stdin_data(&cmd.redirects);
                match proc.spawn() {
                    Ok(child) => PipelineResult::Streaming(StreamingSpawn {
                        children: Vec::new(),
                        last_child: child,
                        stdin_data: stdin_data_script,
                    }),
                    Err(e) => PipelineResult::Sync(SyncExecResult {
                        output: vec![format!("error: {e}")],
                        exit_code: -1,
                        exit_app: false,
                        change_dir: false,
                    }),
                }
            }
            _ => {
                // Builtins, aliases, not-found — execute synchronously
                PipelineResult::Sync(execute_simple_sync(cmd))
            }
        }
    } else {
        // Multi-command pipeline: spawn all processes with piped I/O.
        // Stream the last command's output to the TUI.
        let mut children: Vec<Child> = Vec::new();
        let mut prev_stdout: Option<Stdio> = None;

        for (i, cmd) in pipeline.commands.iter().enumerate() {
            let (kind, args) = resolve_simple(cmd);
            let is_last = i == pipeline.commands.len() - 1;

            match kind {
                CommandKind::Builtin(builtin) => {
                    // Execute builtin synchronously, feed output to next command
                    let result = builtins::execute(builtin, &args);
                    if is_last {
                        return PipelineResult::Sync(SyncExecResult {
                            output: result.output,
                            exit_code: result.exit_code,
                            exit_app: result.exit_app,
                            change_dir: result.change_dir.is_some(),
                        });
                    }
                    // For non-last builtins, we need to feed their output as stdin to the next command.
                    // We'll handle this by spawning a helper that echoes the output.
                    // Simpler approach: just use the sync pipeline path for pipelines containing builtins.
                    return PipelineResult::Sync(execute_pipeline_sync(pipeline));
                }
                CommandKind::External(path) => {
                    let mut proc = Command::new(&path);
                    super::super::widgets::force_color_env(proc.args(&args));

                    if let Some(stdin) = prev_stdout.take() {
                        proc.stdin(stdin);
                    }

                    proc.stdout(Stdio::piped()).stderr(Stdio::piped());

                    // Apply file redirections (propagate errors)
                    if let Err(e) = apply_redirects_to_command(&mut proc, &cmd.redirects) {
                        for mut c in children {
                            let _ = c.kill();
                            let _ = c.wait();
                        }
                        return PipelineResult::Sync(SyncExecResult {
                            output: vec![e],
                            exit_code: 1,
                            exit_app: false,
                            change_dir: false,
                        });
                    }

                    match proc.spawn() {
                        Ok(mut child) => {
                            if !is_last {
                                if let Some(stdout) = child.stdout.take() {
                                    prev_stdout = Some(Stdio::from(stdout));
                                }
                                children.push(child);
                            } else {
                                return PipelineResult::Streaming(StreamingSpawn {
                                    children,
                                    last_child: child,
                                    stdin_data: None,
                                });
                            }
                        }
                        Err(e) => {
                            // Kill any already-spawned children
                            for mut c in children {
                                let _ = c.kill();
                                let _ = c.wait();
                            }
                            return PipelineResult::Sync(SyncExecResult {
                                output: vec![format!("error: {e}")],
                                exit_code: -1,
                                exit_app: false,
                                change_dir: false,
                            });
                        }
                    }
                }
                CommandKind::Script(entry) => {
                    let bun_path = match path_resolver::find_in_path("bun") {
                        Some(p) => p,
                        None => {
                            for mut c in children {
                                let _ = c.kill();
                                let _ = c.wait();
                            }
                            return PipelineResult::Sync(SyncExecResult {
                                output: vec![format!("error: script '{}' requires Bun", entry.name)],
                                exit_code: 1,
                                exit_app: false,
                                change_dir: false,
                            });
                        }
                    };

                    let mut proc = Command::new(&bun_path);
                    super::super::widgets::force_color_env(
                        proc.arg("run")
                            .arg(&entry.entry_point)
                            .args(&args)
                            .current_dir(&entry.script_dir),
                    );

                    if let Some(stdin) = prev_stdout.take() {
                        proc.stdin(stdin);
                    }
                    proc.stdout(Stdio::piped()).stderr(Stdio::piped());

                    match proc.spawn() {
                        Ok(mut child) => {
                            if !is_last {
                                if let Some(stdout) = child.stdout.take() {
                                    prev_stdout = Some(Stdio::from(stdout));
                                }
                                children.push(child);
                            } else {
                                return PipelineResult::Streaming(StreamingSpawn {
                                    children,
                                    last_child: child,
                                    stdin_data: None,
                                });
                            }
                        }
                        Err(e) => {
                            for mut c in children {
                                let _ = c.kill();
                                let _ = c.wait();
                            }
                            return PipelineResult::Sync(SyncExecResult {
                                output: vec![format!("error: {e}")],
                                exit_code: -1,
                                exit_app: false,
                                change_dir: false,
                            });
                        }
                    }
                }
                _ => {
                    // Alias/NotFound in a pipeline — fallback to sync
                    return PipelineResult::Sync(execute_pipeline_sync(pipeline));
                }
            }
        }

        // Should not reach here, but fallback
        PipelineResult::Sync(SyncExecResult {
            output: Vec::new(),
            exit_code: 0,
            exit_app: false,
            change_dir: false,
        })
    }
}

/// Helper: apply file redirects directly to a Command.
/// Returns an error string if any redirect file operation fails.
fn apply_redirects_to_command(cmd: &mut Command, redirects: &[Redirect]) -> Result<(), String> {
    for redir in redirects {
        let target = redir.target.to_plain_string();
        match redir.kind {
            RedirectKind::StdoutOverwrite => {
                let f = File::create(&target)
                    .map_err(|e| format!("redirect: {target}: {e}"))?;
                cmd.stdout(Stdio::from(f));
            }
            RedirectKind::StdoutAppend => {
                let f = OpenOptions::new().create(true).append(true).open(&target)
                    .map_err(|e| format!("redirect: {target}: {e}"))?;
                cmd.stdout(Stdio::from(f));
            }
            RedirectKind::StdinRead => {
                let f = File::open(&target)
                    .map_err(|e| format!("redirect: {target}: {e}"))?;
                cmd.stdin(Stdio::from(f));
            }
            RedirectKind::StderrOverwrite => {
                let f = File::create(&target)
                    .map_err(|e| format!("redirect: {target}: {e}"))?;
                cmd.stderr(Stdio::from(f));
            }
            RedirectKind::StderrAppend => {
                let f = OpenOptions::new().create(true).append(true).open(&target)
                    .map_err(|e| format!("redirect: {target}: {e}"))?;
                cmd.stderr(Stdio::from(f));
            }
            RedirectKind::StderrToStdout => {
                // This is handled at a higher level
            }
            RedirectKind::HereString | RedirectKind::HereDoc => {
                cmd.stdin(Stdio::piped());
            }
        }
    }
    Ok(())
}
