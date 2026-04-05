use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum BuiltinCommand {
    Cd,
    Exit,
}

pub struct BuiltinResult {
    pub output: Vec<String>,
    pub exit_app: bool,
    pub change_dir: Option<PathBuf>,
}

pub fn lookup(name: &str) -> Option<BuiltinCommand> {
    match name.to_lowercase().as_str() {
        "cd" => Some(BuiltinCommand::Cd),
        "exit" => Some(BuiltinCommand::Exit),
        _ => None,
    }
}

pub fn execute(cmd: BuiltinCommand, args: &[&str]) -> BuiltinResult {
    match cmd {
        BuiltinCommand::Cd => execute_cd(args),
        BuiltinCommand::Exit => BuiltinResult {
            output: Vec::new(),
            exit_app: true,
            change_dir: None,
        },
    }
}

fn execute_cd(args: &[&str]) -> BuiltinResult {
    let target = if args.is_empty() {
        // No args: go to home directory
        match home_dir() {
            Some(home) => home,
            None => {
                return BuiltinResult {
                    output: vec!["cd: could not determine home directory".to_string()],
                    exit_app: false,
                    change_dir: None,
                };
            }
        }
    } else {
        let path_str = args[0];
        // Handle ~ expansion
        if path_str == "~" {
            match home_dir() {
                Some(home) => home,
                None => {
                    return BuiltinResult {
                        output: vec!["cd: could not determine home directory".to_string()],
                        exit_app: false,
                        change_dir: None,
                    };
                }
            }
        } else if let Some(rest) = path_str.strip_prefix("~/").or_else(|| path_str.strip_prefix("~\\")) {
            match home_dir() {
                Some(home) => home.join(rest),
                None => {
                    return BuiltinResult {
                        output: vec!["cd: could not determine home directory".to_string()],
                        exit_app: false,
                        change_dir: None,
                    };
                }
            }
        } else {
            PathBuf::from(path_str)
        }
    };

    match std::env::set_current_dir(&target) {
        Ok(()) => BuiltinResult {
            output: Vec::new(),
            exit_app: false,
            change_dir: Some(
                std::env::current_dir().unwrap_or(target),
            ),
        },
        Err(e) => BuiltinResult {
            output: vec![format!("cd: {}: {}", target.display(), e)],
            exit_app: false,
            change_dir: None,
        },
    }
}

fn home_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        std::env::var("USERPROFILE").ok().map(PathBuf::from)
    }
    #[cfg(not(windows))]
    {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}
