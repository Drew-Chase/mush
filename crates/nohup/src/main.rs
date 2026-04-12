use std::process::ExitCode;

use clap::Parser;

use nohup::cli::NohupConfig;
use nohup::ops::execute;

fn main() -> ExitCode {
    let config = NohupConfig::parse();

    if config.command.is_empty() {
        eprintln!("nohup: missing operand");
        return ExitCode::FAILURE;
    }

    execute(&config)
}
