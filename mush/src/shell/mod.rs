pub mod builtins;
pub mod path_resolver;

use std::path::{Path, PathBuf};

use crate::config::Config;

#[derive(Debug)]
pub enum CommandKind {
    Builtin(builtins::BuiltinCommand),
    External(PathBuf),
    Alias(Vec<String>),
    NotFound,
}

/// Resolves the first token of `input` to a command kind.
pub fn resolve_command(input: &str) -> CommandKind {
    let name = match input.split_whitespace().next() {
        Some(name) => name,
        None => return CommandKind::NotFound,
    };

    // Check aliases first (alias name must be the entire first token with no args
    // to trigger alias expansion, OR the full input matches)
    let config = Config::get();
    if let Some(commands) = config.alias.get_commands(name) {
        // If the user typed just the alias name (no extra args), expand it
        let args_after: &str = input[name.len()..].trim();
        if args_after.is_empty() {
            return CommandKind::Alias(commands);
        }
    }

    if let Some(builtin) = builtins::lookup(name) {
        return CommandKind::Builtin(builtin);
    }

    if let Some(path) = path_resolver::find_in_path(name) {
        return CommandKind::External(path);
    }

    CommandKind::NotFound
}

/// Describes an available command for autocomplete purposes.
pub struct CommandInfo {
    pub name: String,
    pub description: Option<String>,
}

/// Returns all available command names with descriptions.
pub fn all_commands() -> Vec<CommandInfo> {
    let mut commands: Vec<CommandInfo> = Vec::new();

    // Aliases
    let config = Config::get();
    for (name, raw) in &config.alias.entries {
        commands.push(CommandInfo {
            name: name.clone(),
            description: Some(crate::config::alias::format_description(raw)),
        });
    }

    // Builtins
    commands.push(CommandInfo { name: "cd".to_string(), description: Some("(builtin)".to_string()) });
    commands.push(CommandInfo { name: "exit".to_string(), description: Some("(builtin)".to_string()) });

    // PATH executables
    for name in path_resolver::list_executables() {
        commands.push(CommandInfo {
            name: name.clone(),
            description: None,
        });
    }

    commands
}

/// Quick check for syntax highlighting: is the first token a valid command?
/// Returns `true` for empty input (no red on empty buffer).
pub fn is_valid_command(input: &str) -> bool {
    let name = match input.split_whitespace().next() {
        Some(name) => name,
        None => return true,
    };

    let config = Config::get();
    if config.alias.has(name) {
        return true;
    }

    if builtins::lookup(name).is_some() {
        return true;
    }

    path_resolver::is_executable(name)
}

pub fn is_interactive(cmd_name: &str) -> bool {
    let stem = Path::new(cmd_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(cmd_name)
        .to_lowercase();

    const INTERACTIVE: &[&str] = &[
        "powershell", "pwsh", "cmd", "bash", "zsh", "fish", "sh", "nu", "mush",
        "vim", "nvim", "vi", "nano", "helix", "emacs",
        "python", "python3", "node", "irb", "lua",
        "top", "htop", "btop", "less", "more", "man",
        "ssh", "ftp", "claude",
    ];

    if INTERACTIVE.contains(&stem.as_str()) {
        return true;
    }

    let config = Config::get();
    config
        .application
        .interactive_commands
        .iter()
        .any(|c| c.to_lowercase() == stem)
}
