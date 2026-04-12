use std::fs;
use std::io::{self, BufReader, Write};
use std::process::ExitCode;

use clap::Parser;

use awk::cli::AwkConfig;
use awk::ops::run;

fn main() -> ExitCode {
    let config = AwkConfig::parse();

    // Get the program text
    let program = if let Some(ref prog_file) = config.program_file {
        match fs::read_to_string(prog_file) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("awk: can't open file {prog_file}: {e}");
                return ExitCode::from(2);
            }
        }
    } else if let Some(prog) = config.program() {
        prog.to_string()
    } else {
        eprintln!("awk: no program text");
        return ExitCode::from(2);
    };

    if program.is_empty() && config.program_file.is_none() {
        eprintln!("awk: no program text");
        return ExitCode::from(2);
    }

    let files_slice = config.files();
    let files = if files_slice.is_empty() {
        vec!["-".to_string()]
    } else {
        files_slice.to_vec()
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
