use std::io::Read;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::cli::XargsConfig;

/// Read items from input, splitting by the appropriate delimiter.
pub fn read_items(input: &mut dyn Read, config: &XargsConfig) -> Vec<String> {
    let mut buf = Vec::new();
    input.read_to_end(&mut buf).unwrap_or(0);

    if config.null {
        // Split on NUL bytes
        buf.split(|&b| b == 0)
            .map(|chunk| String::from_utf8_lossy(chunk).to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else if let Some(delim) = config.delimiter {
        // Split on custom delimiter
        let text = String::from_utf8_lossy(&buf).to_string();
        text.split(delim)
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        // Default: split on whitespace (handling quoted strings simply)
        let text = String::from_utf8_lossy(&buf).to_string();
        split_whitespace_items(&text)
    }
}

/// Split input by whitespace, treating newlines as whitespace.
fn split_whitespace_items(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

/// Build command invocations from items based on config.
pub fn build_commands(items: &[String], config: &XargsConfig) -> Vec<Vec<String>> {
    if items.is_empty() {
        return Vec::new();
    }

    let base_cmd = &config.command;

    // -I replace mode: one item per invocation, replace occurrences
    if let Some(ref replace_str) = config.replace {
        return items
            .iter()
            .map(|item| {
                base_cmd
                    .iter()
                    .map(|part| part.replace(replace_str.as_str(), item))
                    .collect()
            })
            .collect();
    }

    // -L max-lines mode
    if let Some(max_lines) = config.max_lines {
        return items
            .chunks(max_lines)
            .map(|chunk| {
                let mut cmd = base_cmd.clone();
                cmd.extend(chunk.iter().cloned());
                maybe_truncate_cmd(cmd, config)
            })
            .collect();
    }

    // -n max-args mode
    if let Some(max_args) = config.max_args {
        return items
            .chunks(max_args)
            .map(|chunk| {
                let mut cmd = base_cmd.clone();
                cmd.extend(chunk.iter().cloned());
                maybe_truncate_cmd(cmd, config)
            })
            .collect();
    }

    // Default: all items in one invocation
    let mut cmd = base_cmd.clone();
    cmd.extend(items.iter().cloned());
    vec![maybe_truncate_cmd(cmd, config)]
}

/// If -s is set, truncate the command to fit within max_chars.
fn maybe_truncate_cmd(cmd: Vec<String>, config: &XargsConfig) -> Vec<String> {
    if let Some(max_chars) = config.max_chars {
        let mut total = 0;
        let mut result = Vec::new();
        for (i, part) in cmd.iter().enumerate() {
            let add = if i > 0 { 1 + part.len() } else { part.len() };
            if total + add > max_chars {
                break;
            }
            total += add;
            result.push(part.clone());
        }
        result
    } else {
        cmd
    }
}

/// Execute commands, returning the worst (highest) exit code.
pub fn execute_commands(commands: &[Vec<String>], config: &XargsConfig) -> i32 {
    if commands.is_empty() {
        return 0;
    }

    if config.max_procs > 1 {
        execute_parallel(commands, config)
    } else {
        execute_sequential(commands, config)
    }
}

fn execute_sequential(commands: &[Vec<String>], config: &XargsConfig) -> i32 {
    let mut worst = 0i32;

    for cmd in commands {
        if cmd.is_empty() {
            continue;
        }

        if config.verbose {
            eprintln!("{}", cmd.join(" "));
        }

        if config.interactive {
            eprint!("{} ?...", cmd.join(" "));
            let mut response = String::new();
            if std::io::stdin().read_line(&mut response).is_err() {
                continue;
            }
            let response = response.trim().to_lowercase();
            if response != "y" && response != "yes" {
                continue;
            }
        }

        let status = Command::new(&cmd[0])
            .args(&cmd[1..])
            .status();

        match status {
            Ok(s) => {
                let code = s.code().unwrap_or(126);
                if code > worst {
                    worst = code;
                }
            }
            Err(e) => {
                eprintln!("xargs: {}: {e}", cmd[0]);
                if 127 > worst {
                    worst = 127;
                }
            }
        }
    }

    worst
}

fn execute_parallel(commands: &[Vec<String>], config: &XargsConfig) -> i32 {
    let worst = Arc::new(Mutex::new(0i32));
    let semaphore = Arc::new(Mutex::new(config.max_procs));
    let mut handles = Vec::new();

    for cmd in commands {
        if cmd.is_empty() {
            continue;
        }

        let cmd = cmd.clone();
        let verbose = config.verbose;
        let worst = Arc::clone(&worst);
        let semaphore = Arc::clone(&semaphore);

        // Simple semaphore: wait for a slot
        loop {
            let mut slots = semaphore.lock().unwrap();
            if *slots > 0 {
                *slots -= 1;
                break;
            }
            drop(slots);
            thread::yield_now();
        }

        let handle = thread::spawn(move || {
            if verbose {
                eprintln!("{}", cmd.join(" "));
            }

            let status = Command::new(&cmd[0])
                .args(&cmd[1..])
                .status();

            let code = match status {
                Ok(s) => s.code().unwrap_or(126),
                Err(e) => {
                    eprintln!("xargs: {}: {e}", cmd[0]);
                    127
                }
            };

            let mut w = worst.lock().unwrap();
            if code > *w {
                *w = code;
            }

            let mut slots = semaphore.lock().unwrap();
            *slots += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    let w = worst.lock().unwrap();
    *w
}
