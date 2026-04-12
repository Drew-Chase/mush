use std::fs::File;
use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;

use cat::cli::CatConfig;
use cat::ops::cat;

fn main() -> ExitCode {
    let mut config = CatConfig::parse();
    config.resolve();

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let mut line_num: usize = 0;
    let mut exit_code = 0u8;

    for file in &files {
        if file == "-" {
            let mut stdin = io::stdin().lock();
            if let Err(e) = cat(&mut stdin, &config, &mut out, &mut line_num) {
                eprintln!("cat: error reading standard input: {e}");
                exit_code = 1;
            }
        } else {
            match File::open(file) {
                Ok(mut f) => {
                    if let Err(e) = cat(&mut f, &config, &mut out, &mut line_num) {
                        eprintln!("cat: {file}: {e}");
                        exit_code = 1;
                    }
                }
                Err(e) => {
                    eprintln!("cat: {file}: {e}");
                    exit_code = 1;
                }
            }
        }
    }

    let _ = out.flush();
    ExitCode::from(exit_code)
}
