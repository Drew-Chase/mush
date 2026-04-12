use std::process::ExitCode;

use clap::Parser;

use timeout::cli::TimeoutConfig;
use timeout::ops;

fn main() -> ExitCode {
    let mut config = TimeoutConfig::parse();

    if let Err(e) = config.resolve() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    match ops::run_with_timeout(&config) {
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            eprintln!("timeout: {e}");
            ExitCode::from(125)
        }
    }
}
