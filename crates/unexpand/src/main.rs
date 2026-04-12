use std::fs::File;
use std::io::{self, Write};
use std::process::ExitCode;

use unexpand::cli::UnexpandConfig;
use unexpand::ops;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = UnexpandConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let mut exit_code = 0u8;

    for file in &files {
        if file == "-" {
            let mut stdin = io::stdin().lock();
            if let Err(e) = ops::unexpand(&mut stdin, &mut out, &config) {
                eprintln!("unexpand: error reading standard input: {e}");
                exit_code = 1;
            }
        } else {
            match File::open(file) {
                Ok(mut f) => {
                    if let Err(e) = ops::unexpand(&mut f, &mut out, &config) {
                        eprintln!("unexpand: {file}: {e}");
                        exit_code = 1;
                    }
                }
                Err(e) => {
                    eprintln!("unexpand: {file}: {e}");
                    exit_code = 1;
                }
            }
        }
    }

    let _ = out.flush();
    ExitCode::from(exit_code)
}
