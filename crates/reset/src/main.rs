use std::io;
use std::process::ExitCode;

use reset::cli::ResetConfig;
use reset::ops::reset_terminal;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(_config) = ResetConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();

    if let Err(e) = reset_terminal(&mut out) {
        eprintln!("reset: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
