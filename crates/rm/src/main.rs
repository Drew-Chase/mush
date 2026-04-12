use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

use rm::cli::{InteractiveMode, RmConfig};
use rm::ops::remove_path;

fn main() -> ExitCode {
    let config = match RmConfig::parse().resolve() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    if config.paths.is_empty() {
        if config.force {
            return ExitCode::SUCCESS;
        }
        eprintln!("rm: missing operand");
        return ExitCode::FAILURE;
    }

    if config.interactive == InteractiveMode::Once {
        let should_prompt = config.paths.len() > 3 || config.recursive;
        if should_prompt {
            let msg = if config.recursive {
                format!("remove {} argument(s) recursively", config.paths.len())
            } else {
                format!("remove {} arguments", config.paths.len())
            };
            if !prompt_once(&msg) {
                return ExitCode::SUCCESS;
            }
        }
    }

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stderr = io::stderr();
    let mut writer = stderr.lock();

    let mut exit_code = 0u8;

    for path_str in &config.paths {
        if let Err(e) = remove_path(Path::new(path_str), &config, &mut reader, &mut writer) {
            eprintln!("rm: {}", e);
            exit_code = 1;
        }
    }

    ExitCode::from(exit_code)
}

fn prompt_once(message: &str) -> bool {
    eprint!("rm: {}? ", message);
    let _ = io::stderr().flush();
    let mut response = String::new();
    if io::stdin().read_line(&mut response).is_err() {
        return false;
    }
    let trimmed = response.trim().to_lowercase();
    trimmed == "y" || trimmed == "yes"
}
