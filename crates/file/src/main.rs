use std::process::ExitCode;

use clap::Parser;

use file::cli::FileConfig;
use file::ops::run;

fn main() -> ExitCode {
    let config = FileConfig::parse();

    if let Err(_e) = run(&config) {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
