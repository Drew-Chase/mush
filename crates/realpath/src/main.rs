use std::io::{self, Write};
use std::process::ExitCode;

use realpath::cli::RealpathConfig;
use realpath::ops::resolve_path;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = RealpathConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    if config.files.is_empty() {
        eprintln!("realpath: missing operand");
        return ExitCode::FAILURE;
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let terminator = if config.zero { '\0' } else { '\n' };
    let mut exit_code = ExitCode::SUCCESS;

    for file in &config.files {
        match resolve_path(file, &config) {
            Ok(resolved) => {
                let _ = write!(out, "{}{terminator}", resolved.display());
            }
            Err(e) => {
                if !config.quiet {
                    eprintln!("realpath: {e}");
                }
                exit_code = ExitCode::FAILURE;
            }
        }
    }

    let _ = out.flush();
    exit_code
}
