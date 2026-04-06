use std::io;
use std::path::Path;

use cp::cli::CpConfig;
use cp::ops::copy_path;

fn main() {
    let _ = std::fs::create_dir_all("example_cp_output/src/sub");
    let _ = std::fs::write("example_cp_output/src/file.txt", "hello");
    let _ = std::fs::write("example_cp_output/src/sub/nested.txt", "world");

    let config = CpConfig {
        recursive: true,
        verbose: true,
        ..Default::default()
    };

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stderr = io::stderr();
    let mut writer = stderr.lock();

    if let Err(e) = copy_path(
        Path::new("example_cp_output/src"),
        Path::new("example_cp_output/dest"),
        &config,
        &mut reader,
        &mut writer,
    ) {
        eprintln!("error: {e}");
    }

    let _ = std::fs::remove_dir_all("example_cp_output");
}
