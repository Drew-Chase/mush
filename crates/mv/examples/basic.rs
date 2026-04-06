use std::io;
use std::path::Path;

use mv::cli::MvConfig;
use mv::ops::move_path;

fn main() {
    let _ = std::fs::create_dir_all("example_mv_output/src");
    let _ = std::fs::write("example_mv_output/src/file.txt", "hello");

    let config = MvConfig {
        verbose: true,
        ..Default::default()
    };

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stderr = io::stderr();
    let mut writer = stderr.lock();

    if let Err(e) = move_path(
        Path::new("example_mv_output/src/file.txt"),
        Path::new("example_mv_output/moved.txt"),
        &config,
        &mut reader,
        &mut writer,
    ) {
        eprintln!("error: {e}");
    }

    let _ = std::fs::remove_dir_all("example_mv_output");
}
