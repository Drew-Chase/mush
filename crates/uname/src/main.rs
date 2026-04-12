use std::process::ExitCode;

use clap::Parser;

use uname::cli::UnameConfig;
use uname::ops::get_system_info;

fn main() -> ExitCode {
    let mut config = UnameConfig::parse();
    config.resolve();

    let info = get_system_info(&config);
    println!("{info}");

    ExitCode::SUCCESS
}
