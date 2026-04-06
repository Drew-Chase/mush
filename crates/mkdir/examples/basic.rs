use std::path::Path;

use mkdir::cli::MkdirConfig;
use mkdir::ops::create_directory;

fn main() {
    let config = MkdirConfig {
        parents: true,
        verbose: true,
        ..Default::default()
    };

    let dirs = ["example_output/a/b", "example_output/c"];
    for dir in &dirs {
        if let Err(e) = create_directory(Path::new(dir), &config) {
            eprintln!("error: {e}");
        }
    }

    // Cleanup
    let _ = std::fs::remove_dir_all("example_output");
}
