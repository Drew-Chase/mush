use std::io;
use std::process::ExitCode;

use clap::Parser;

use reset::cli::ResetConfig;
use reset::ops::reset_terminal;

fn main() -> ExitCode {
    let _config = ResetConfig::parse();

    let stdout = io::stdout();
    let mut out = stdout.lock();

    if let Err(e) = reset_terminal(&mut out) {
        eprintln!("reset: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
