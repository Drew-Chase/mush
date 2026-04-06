use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use mv::cli::MvConfig;
use mv::ops::move_path;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = MvConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    if config.paths.is_empty() {
        eprintln!("mv: missing file operand");
        return ExitCode::FAILURE;
    }

    if config.target_directory.is_some() && config.no_target_directory {
        eprintln!("mv: cannot combine --target-directory and --no-target-directory");
        return ExitCode::FAILURE;
    }

    let (sources, target) = match resolve_targets(&config) {
        Ok(result) => result,
        Err(msg) => {
            eprintln!("mv: {msg}");
            return ExitCode::FAILURE;
        }
    };

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stderr = io::stderr();
    let mut writer = stderr.lock();

    let mut exit_code = 0u8;

    for source_str in &sources {
        let source = if config.strip_trailing_slashes {
            strip_slashes(Path::new(source_str))
        } else {
            PathBuf::from(source_str)
        };

        let dest = if target.is_dir() && !config.no_target_directory {
            match source.file_name() {
                Some(name) => target.join(name),
                None => {
                    eprintln!(
                        "mv: cannot determine filename from '{}'",
                        source.display()
                    );
                    exit_code = 1;
                    continue;
                }
            }
        } else {
            target.clone()
        };

        if let Err(e) = move_path(&source, &dest, &config, &mut reader, &mut writer) {
            eprintln!("mv: {e}");
            exit_code = 1;
        }
    }

    ExitCode::from(exit_code)
}

fn resolve_targets(config: &MvConfig) -> Result<(Vec<String>, PathBuf), String> {
    if let Some(ref dir) = config.target_directory {
        let target = PathBuf::from(dir);
        if !target.is_dir() {
            return Err(format!("target '{}' is not a directory", target.display()));
        }
        Ok((config.paths.clone(), target))
    } else if config.paths.len() == 1 {
        Err(format!(
            "missing destination file operand after '{}'",
            config.paths[0]
        ))
    } else if config.paths.len() == 2 {
        let target = PathBuf::from(&config.paths[1]);
        let sources = vec![config.paths[0].clone()];
        Ok((sources, target))
    } else {
        let last = config.paths.len() - 1;
        let target = PathBuf::from(&config.paths[last]);
        if !target.is_dir() {
            return Err(format!("target '{}' is not a directory", target.display()));
        }
        let sources = config.paths[..last].to_vec();
        Ok((sources, target))
    }
}

fn strip_slashes(path: &Path) -> PathBuf {
    let s = path.to_string_lossy();
    let trimmed = s.trim_end_matches(['/', '\\']);
    if trimmed.is_empty() {
        PathBuf::from(&*s)
    } else {
        PathBuf::from(trimmed)
    }
}
