use std::io;
use std::process::ExitCode;

use tput::cli::TputConfig;
use tput::ops::execute_capability;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = TputConfig::from_args(&args) else {
        return ExitCode::FAILURE;
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();

    if let Err(e) = execute_capability(&config.capability, &mut out) {
        eprintln!("tput: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
