use std::fs::File;
use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;

use column::cli::ColumnConfig;
use column::ops;

fn main() -> ExitCode {
    let config = ColumnConfig::parse();

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
            if let Err(e) = ops::column(&mut stdin, &mut out, &config) {
                eprintln!("column: error reading standard input: {e}");
                exit_code = 1;
            }
        } else {
            match File::open(file) {
                Ok(mut f) => {
                    if let Err(e) = ops::column(&mut f, &mut out, &config) {
                        eprintln!("column: {file}: {e}");
                        exit_code = 1;
                    }
                }
                Err(e) => {
                    eprintln!("column: {file}: {e}");
                    exit_code = 1;
                }
            }
        }
    }

    let _ = out.flush();
    ExitCode::from(exit_code)
}
