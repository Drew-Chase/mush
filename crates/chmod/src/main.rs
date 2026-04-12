use std::process::ExitCode;

use clap::Parser;

use chmod::cli::ChmodConfig;
use chmod::ops::chmod;

fn main() -> ExitCode {
    let config = match ChmodConfig::parse().resolve() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    let mut exit_code = 0u8;

    for file in &config.files {
        let path = std::path::Path::new(file);
        if let Err(e) = chmod(path, &config) {
            if !config.quiet {
                eprintln!("chmod: cannot access '{}': {}", file, e);
            }
            exit_code = 1;
        }
    }

    ExitCode::from(exit_code)
}
