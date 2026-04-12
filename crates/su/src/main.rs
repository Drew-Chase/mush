use std::process::ExitCode;

use clap::Parser;

use su::cli::SuConfig;
use su::ops::execute;

fn main() -> ExitCode {
    let config = SuConfig::parse();

    execute(&config)
}
