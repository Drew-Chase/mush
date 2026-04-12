use std::process::ExitCode;

use clap::Parser;

use arch::cli::ArchConfig;
use arch::ops::machine_arch;

fn main() -> ExitCode {
    let _config = ArchConfig::parse();

    println!("{}", machine_arch());
    ExitCode::SUCCESS
}
