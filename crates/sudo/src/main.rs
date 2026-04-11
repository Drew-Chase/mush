use std::process::ExitCode;

use sudo::cli::SudoConfig;
use sudo::ops::execute;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = SudoConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    if config.command.is_empty() && !config.login && !config.shell {
        eprintln!("sudo: no command specified");
        return ExitCode::from(1);
    }

    execute(&config)
}
