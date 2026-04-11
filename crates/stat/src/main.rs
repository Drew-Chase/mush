use std::process::ExitCode;

use stat::cli::StatConfig;
use stat::ops::stat_file;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = StatConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let mut exit_code = 0u8;

    for file in &config.files {
        let path = std::path::Path::new(file);
        match stat_file(path, &config) {
            Ok(output) => println!("{output}"),
            Err(e) => {
                eprintln!("stat: cannot stat '{}': {}", file, e);
                exit_code = 1;
            }
        }
    }

    ExitCode::from(exit_code)
}
