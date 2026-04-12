use std::process::ExitCode;

use clap::Parser;

use install::cli::InstallConfig;
use install::ops::install_files;

fn main() -> ExitCode {
    let config = InstallConfig::parse();

    if let Err(e) = install_files(&config) {
        eprintln!("install: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
