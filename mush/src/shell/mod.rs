pub mod builtins;
pub mod path_resolver;

use std::path::PathBuf;

#[derive(Debug)]
pub enum CommandKind {
    Builtin(builtins::BuiltinCommand),
    External(PathBuf),
    NotFound,
}

/// Resolves the first token of `input` to a command kind.
pub fn resolve_command(input: &str) -> CommandKind {
    let name = match input.split_whitespace().next() {
        Some(name) => name,
        None => return CommandKind::NotFound,
    };

    if let Some(builtin) = builtins::lookup(name) {
        return CommandKind::Builtin(builtin);
    }

    if let Some(path) = path_resolver::find_in_path(name) {
        return CommandKind::External(path);
    }

    CommandKind::NotFound
}

/// Returns all available command names (builtins + PATH executables).
pub fn all_command_names() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = vec!["cd", "exit"];
    for name in path_resolver::list_executables() {
        names.push(name.as_str());
    }
    names
}

/// Quick check for syntax highlighting: is the first token a valid command?
/// Returns `true` for empty input (no red on empty buffer).
pub fn is_valid_command(input: &str) -> bool {
    let name = match input.split_whitespace().next() {
        Some(name) => name,
        None => return true,
    };

    if builtins::lookup(name).is_some() {
        return true;
    }

    path_resolver::is_executable(name)
}
