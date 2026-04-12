use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

use du::cli::DuConfig;
use du::ops::{du_path, format_size};

fn main() -> ExitCode {
    let config = DuConfig::parse();

    let files = if config.files.is_empty() {
        vec![".".to_string()]
    } else {
        config.files.clone()
    };

    let mut exit_code = ExitCode::SUCCESS;
    let mut grand_total: u64 = 0;

    for filename in &files {
        let path = Path::new(filename);
        let mut output = Vec::new();

        match du_path(path, &config, 0, &mut output) {
            Ok(size) => {
                grand_total += size;

                for (entry_size, entry_path) in &output {
                    println!("{}\t{entry_path}", format_size(*entry_size, &config));
                }

                if config.summarize {
                    println!("{}\t{filename}", format_size(size, &config));
                }
            }
            Err(e) => {
                eprintln!("du: cannot access '{filename}': {e}");
                exit_code = ExitCode::FAILURE;
            }
        }
    }

    if config.total {
        println!("{}\ttotal", format_size(grand_total, &config));
    }

    exit_code
}
