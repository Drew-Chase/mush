use std::process::ExitCode;

use clap::Parser;

use mktemp::cli::MktempConfig;
use mktemp::ops::run;

fn main() -> ExitCode {
    let config = MktempConfig::parse();

    if let Err(e) = run(&config) {
        if !config.quiet {
            eprintln!("mktemp: {e}");
        }
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
