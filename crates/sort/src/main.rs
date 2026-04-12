use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::ExitCode;

use clap::Parser;

use sort::cli::SortConfig;
use sort::ops::{check_sorted, sort_lines};

fn read_lines_from(filename: &str) -> Result<Vec<String>, io::Error> {
    if filename == "-" {
        let stdin = io::stdin().lock();
        stdin.lines().collect()
    } else {
        let file = File::open(filename)?;
        BufReader::new(file).lines().collect()
    }
}

fn main() -> ExitCode {
    let mut config = SortConfig::parse();

    if let Err(e) = config.resolve() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let mut all_lines: Vec<String> = Vec::new();

    for filename in &files {
        match read_lines_from(filename) {
            Ok(lines) => all_lines.extend(lines),
            Err(e) => {
                eprintln!("sort: {filename}: {e}");
                return ExitCode::FAILURE;
            }
        }
    }

    if config.check {
        return if check_sorted(&all_lines, &config) {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }

    sort_lines(&mut all_lines, &config);

    if let Some(ref outfile) = config.output {
        match File::create(outfile) {
            Ok(mut f) => {
                for line in &all_lines {
                    if writeln!(f, "{line}").is_err() {
                        eprintln!("sort: write error");
                        return ExitCode::FAILURE;
                    }
                }
            }
            Err(e) => {
                eprintln!("sort: {outfile}: {e}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        for line in &all_lines {
            println!("{line}");
        }
    }

    ExitCode::SUCCESS
}
