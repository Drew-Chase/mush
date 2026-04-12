use std::process::ExitCode;

use clap::Parser;

use chown::cli::ChownConfig;
use chown::ops::chown;

fn main() -> ExitCode {
    let config = match ChownConfig::parse().resolve() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(_e) = chown(&config) {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
