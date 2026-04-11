use std::fs;
use std::io;
use std::path::Path;

use crate::cli::DuConfig;

pub fn format_size(bytes: u64, config: &DuConfig) -> String {
    if config.bytes {
        return bytes.to_string();
    }
    if config.human_readable {
        return human_readable(bytes);
    }
    if config.megabytes {
        return (bytes / (1024 * 1024)).to_string();
    }
    if config.kilobytes {
        return (bytes / 1024).to_string();
    }
    // Default: 512-byte blocks
    bytes.div_ceil(512).to_string()
}

fn human_readable(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{bytes}");
    }
    let units = ['K', 'M', 'G', 'T', 'P'];
    let mut size = bytes as f64;
    for unit in &units {
        size /= 1024.0;
        if size < 1024.0 {
            if size < 10.0 {
                return format!("{:.1}{unit}", size);
            }
            return format!("{:.0}{unit}", size);
        }
    }
    let unit = units[units.len() - 1];
    format!("{:.0}{unit}", size)
}

pub fn du_path(
    path: &Path,
    config: &DuConfig,
    depth: usize,
    output: &mut Vec<(u64, String)>,
) -> io::Result<u64> {
    let meta = fs::metadata(path)?;

    if meta.is_file() {
        let size = meta.len();
        // Print file if: not summarize, and (all flag set), and within max_depth
        if !config.summarize && config.all && within_depth(depth, config) {
            output.push((size, path.to_string_lossy().to_string()));
        }
        return Ok(size);
    }

    if !meta.is_dir() {
        return Ok(0);
    }

    let mut total: u64 = 0;

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let child_path = entry.path();
        match du_path(&child_path, config, depth + 1, output) {
            Ok(size) => total += size,
            Err(e) => {
                eprintln!("du: cannot access '{}': {e}", child_path.display());
            }
        }
    }

    // Print directory if: not summarize, and within max_depth
    if !config.summarize && within_depth(depth, config) {
        output.push((total, path.to_string_lossy().to_string()));
    }

    Ok(total)
}

fn within_depth(depth: usize, config: &DuConfig) -> bool {
    match config.max_depth {
        Some(max) => depth <= max,
        None => true,
    }
}
