use std::io;
use std::process::ExitCode;

use clap::Parser;

use tput::cli::TputConfig;
use tput::ops::execute_capability;

fn main() -> ExitCode {
    let mut config = TputConfig::parse();

    if let Err(e) = config.resolve() {
        eprintln!("tput: {e}");
        return ExitCode::FAILURE;
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();

    if let Err(e) = execute_capability(config.get_capability(), &mut out) {
        eprintln!("tput: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
