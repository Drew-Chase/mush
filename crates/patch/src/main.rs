use std::fs::File;
use std::io::{self, BufReader};
use std::process::ExitCode;

use patch::cli::PatchConfig;
use patch::ops::apply_patches_from_input;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = PatchConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let result = if let Some(ref path) = config.patch_file {
        match File::open(path) {
            Ok(f) => apply_patches_from_input(&mut BufReader::new(f), &config, &mut out),
            Err(e) => {
                eprintln!("patch: {path}: {e}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        let stdin = io::stdin();
        apply_patches_from_input(&mut stdin.lock(), &config, &mut out)
    };

    if let Err(e) = result {
        eprintln!("patch: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
