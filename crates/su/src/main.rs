use std::process::ExitCode;

use su::cli::SuConfig;
use su::ops::execute;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = SuConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    execute(&config)
}
