use std::fs;
use std::io::{self, BufRead};
use std::process::ExitCode;

use clap::Parser;

use more::cli::MoreConfig;
use more::ops::more;

fn main() -> ExitCode {
    let config = MoreConfig::parse();

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    for filename in &files {
        let lines = if filename == "-" {
            let stdin = io::stdin().lock();
            match stdin.lines().collect::<Result<Vec<_>, _>>() {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("more: error reading standard input: {e}");
                    return ExitCode::FAILURE;
                }
            }
        } else {
            match fs::read_to_string(filename) {
                Ok(content) => content.lines().map(String::from).collect(),
                Err(e) => {
                    eprintln!("more: cannot open '{filename}': {e}");
                    return ExitCode::FAILURE;
                }
            }
        };

        if let Err(e) = more(&lines, &config) {
            eprintln!("more: {e}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
