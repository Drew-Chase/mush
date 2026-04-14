//! Mush - Multi-Unified Shell
//!
//! A cross-platform shell interpreter with a TUI interface, 78 bundled Unix utilities,
//! and smart autocomplete. Built with [Ratatui](https://ratatui.rs) for live-streaming
//! terminal output.

mod config;
mod db;
mod shell;
mod widgets;

use clap::Parser;

use crate::config::Config;
use crate::db::HistoryDb;
use crate::widgets::App;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mush", version, about = "A cross-platform shell interpreter")]
struct MushArgs {
    /// Open config file in default editor
    #[arg(long)]
    config: bool,

    /// Print install directory
    #[arg(long)]
    install_dir: bool,

    /// Open GitHub repository in browser
    #[arg(long)]
    github: bool,

    /// Check for available updates
    #[arg(long)]
    check_updates: bool,

    /// Install available updates
    #[arg(long)]
    install_updates: bool,

    /// Execute a command string and exit
    #[arg(short = 'c', long = "command", conflicts_with = "file")]
    command: Option<String>,

    /// Script file to execute
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = MushArgs::parse();

    if args.config {
        let config_path = get_appdata_path().join("config.toml");
        // Ensure config file exists before opening
        std::fs::create_dir_all(get_appdata_path())?;
        Config::load_or_default(config_path.clone())?;
        Config::get().save()?;
        open::that(&config_path)?;
        return Ok(());
    }
    if args.install_dir {
        let dir = std::env::current_exe()?
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_default();
        println!("{}", dir.display());
        return Ok(());
    }
    if args.github {
        open::that("https://github.com/drew-chase/mush")?;
        return Ok(());
    }
    if args.check_updates {
        println!("Update checking is not yet implemented.");
        return Ok(());
    }
    if args.install_updates {
        println!("Update installation is not yet implemented.");
        return Ok(());
    }

    std::fs::create_dir_all(get_appdata_path())?;
    std::fs::create_dir_all(get_appdata_path().join("scripts"))?;
    shell::script_registry::scan_scripts(&get_appdata_path().join("scripts"));
    Config::load_or_default(get_appdata_path().join("config.toml"))?;
    Config::get().save()?;

    // Initialize history database for all modes (prevents panic if `history` is called in scripts)
    let db_path = Config::get().db_path();
    let _ = HistoryDb::init_global(&db_path);

    // Non-interactive: -c "command"
    if let Some(cmd) = &args.command {
        source_env_file();
        let exit_code = run_command_string(cmd);
        std::process::exit(exit_code);
    }

    // Non-interactive: mush script.mush
    if let Some(file) = &args.file {
        source_env_file();
        let content = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("mush: {}: {e}", file.display());
                std::process::exit(1);
            }
        };
        let exit_code = run_script_content(&content);
        std::process::exit(exit_code);
    }

    // Interactive mode: source startup files, then launch TUI
    source_init_file();
    ratatui::run(|terminal| -> color_eyre::Result<()> {
        let mut instance = App::new()?;
        instance.run(terminal)?;
        Ok(())
    })?;
    Ok(())
}

/// Source `~/.config/mush/init.mush` or `~/.mushrc` for interactive sessions.
fn source_init_file() {
    let init_path = get_appdata_path().join("init.mush");
    if init_path.exists()
        && let Ok(content) = std::fs::read_to_string(&init_path)
    {
        run_script_content(&content);
        return;
    }
    // Fallback: ~/.mushrc
    if let Some(home) = shell::builtins::home_dir() {
        let mushrc = home.join(".mushrc");
        if mushrc.exists()
            && let Ok(content) = std::fs::read_to_string(&mushrc)
        {
            run_script_content(&content);
        }
    }
}

/// Source `~/.config/mush/env.mush` for non-interactive (script) sessions.
fn source_env_file() {
    let env_path = get_appdata_path().join("env.mush");
    if env_path.exists()
        && let Ok(content) = std::fs::read_to_string(&env_path)
    {
        run_script_content(&content);
    }
}

/// Execute a single command string (for `mush -c "..."`) and return the exit code.
fn run_command_string(cmd: &str) -> i32 {
    let trimmed = cmd.trim();
    if trimmed.is_empty() {
        return 0;
    }
    match shell::parser::parse(trimmed) {
        Ok(cl) => {
            let mut env = shell::expand::ShellEnv {
                last_exit_code: 0,
                temp_files: Vec::new(),
            };
            match shell::expand::expand(&cl, &mut env) {
                Ok(expanded) => {
                    let mut exit_code = 0;
                    for chain in &expanded.chains {
                        let result = shell::pipeline::execute_chain_sync(chain);
                        for line in &result.output {
                            println!("{line}");
                        }
                        exit_code = result.exit_code;
                        if result.exit_app {
                            break;
                        }
                    }
                    for path in &env.temp_files {
                        let _ = std::fs::remove_file(path);
                    }
                    exit_code
                }
                Err(e) => {
                    eprintln!("mush: expansion error: {e}");
                    1
                }
            }
        }
        Err(e) => {
            eprintln!("mush: parse error: {e}");
            1
        }
    }
}

/// Execute multi-line script content and return the exit code.
fn run_script_content(content: &str) -> i32 {
    // Join line continuations (lines ending with \)
    let mut logical_lines: Vec<String> = Vec::new();
    let mut current_line = String::new();
    for line in content.lines() {
        let trimmed = line.trim_end();
        if let Some(stripped) = trimmed.strip_suffix('\\') {
            current_line.push_str(stripped);
            current_line.push(' ');
        } else {
            current_line.push_str(trimmed);
            if !current_line.is_empty() {
                logical_lines.push(std::mem::take(&mut current_line));
            }
        }
    }
    if !current_line.is_empty() {
        logical_lines.push(current_line);
    }

    let mut exit_code = 0;

    for line in &logical_lines {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        match shell::parser::parse(trimmed) {
            Ok(cl) => {
                let mut env = shell::expand::ShellEnv {
                    last_exit_code: exit_code,
                    temp_files: Vec::new(),
                };
                match shell::expand::expand(&cl, &mut env) {
                    Ok(expanded) => {
                        for chain in &expanded.chains {
                            let result = shell::pipeline::execute_chain_sync(chain);
                            for line in &result.output {
                                println!("{line}");
                            }
                            exit_code = result.exit_code;
                            if result.exit_app {
                                for path in &env.temp_files {
                                    let _ = std::fs::remove_file(path);
                                }
                                return exit_code;
                            }
                        }
                        for path in &env.temp_files {
                            let _ = std::fs::remove_file(path);
                        }
                    }
                    Err(e) => {
                        eprintln!("mush: expansion error: {e}");
                        exit_code = 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("mush: parse error: {e}");
                exit_code = 1;
            }
        }
    }

    exit_code
}

pub fn get_appdata_path() -> PathBuf {
    if let Ok(config_path) = std::env::var("MUSH_APPDATA") {
        PathBuf::from(config_path)
    } else if let Some(local_appdata) = dirs::config_local_dir() {
        local_appdata.join("mush")
    } else if let Some(config) = dirs::config_dir() {
        config.join("mush")
    } else {
        std::env::temp_dir().join("mush")
    }
}
