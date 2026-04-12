use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

use ln::cli::LnConfig;
use ln::ops::create_link;

fn main() -> ExitCode {
    let config = LnConfig::parse();

    if config.targets.len() < 2 {
        eprintln!("ln: missing file operand");
        return ExitCode::FAILURE;
    }

    let mut exit_code = 0u8;

    let last = &config.targets[config.targets.len() - 1];
    let dest = Path::new(last);

    if config.targets.len() == 2 {
        // ln TARGET LINK_NAME
        let target = Path::new(&config.targets[0]);

        let link_path = if dest.is_dir() && (!config.no_deref || !dest.symlink_metadata().map(|m| m.is_symlink()).unwrap_or(false)) {
            dest.join(target.file_name().unwrap_or(target.as_ref()))
        } else {
            dest.to_path_buf()
        };

        if let Err(e) = create_link(target, &link_path, &config) {
            eprintln!("ln: {e}");
            exit_code = 1;
        }
    } else {
        // ln TARGET... DIRECTORY
        if !dest.is_dir() {
            eprintln!("ln: target '{}' is not a directory", dest.display());
            return ExitCode::FAILURE;
        }

        for t in &config.targets[..config.targets.len() - 1] {
            let target = Path::new(t);
            let link_path = dest.join(target.file_name().unwrap_or(target.as_ref()));

            if let Err(e) = create_link(target, &link_path, &config) {
                eprintln!("ln: {e}");
                exit_code = 1;
            }
        }
    }

    ExitCode::from(exit_code)
}
