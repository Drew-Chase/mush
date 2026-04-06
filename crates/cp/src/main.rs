use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

use cp::cli::CpConfig;
use cp::ops::copy_path;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = CpConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    if config.paths.is_empty() {
        eprintln!("cp: missing file operand");
        return ExitCode::FAILURE;
    }

    if config.target_directory.is_some() && config.no_target_directory {
        eprintln!("cp: cannot combine --target-directory and --no-target-directory");
        return ExitCode::FAILURE;
    }

    let (sources, target) = match resolve_targets(&config) {
        Ok(result) => result,
        Err(msg) => {
            eprintln!("cp: {msg}");
            return ExitCode::FAILURE;
        }
    };

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stderr = io::stderr();
    let mut writer = stderr.lock();

    let mut exit_code = 0u8;

    for source_str in &sources {
        let source = PathBuf::from(source_str);

        let dest = if target.is_dir() && !config.no_target_directory {
            match source.file_name() {
                Some(name) => target.join(name),
                None => {
                    eprintln!(
                        "cp: cannot determine filename from '{}'",
                        source.display()
                    );
                    exit_code = 1;
                    continue;
                }
            }
        } else {
            target.clone()
        };

        if let Err(e) = copy_path(&source, &dest, &config, &mut reader, &mut writer) {
            eprintln!("cp: {e}");
            exit_code = 1;
        }
    }

    ExitCode::from(exit_code)
}

fn resolve_targets(config: &CpConfig) -> Result<(Vec<String>, PathBuf), String> {
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
