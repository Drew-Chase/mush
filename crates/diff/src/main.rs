use std::fs;
use std::process::ExitCode;

use clap::Parser;

use diff::cli::DiffConfig;
use diff::ops::{compute_diff, format_github, format_normal, format_side_by_side, format_unified};

fn main() -> ExitCode {
    let config = DiffConfig::parse();

    let content1 = match fs::read_to_string(&config.file1) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("diff: {}: {e}", config.file1);
            return ExitCode::from(2);
        }
    };

    let content2 = match fs::read_to_string(&config.file2) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("diff: {}: {e}", config.file2);
            return ExitCode::from(2);
        }
    };

    let lines1: Vec<&str> = content1.lines().collect();
    let lines2: Vec<&str> = content2.lines().collect();

    let hunks = compute_diff(&lines1, &lines2, &config);

    if hunks.is_empty() {
        if config.report_identical {
            println!("Files {} and {} are identical", config.file1, config.file2);
        }
        return ExitCode::SUCCESS;
    }

    if config.brief {
        println!(
            "Files {} and {} differ",
            config.file1, config.file2
        );
        return ExitCode::from(1);
    }

    let output = if config.github {
        format_github(&hunks, &config.file1, &config.file2, config.color)
    } else if config.unified.is_some() {
        format_unified(&hunks, &config.file1, &config.file2, config.color)
    } else if config.side_by_side {
        format_side_by_side(&hunks, &lines1, &lines2, config.width)
    } else {
        format_normal(&hunks)
    };

    for line in &output {
        println!("{line}");
    }

    ExitCode::from(1)
}
