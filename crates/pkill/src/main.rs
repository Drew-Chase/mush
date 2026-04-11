use std::process::ExitCode;

use pkill::cli::PkillConfig;
use pkill::ops::pkill;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = PkillConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    if config.pattern.is_empty() {
        return ExitCode::from(2);
    }

    ExitCode::from(pkill(&config) as u8)
}
