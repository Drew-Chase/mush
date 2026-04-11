use std::process::ExitCode;

use chmod::cli::ChmodConfig;
use chmod::ops::chmod;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = ChmodConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
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
