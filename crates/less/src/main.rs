use std::fs;
use std::io::{self, BufRead};
use std::process::ExitCode;

use clap::Parser;

use less::cli::LessConfig;
use less::ops::Pager;

fn main() -> ExitCode {
    let mut config = LessConfig::parse();

    // Handle +NUM and +/PATTERN from raw args (clap can't parse these)
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    for arg in &raw_args {
        if arg.starts_with("+/") && arg.len() > 2 {
            config.start_pattern = Some(arg[2..].to_string());
        } else if arg.starts_with('+') && arg.len() > 1
            && let Ok(n) = arg[1..].parse::<usize>()
        {
            config.start_line = Some(n);
        }
    }

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    // Collect all lines from all files
    let mut all_lines = Vec::new();
    for filename in &files {
        let lines = if filename == "-" {
            let stdin = io::stdin().lock();
            match stdin.lines().collect::<Result<Vec<_>, _>>() {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("less: error reading standard input: {e}");
                    return ExitCode::FAILURE;
                }
            }
        } else {
            match fs::read_to_string(filename) {
                Ok(content) => content.lines().map(String::from).collect(),
                Err(e) => {
                    eprintln!("less: cannot open '{filename}': {e}");
                    return ExitCode::FAILURE;
                }
            }
        };
        all_lines.extend(lines);
    }

    // Quit if one screen and -F is set
    if config.quit_if_one_screen
        && let Ok((_, height)) = crossterm::terminal::size()
    {
        let visible = height.saturating_sub(1) as usize;
        if all_lines.len() <= visible {
            for line in &all_lines {
                println!("{line}");
            }
            return ExitCode::SUCCESS;
        }
    }

    let mut pager = Pager::new(all_lines, config);
    if let Err(e) = pager.run() {
        eprintln!("less: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
