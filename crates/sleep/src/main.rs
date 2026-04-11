use std::process::ExitCode;

use sleep::cli::SleepConfig;
use sleep::ops::sleep;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = SleepConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    sleep(config.seconds);
    ExitCode::SUCCESS
}
