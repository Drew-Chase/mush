use std::fs::File;
use std::io::{self, BufReader};
use std::process::ExitCode;

use clap::Parser;

use wc::cli::WcConfig;
use wc::ops::{WcCounts, count, format_counts};

fn main() -> ExitCode {
    let mut config = WcConfig::parse();
    config.apply_defaults();

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let mut total = WcCounts::default();
    let mut exit_code = ExitCode::SUCCESS;

    for filename in &files {
        let counts = if filename == "-" {
            let mut stdin = io::stdin().lock();
            count(&mut stdin, &config)
        } else {
            match File::open(filename) {
                Ok(f) => {
                    let mut reader = BufReader::new(f);
                    count(&mut reader, &config)
                }
                Err(e) => {
                    eprintln!("wc: {filename}: {e}");
                    exit_code = ExitCode::FAILURE;
                    continue;
                }
            }
        };

        total.add(&counts);

        let display_name = if filename == "-" { None } else { Some(filename.as_str()) };
        println!("{}", format_counts(&counts, &config, display_name));
    }

    if files.len() > 1 {
        println!("{}", format_counts(&total, &config, Some("total")));
    }

    exit_code
}
