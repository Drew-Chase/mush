use std::fs;
use std::io::{self, BufReader, Write};
use std::process::ExitCode;

use awk::cli::AwkConfig;
use awk::ops::run;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = AwkConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    // Get the program text
    let program = if let Some(ref prog_file) = config.program_file {
        match fs::read_to_string(prog_file) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("awk: can't open file {prog_file}: {e}");
                return ExitCode::from(2);
            }
        }
    } else {
        config.program.clone()
    };

    if program.is_empty() && config.program_file.is_none() {
        eprintln!("awk: no program text");
        return ExitCode::from(2);
    }

    let files = if config.files.is_empty() {
        vec!["-".to_string()]
    } else {
        config.files.clone()
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();

    for file in &files {
        let result = if file == "-" {
            let stdin = io::stdin();
            let mut reader = BufReader::new(stdin.lock());
            run(&program, &config, &mut reader, &mut out)
        } else {
            match fs::File::open(file) {
                Ok(f) => {
                    let mut reader = BufReader::new(f);
                    run(&program, &config, &mut reader, &mut out)
                }
                Err(e) => {
                    eprintln!("awk: can't open file {file}: {e}");
                    return ExitCode::from(2);
                }
            }
        };

        match result {
            Ok(0) => {}
            Ok(code) => return ExitCode::from(code as u8),
            Err(e) => {
                eprintln!("awk: {e}");
                return ExitCode::from(2);
            }
        }
    }

    let _ = out.flush();
    ExitCode::SUCCESS
}
