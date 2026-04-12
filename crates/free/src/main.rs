use std::process::ExitCode;

use free::cli::FreeConfig;
use free::ops::format_output;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = FreeConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let lines = format_output(&config);
    for line in &lines {
        println!("{line}");
    }

    ExitCode::SUCCESS
}
