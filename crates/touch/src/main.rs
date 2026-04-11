use std::path::Path;
use std::process::ExitCode;

use touch::cli::TouchConfig;
use touch::ops::touch;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = TouchConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let mut exit_code = 0u8;

    for file in &config.files {
        if let Err(e) = touch(Path::new(file), &config) {
            eprintln!("touch: {file}: {e}");
            exit_code = 1;
        }
    }

    ExitCode::from(exit_code)
}
