use std::process::ExitCode;

use clap::Parser;

use pkill::cli::PkillConfig;
use pkill::ops::pkill;

fn main() -> ExitCode {
    let config = PkillConfig::parse();

    if config.pattern.is_empty() {
        return ExitCode::from(2);
    }

    ExitCode::from(pkill(&config) as u8)
}
