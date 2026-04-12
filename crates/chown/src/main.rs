use std::process::ExitCode;

use chown::cli::ChownConfig;
use chown::ops::chown;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = ChownConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    if let Err(_e) = chown(&config) {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
