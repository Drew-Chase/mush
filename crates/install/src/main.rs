use std::process::ExitCode;

use install::cli::InstallConfig;
use install::ops::install_files;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = InstallConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    if let Err(e) = install_files(&config) {
        eprintln!("install: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
