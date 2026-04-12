use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

use touch::cli::TouchConfig;
use touch::ops::touch;

fn main() -> ExitCode {
    let config = TouchConfig::parse();

    if config.files.is_empty() {
        eprintln!("touch: missing file operand");
        eprintln!("Try 'touch --help' for more information.");
        return ExitCode::SUCCESS;
    }

    let mut exit_code = 0u8;

    for file in &config.files {
        if let Err(e) = touch(Path::new(file), &config) {
            eprintln!("touch: {file}: {e}");
            exit_code = 1;
        }
    }

    ExitCode::from(exit_code)
}
