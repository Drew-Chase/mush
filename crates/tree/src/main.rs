use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

use tree::cli::TreeConfig;
use tree::ops::print_tree;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let Some(config) = TreeConfig::from_args(&args) else {
        return ExitCode::SUCCESS;
    };

    let paths = if config.paths.is_empty() {
        vec![".".to_string()]
    } else {
        config.paths.clone()
    };

    let mut stdout = io::stdout().lock();
    let mut exit_code = ExitCode::SUCCESS;

    for dir in &paths {
        let path = Path::new(dir);
        if let Err(e) = print_tree(path, &config, &mut stdout) {
            eprintln!("tree: {dir}: {e}");
            exit_code = ExitCode::FAILURE;
        }
    }

    let _ = stdout.flush();
    exit_code
}
