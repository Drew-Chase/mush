use std::process::ExitCode;

use nohup::cli::NohupConfig;
use nohup::ops::execute;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = NohupConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    execute(&config)
}
