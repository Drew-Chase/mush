use std::fs::File;
use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;

use fold::cli::FoldConfig;
use fold::ops;

fn main() -> ExitCode {
    let config = FoldConfig::parse();

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
            if let Err(e) = ops::fold(&mut stdin, &mut out, &config) {
                eprintln!("fold: error reading standard input: {e}");
                exit_code = 1;
            }
        } else {
            match File::open(file) {
                Ok(mut f) => {
                    if let Err(e) = ops::fold(&mut f, &mut out, &config) {
                        eprintln!("fold: {file}: {e}");
                        exit_code = 1;
                    }
                }
                Err(e) => {
                    eprintln!("fold: {file}: {e}");
                    exit_code = 1;
                }
            }
        }
    }

    let _ = out.flush();
    ExitCode::from(exit_code)
}
