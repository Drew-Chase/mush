use std::fs::{self, Metadata};
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use glob::Pattern;
use regex::Regex;

use crate::cli::{Cmp, FileType, FindConfig, Predicate, SizeSpec};

/// Walk directories and collect matching paths.
pub fn find(config: &FindConfig) -> io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();

    for path_str in &config.paths {
        let path = Path::new(path_str);
        if !path.exists() {
            eprintln!("find: '{}': No such file or directory", path.display());
            continue;
        }
        walk_dir(path, 0, config, &mut results)?;
    }

    Ok(results)
}

/// Recursively walk a directory, evaluating predicates and collecting matches.
pub fn walk_dir(
    dir: &Path,
    depth: usize,
    config: &FindConfig,
    results: &mut Vec<PathBuf>,
) -> io::Result<()> {
    // Check max_depth before descending
    if let Some(max) = config.max_depth
        && depth > max
    {
        return Ok(());
    }

    // Check the directory entry itself
    let min_ok = config.min_depth.is_none() || depth >= config.min_depth.unwrap();

    if min_ok
        && let Ok(meta) = fs::symlink_metadata(dir)
        && matches_predicates(dir, &meta, &config.predicates)
    {
        results.push(dir.to_path_buf());
    }

    // If it's a directory, recurse into it
    if dir.is_dir() {
        if let Some(max) = config.max_depth
            && depth >= max
        {
            return Ok(());
        }

        let mut entries: Vec<_> = match fs::read_dir(dir) {
            Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
            Err(e) => {
                eprintln!("find: '{}': {}", dir.display(), e);
                return Ok(());
            }
        };
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let child = entry.path();
            let child_meta = match fs::symlink_metadata(&child) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("find: '{}': {}", child.display(), e);
                    continue;
                }
            };

            if child_meta.is_dir() {
                walk_dir(&child, depth + 1, config, results)?;
            } else {
                let child_depth = depth + 1;
                if let Some(max) = config.max_depth
                    && child_depth > max
                {
                    continue;
                }
                let child_min_ok =
                    config.min_depth.is_none() || child_depth >= config.min_depth.unwrap();
                if child_min_ok && matches_predicates(&child, &child_meta, &config.predicates) {
                    results.push(child);
                }
            }
        }
    }

    Ok(())
}

/// Evaluate all predicates against an entry. Predicates are AND-combined by default.
pub fn matches_predicates(entry: &Path, meta: &Metadata, predicates: &[Predicate]) -> bool {
    for pred in predicates {
        if !eval_predicate(entry, meta, pred) {
            return false;
        }
    }
    true
}

fn eval_predicate(entry: &Path, meta: &Metadata, pred: &Predicate) -> bool {
    match pred {
        Predicate::Name(pattern) => {
            let name = entry
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            match_glob(pattern, &name)
        }
        Predicate::IName(pattern) => {
            let name = entry
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let lower_pattern = pattern.to_lowercase();
            let lower_name = name.to_lowercase();
            match_glob(&lower_pattern, &lower_name)
        }
        Predicate::Type(ft) => match_type(meta, ft),
        Predicate::Size(spec) => match_size(meta, spec),
        Predicate::Empty => {
            if meta.is_file() {
                meta.len() == 0
            } else if meta.is_dir() {
                match fs::read_dir(entry) {
                    Ok(mut rd) => rd.next().is_none(),
                    Err(_) => false,
                }
            } else {
                false
            }
        }
        Predicate::Newer(ref_file) => {
            let ref_path = Path::new(ref_file);
            match (meta.modified(), fs::metadata(ref_path).and_then(|m| m.modified())) {
                (Ok(entry_time), Ok(ref_time)) => entry_time > ref_time,
                _ => false,
            }
        }
        Predicate::Path(pattern) => {
            let path_str = entry.to_string_lossy().to_string();
            match_glob(pattern, &path_str)
        }
        Predicate::Regex(pattern) => {
            let path_str = entry.to_string_lossy().to_string();
            match Regex::new(pattern) {
                Ok(re) => re.is_match(&path_str),
                Err(_) => false,
            }
        }
        Predicate::Not(inner) => !eval_predicate(entry, meta, inner),
        Predicate::And(left, right) => {
            eval_predicate(entry, meta, left) && eval_predicate(entry, meta, right)
        }
        Predicate::Or(left, right) => {
            eval_predicate(entry, meta, left) || eval_predicate(entry, meta, right)
        }
        Predicate::MaxDepth(_) | Predicate::MinDepth(_) => {
            // These are handled by the walker, not per-entry
            true
        }
        Predicate::Mtime(n) => {
            match meta.modified() {
                Ok(mtime) => {
                    let elapsed = SystemTime::now()
                        .duration_since(mtime)
                        .unwrap_or_default();
                    let days = (elapsed.as_secs() / 86400) as i64;
                    match_numeric_arg(*n, days)
                }
                Err(_) => false,
            }
        }
        Predicate::Mmin(n) => {
            match meta.modified() {
                Ok(mtime) => {
                    let elapsed = SystemTime::now()
                        .duration_since(mtime)
                        .unwrap_or_default();
                    let minutes = (elapsed.as_secs() / 60) as i64;
                    match_numeric_arg(*n, minutes)
                }
                Err(_) => false,
            }
        }
        Predicate::Perm(mode) => {
            // On Unix, check permission bits; on Windows, this is a no-op
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                (meta.mode() & 0o7777) == *mode
            }
            #[cfg(not(unix))]
            {
                let _ = mode;
                // Perm matching not supported on Windows
                true
            }
        }
    }
}

/// Match numeric arguments in find style: +N means > N, -N means < N, N means == N
fn match_numeric_arg(spec: i64, actual: i64) -> bool {
    if spec > 0 {
        // +N in the original string becomes a positive number
        actual > spec
    } else if spec < 0 {
        // -N in the original string becomes a negative number
        actual < spec.unsigned_abs() as i64
    } else {
        actual == 0
    }
}

/// Shell glob matching using glob::Pattern.
fn match_glob(pattern: &str, name: &str) -> bool {
    match Pattern::new(pattern) {
        Ok(pat) => pat.matches(name),
        Err(_) => false,
    }
}

/// Check if metadata matches the given file type.
fn match_type(meta: &Metadata, file_type: &FileType) -> bool {
    match file_type {
        FileType::File => meta.is_file(),
        FileType::Dir => meta.is_dir(),
        FileType::Symlink => meta.file_type().is_symlink(),
    }
}

/// Check if file size matches the specification.
fn match_size(meta: &Metadata, spec: &SizeSpec) -> bool {
    let size = meta.len();
    match spec.cmp {
        Cmp::Exact => size == spec.bytes,
        Cmp::GreaterThan => size > spec.bytes,
        Cmp::LessThan => size < spec.bytes,
    }
}
