use std::process::ExitCode;

use clap::Parser;

use yes::cli::YesConfig;
use yes::ops::yes_loop;

fn main() -> ExitCode {
    let config = YesConfig::parse();
    let string = config.string();

    let _ = yes_loop(&string);
    ExitCode::SUCCESS
}
