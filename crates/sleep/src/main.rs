use std::process::ExitCode;

use clap::Parser;

use sleep::cli::SleepConfig;
use sleep::ops::sleep;

fn main() -> ExitCode {
    let config = SleepConfig::parse();
    let seconds = config.parse_duration();

    sleep(seconds);
    ExitCode::SUCCESS
}
