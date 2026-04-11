use std::io;
use std::process::ExitCode;

use tr::cli::TrConfig;
use tr::ops;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = TrConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();

    if let Err(e) = ops::translate(&mut input, &mut output, &config) {
        eprintln!("tr: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
