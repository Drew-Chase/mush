use std::process::ExitCode;

use yes::cli::YesConfig;
use yes::ops::yes_loop;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = YesConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let _ = yes_loop(&config.string);
    ExitCode::SUCCESS
}
