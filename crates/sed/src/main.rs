use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::process::ExitCode;

use clap::Parser;

use sed::cli::SedConfig;
use sed::ops::{parse_script, sed_process};

fn main() -> ExitCode {
    let config = SedConfig::parse();

    // Collect all scripts
    let mut all_scripts = config.effective_scripts();

    // Read script files
    for script_file in &config.script_files {
        match fs::read_to_string(script_file) {
            Ok(content) => all_scripts.push(content),
            Err(e) => {
                eprintln!("sed: couldn't open file {script_file}: {e}");
                return ExitCode::from(1);
            }
        }
    }

    if all_scripts.is_empty() {
        eprintln!("sed: no script command has been given");
        return ExitCode::from(1);
    }

    // Parse all scripts into commands
    let combined_script = all_scripts.join("; ");
    let commands = match parse_script(&combined_script, config.extended_regexp) {
        Ok(cmds) => cmds,
        Err(e) => {
            eprintln!("sed: {e}");
            return ExitCode::from(1);
        }
    };

    let files_slice = config.effective_files();
    let files = if files_slice.is_empty() {
        vec!["-".to_string()]
    } else {
        files_slice.to_vec()
    };

    let mut exit_code = 0u8;

    for file in &files {
        if file == "-" {
            let stdin = io::stdin();
            let mut reader = BufReader::new(stdin.lock());
            let stdout = io::stdout();
            let mut out = stdout.lock();
            if let Err(e) = sed_process(&mut reader, &commands, &config, &mut out) {
                eprintln!("sed: error reading standard input: {e}");
                exit_code = 1;
            }
        } else if config.in_place.is_some() {
            // In-place editing
            let content = match fs::read_to_string(file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("sed: {file}: {e}");
                    exit_code = 1;
                    continue;
                }
            };

            // Create backup if suffix provided
            if let Some(Some(suffix)) = &config.in_place {
                let backup = format!("{file}{suffix}");
                if let Err(e) = fs::copy(file, &backup) {
                    eprintln!("sed: couldn't create backup {backup}: {e}");
                    exit_code = 1;
                    continue;
                }
            }

            let mut output = Vec::new();
            let mut reader = BufReader::new(content.as_bytes());
            if let Err(e) = sed_process(&mut reader, &commands, &config, &mut output) {
                eprintln!("sed: {file}: {e}");
                exit_code = 1;
                continue;
            }

            match File::create(file) {
                Ok(mut f) => {
                    if let Err(e) = f.write_all(&output) {
                        eprintln!("sed: {file}: {e}");
                        exit_code = 1;
                    }
                }
                Err(e) => {
                    eprintln!("sed: {file}: {e}");
                    exit_code = 1;
                }
            }
        } else {
            match File::open(file) {
                Ok(f) => {
                    let mut reader = BufReader::new(f);
                    let stdout = io::stdout();
                    let mut out = stdout.lock();
                    if let Err(e) = sed_process(&mut reader, &commands, &config, &mut out) {
                        eprintln!("sed: {file}: {e}");
                        exit_code = 1;
                    }
                }
                Err(e) => {
                    eprintln!("sed: {file}: {e}");
                    exit_code = 1;
                }
            }
        }
    }

    ExitCode::from(exit_code)
}
