use std::process::ExitCode;

use arch::cli::ArchConfig;
use arch::ops::machine_arch;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(_config) = ArchConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    println!("{}", machine_arch());
    ExitCode::SUCCESS
}
