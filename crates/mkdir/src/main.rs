use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

use mkdir::cli::MkdirConfig;
use mkdir::ops::create_directory;

fn main() -> ExitCode {
    let config = MkdirConfig::parse();

    if config.directories.is_empty() {
        eprintln!("mkdir: missing operand");
        return ExitCode::FAILURE;
    }

    let mut exit_code = 0u8;

    for dir in &config.directories {
        if let Err(e) = create_directory(Path::new(dir), &config) {
            eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
            exit_code = 1;
        }
    }

    ExitCode::from(exit_code)
}
