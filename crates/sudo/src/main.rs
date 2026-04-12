use std::process::ExitCode;

use clap::Parser;

use sudo::cli::SudoConfig;
use sudo::ops::execute;

fn main() -> ExitCode {
    let config = SudoConfig::parse();

    if config.command.is_empty() && !config.login && !config.shell {
        eprintln!("sudo: no command specified");
        return ExitCode::from(1);
    }

    execute(&config)
}
