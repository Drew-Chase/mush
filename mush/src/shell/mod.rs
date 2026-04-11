pub mod ast;
pub mod builtins;
pub mod expand;
pub mod help_parser;
pub mod parser;
pub mod path_resolver;
pub mod pipeline;
pub mod script_registry;

use std::path::{Path, PathBuf};

use crate::config::Config;

#[derive(Debug)]
pub enum CommandKind {
    Builtin(builtins::BuiltinCommand),
    External(PathBuf),
    Alias(Vec<String>),
    Script(script_registry::ScriptEntry),
    NotFound,
}

/// Resolves the first token of `input` to a command kind.
pub fn resolve_command(input: &str) -> CommandKind {
    let tokens = tokenize(input);
    let name = match tokens.first() {
        Some(name) => name,
        None => return CommandKind::NotFound,
    };

    // Check aliases first — only expand if user typed just the alias name (no extra args)
    let config = Config::get();
    if let Some(commands) = config.alias.get_commands(name)
        && tokens.len() == 1
    {
        return CommandKind::Alias(commands);
    }

    if let Some(builtin) = builtins::lookup(name) {
        return CommandKind::Builtin(builtin);
    }

    if let Some(entry) = script_registry::find_script(name) {
        return CommandKind::Script(entry);
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
    commands.push(CommandInfo { name: "cd".to_string(), description: Some("(builtin) Change directory".to_string()) });
    commands.push(CommandInfo { name: "clear".to_string(), description: Some("(builtin) Clear screen".to_string()) });
    commands.push(CommandInfo { name: "cls".to_string(), description: Some("(builtin) Clear screen".to_string()) });
    commands.push(CommandInfo { name: "exit".to_string(), description: Some("(builtin) Exit shell".to_string()) });
    commands.push(CommandInfo { name: "scripts".to_string(), description: Some("(builtin) Manage mush scripts".to_string()) });
    commands.push(CommandInfo { name: "pwd".to_string(), description: Some("(builtin) Print working directory".to_string()) });
    commands.push(CommandInfo { name: "export".to_string(), description: Some("(builtin) Set environment variables".to_string()) });
    commands.push(CommandInfo { name: "unset".to_string(), description: Some("(builtin) Remove environment variables".to_string()) });
    commands.push(CommandInfo { name: "printf".to_string(), description: Some("(builtin) Formatted output".to_string()) });
    commands.push(CommandInfo { name: "env".to_string(), description: Some("(builtin) Print or modify environment".to_string()) });
    commands.push(CommandInfo { name: "alias".to_string(), description: Some("(builtin) Define command aliases".to_string()) });
    commands.push(CommandInfo { name: "unalias".to_string(), description: Some("(builtin) Remove command aliases".to_string()) });
    commands.push(CommandInfo { name: "type".to_string(), description: Some("(builtin) Locate a command".to_string()) });
    commands.push(CommandInfo { name: "which".to_string(), description: Some("(builtin) Locate a command".to_string()) });
    commands.push(CommandInfo { name: "history".to_string(), description: Some("(builtin) Display command history".to_string()) });
    commands.push(CommandInfo { name: "source".to_string(), description: Some("(builtin) Execute commands from file".to_string()) });
    commands.push(CommandInfo { name: "read".to_string(), description: Some("(builtin) Read input into variable".to_string()) });
    commands.push(CommandInfo { name: "test".to_string(), description: Some("(builtin) Evaluate expressions".to_string()) });
    commands.push(CommandInfo { name: "true".to_string(), description: Some("(builtin) Return success".to_string()) });
    commands.push(CommandInfo { name: "false".to_string(), description: Some("(builtin) Return failure".to_string()) });
    commands.push(CommandInfo { name: "printenv".to_string(), description: Some("(builtin) Print environment variables".to_string()) });
    commands.push(CommandInfo { name: "pushd".to_string(), description: Some("(builtin) Push directory onto stack".to_string()) });
    commands.push(CommandInfo { name: "popd".to_string(), description: Some("(builtin) Pop directory from stack".to_string()) });
    commands.push(CommandInfo { name: "set".to_string(), description: Some("(builtin) Set shell options".to_string()) });
    commands.push(CommandInfo { name: "jobs".to_string(), description: Some("(builtin) List background jobs".to_string()) });
    commands.push(CommandInfo { name: "fg".to_string(), description: Some("(builtin) Bring job to foreground".to_string()) });
    commands.push(CommandInfo { name: "bg".to_string(), description: Some("(builtin) Resume job in background".to_string()) });
    commands.push(CommandInfo { name: "dirs".to_string(), description: Some("(builtin) Display directory stack".to_string()) });
    commands.push(CommandInfo { name: "wait".to_string(), description: Some("(builtin) Wait for background jobs".to_string()) });
    commands.push(CommandInfo { name: "expr".to_string(), description: Some("(builtin) Evaluate expressions".to_string()) });
    commands.push(CommandInfo { name: "umask".to_string(), description: Some("(builtin) Set file creation mask".to_string()) });

    // Scripts
    for entry in script_registry::list_scripts() {
        commands.push(CommandInfo {
            name: entry.name.clone(),
            description: Some(format!("(script) {}", entry.description)),
        });
    }

    // PATH executables
    for name in path_resolver::list_executables() {
        commands.push(CommandInfo {
            name,
            description: None,
        });
    }

    commands
}

/// Quick check for syntax highlighting: is the first token a valid command?
/// Returns `true` for empty input (no red on empty buffer).
pub fn is_valid_command(input: &str) -> bool {
    let tokens = tokenize(input);
    let name = match tokens.first() {
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

    if script_registry::is_script(name) {
        return true;
    }

    path_resolver::is_executable(name)
}

/// Splits a shell input string into tokens, respecting double quotes, single
/// quotes, backticks, and backslash-escaped spaces.
///
/// Quotes are stripped from the output. Unclosed quotes are handled gracefully
/// by treating the remainder of the string as part of the current token.
pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_token = false;

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_token = true;
                // consume until closing double quote
                for ch in chars.by_ref() {
                    if ch == '"' {
                        break;
                    }
                    current.push(ch);
                }
            }
            '\'' => {
                in_token = true;
                for ch in chars.by_ref() {
                    if ch == '\'' {
                        break;
                    }
                    current.push(ch);
                }
            }
            '`' => {
                in_token = true;
                for ch in chars.by_ref() {
                    if ch == '`' {
                        break;
                    }
                    current.push(ch);
                }
            }
            '\\' => {
                in_token = true;
                if let Some(&next) = chars.peek() {
                    if next == ' ' {
                        current.push(' ');
                        chars.next();
                    } else {
                        // preserve backslash for non-space escapes
                        current.push('\\');
                    }
                } else {
                    current.push('\\');
                }
            }
            c if c.is_whitespace() => {
                if in_token {
                    tokens.push(std::mem::take(&mut current));
                    in_token = false;
                }
            }
            _ => {
                in_token = true;
                current.push(c);
            }
        }
    }

    if in_token {
        tokens.push(current);
    }

    tokens
}

pub fn is_interactive(cmd_name: &str, args: &[String]) -> bool {
    const NON_INTERACTIVE_FLAGS: &[&str] = &["--help", "-h", "-?", "--version", "-V"];
    if args.iter().any(|a| NON_INTERACTIVE_FLAGS.contains(&a.as_str())) {
        return false;
    }

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

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn simple_split() {
        assert_eq!(tokenize("cmd arg1 arg2"), vec!["cmd", "arg1", "arg2"]);
    }

    #[test]
    fn double_quotes() {
        assert_eq!(
            tokenize(r#"cmd "arg with spaces" arg2"#),
            vec!["cmd", "arg with spaces", "arg2"]
        );
    }

    #[test]
    fn single_quotes() {
        assert_eq!(
            tokenize("cmd 'arg with spaces'"),
            vec!["cmd", "arg with spaces"]
        );
    }

    #[test]
    fn backtick_quotes() {
        assert_eq!(
            tokenize("cmd `arg with spaces`"),
            vec!["cmd", "arg with spaces"]
        );
    }

    #[test]
    fn escaped_spaces() {
        assert_eq!(
            tokenize(r"cmd arg\ with\ spaces"),
            vec!["cmd", "arg with spaces"]
        );
    }

    #[test]
    fn mixed() {
        assert_eq!(
            tokenize(r#"cmd "a b" 'c d' e\ f"#),
            vec!["cmd", "a b", "c d", "e f"]
        );
    }

    #[test]
    fn unclosed_quote() {
        assert_eq!(tokenize(r#"cmd "unclosed"#), vec!["cmd", "unclosed"]);
    }

    #[test]
    fn empty_input() {
        assert_eq!(tokenize(""), Vec::<String>::new());
    }

    #[test]
    fn whitespace_only() {
        assert_eq!(tokenize("   "), Vec::<String>::new());
    }

    #[test]
    fn empty_quotes() {
        assert_eq!(
            tokenize(r#"cmd "" arg"#),
            vec!["cmd", "", "arg"]
        );
    }

    #[test]
    fn adjacent_quoted_and_unquoted() {
        assert_eq!(tokenize(r#"pre"mid"post"#), vec!["premidpost"]);
    }

    #[test]
    fn cd_program_files() {
        assert_eq!(
            tokenize(r#"cd "C:\Program Files\Mush""#),
            vec!["cd", r"C:\Program Files\Mush"]
        );
    }

    #[test]
    fn mkdir_escaped() {
        assert_eq!(
            tokenize(r"mkdir New\ Directory"),
            vec!["mkdir", "New Directory"]
        );
    }
}
