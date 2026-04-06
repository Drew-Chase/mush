use std::collections::HashSet;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;

use ls::cli::ResolvedConfig;
use ls::color::ColorScheme;
use ls::entry::{FileEntry, FileType};
use ls::{format, read, sort};

fn main() -> ExitCode {
    let cli = ls::cli::Cli::parse();
    let config = ResolvedConfig::from_cli(cli);
    let colors = ColorScheme::new(&config);
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut exit_code = 0u8;
    let multi = config.paths.len() > 1 || config.recursive;

    let mut files = Vec::new();
    let mut dirs = Vec::new();

    for path in &config.paths {
        match path.symlink_metadata() {
            Ok(meta) if meta.is_dir() && !config.directory_mode => {
                dirs.push(path.clone());
            }
            Ok(_) => match FileEntry::from_path(path.clone()) {
                Ok(entry) => files.push(entry),
                Err(e) => {
                    eprintln!("ls: cannot access '{}': {e}", path.display());
                    exit_code = 2;
                }
            },
            Err(e) => {
                eprintln!("ls: cannot access '{}': {e}", path.display());
                exit_code = 2;
            }
        }
    }

    if !files.is_empty() {
        sort::sort_entries(&mut files, &config);
        if let Err(e) = format::write_output(&files, &config, &colors, &mut out) {
            eprintln!("ls: write error: {e}");
            exit_code = 1;
        }
    }

    for (i, dir) in dirs.iter().enumerate() {
        if !files.is_empty() || i > 0 {
            let _ = writeln!(out);
        }
        if multi {
            let _ = writeln!(out, "{}:", dir.display());
        }

        exit_code = exit_code.max(list_directory(dir, &config, &colors, &mut out, multi));
    }

    let _ = out.flush();
    ExitCode::from(exit_code)
}

fn list_directory(
    dir: &Path,
    config: &ResolvedConfig,
    colors: &ColorScheme,
    out: &mut impl Write,
    _multi: bool,
) -> u8 {
    let mut exit_code = 0u8;

    let mut entries = match read::read_entries(dir, config) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("ls: cannot open directory '{}': {e}", dir.display());
            return 2;
        }
    };

    sort::sort_entries(&mut entries, config);

    if let Err(e) = format::write_output(&entries, config, colors, out) {
        eprintln!("ls: write error: {e}");
        exit_code = 1;
    }

    if config.recursive {
        let mut visited = HashSet::new();
        if let Ok(canonical) = dir.canonicalize() {
            visited.insert(canonical);
        }
        exit_code = exit_code.max(recurse(&entries, dir, config, colors, out, &mut visited));
    }

    exit_code
}

fn recurse(
    entries: &[FileEntry],
    _parent: &Path,
    config: &ResolvedConfig,
    colors: &ColorScheme,
    out: &mut impl Write,
    visited: &mut HashSet<PathBuf>,
) -> u8 {
    let mut exit_code = 0u8;

    let subdirs: Vec<&FileEntry> = entries
        .iter()
        .filter(|e| e.file_type == FileType::Directory && e.name != "." && e.name != "..")
        .collect();

    for entry in subdirs {
        if let Ok(canonical) = entry.path.canonicalize()
            && !visited.insert(canonical)
        {
            continue;
        }

        let _ = writeln!(out);
        let _ = writeln!(out, "{}:", entry.path.display());
        exit_code = exit_code.max(list_directory(&entry.path, config, colors, out, true));
    }

    exit_code
}
