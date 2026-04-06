use std::io;
use std::path::Path;

use rm::cli::RmConfig;
use rm::ops::remove_path;

fn main() {
    let _ = std::fs::create_dir_all("example_rm_output/sub");
    let _ = std::fs::write("example_rm_output/file.txt", "hello");
    let _ = std::fs::write("example_rm_output/sub/nested.txt", "world");

    let config = RmConfig {
        recursive: true,
        verbose: true,
        force: true,
        ..Default::default()
    };

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stderr = io::stderr();
    let mut writer = stderr.lock();

    if let Err(e) = remove_path(
        Path::new("example_rm_output"),
        &config,
        &mut reader,
        &mut writer,
    ) {
        eprintln!("error: {e}");
    }
}
