use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

use grep::cli::GrepConfig;
use grep::ops::{build_regex, grep_reader, grep_recursive, GrepResult};

fn main() -> ExitCode {
    let config = GrepConfig::parse();

    let re = match build_regex(&config) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("grep: invalid pattern: {e}");
            return ExitCode::from(2);
        }
    };

    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let mut overall = GrepResult::default();
    let mut had_error = false;

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    for filename in &files {
        if filename == "-" {
            let stdin = io::stdin();
            let mut reader = stdin.lock();
            let result = grep_reader(&mut reader, None, &config, &re, &mut writer);
            overall.merge(&result);
        } else if config.recursive && Path::new(filename).is_dir() {
            let result = grep_recursive(Path::new(filename), &config, &re, &mut writer);
            overall.merge(&result);
        } else {
            match File::open(filename) {
                Ok(f) => {
                    let mut reader = BufReader::new(f);
                    let result = grep_reader(
                        &mut reader,
                        Some(filename),
                        &config,
                        &re,
                        &mut writer,
                    );
                    overall.merge(&result);
                }
                Err(e) => {
                    eprintln!("grep: {filename}: {e}");
                    had_error = true;
                }
            }
        }
    }

    if had_error {
        ExitCode::from(2)
    } else if overall.had_match {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
