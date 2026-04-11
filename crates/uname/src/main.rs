use std::process::ExitCode;

use uname::cli::UnameConfig;
use uname::ops::get_system_info;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = UnameConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let info = get_system_info(&config);
    println!("{info}");

    ExitCode::SUCCESS
}
