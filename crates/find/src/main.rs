use std::fs;
use std::io::{self, Write};
use std::process::{Command, ExitCode};

use find::cli::{Action, FindConfig};
use find::ops::find;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = FindConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let paths = match find(&config) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("find: {e}");
            return ExitCode::from(1);
        }
    };

    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let mut had_error = false;

    for action in &config.actions {
        match action {
            Action::Print => {
                for path in &paths {
                    if writeln!(writer, "{}", path.display()).is_err() {
                        return ExitCode::from(1);
                    }
                }
            }
            Action::Print0 => {
                for path in &paths {
                    if write!(writer, "{}\0", path.display()).is_err() {
                        return ExitCode::from(1);
                    }
                }
            }
            Action::Delete => {
                for path in paths.iter().rev() {
                    if path.is_dir() {
                        if let Err(e) = fs::remove_dir(path) {
                            eprintln!("find: cannot delete '{}': {}", path.display(), e);
                            had_error = true;
                        }
                    } else if let Err(e) = fs::remove_file(path) {
                        eprintln!("find: cannot delete '{}': {}", path.display(), e);
                        had_error = true;
                    }
                }
            }
            Action::Exec(cmd_parts) => {
                for path in &paths {
                    let path_str = path.to_string_lossy().to_string();
                    let resolved: Vec<String> = cmd_parts
                        .iter()
                        .map(|part| {
                            if part == "{}" {
                                path_str.clone()
                            } else {
                                part.clone()
                            }
                        })
                        .collect();

                    if resolved.is_empty() {
                        continue;
                    }

                    let status = Command::new(&resolved[0])
                        .args(&resolved[1..])
                        .status();

                    match status {
                        Ok(s) if !s.success() => had_error = true,
                        Err(e) => {
                            eprintln!("find: '{}': {}", resolved[0], e);
                            had_error = true;
                        }
                        _ => {}
                    }
                }
            }
            Action::ExecPlus(cmd_parts) => {
                if paths.is_empty() {
                    continue;
                }

                let mut resolved: Vec<String> = Vec::new();
                let mut placeholder_found = false;

                for part in cmd_parts {
                    if part == "{}" {
                        placeholder_found = true;
                        for path in &paths {
                            resolved.push(path.to_string_lossy().to_string());
                        }
                    } else {
                        resolved.push(part.clone());
                    }
                }

                if !placeholder_found {
                    for path in &paths {
                        resolved.push(path.to_string_lossy().to_string());
                    }
                }

                if resolved.is_empty() {
                    continue;
                }

                let status = Command::new(&resolved[0])
                    .args(&resolved[1..])
                    .status();

                match status {
                    Ok(s) if !s.success() => had_error = true,
                    Err(e) => {
                        eprintln!("find: '{}': {}", resolved[0], e);
                        had_error = true;
                    }
                    _ => {}
                }
            }
        }
    }

    if had_error {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
