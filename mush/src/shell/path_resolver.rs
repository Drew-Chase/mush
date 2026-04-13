use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock, RwLock};

static PATH_CACHE: OnceLock<Mutex<HashMap<String, Option<PathBuf>>>> = OnceLock::new();
static EXECUTABLE_NAMES: OnceLock<RwLock<Vec<String>>> = OnceLock::new();

fn cache() -> &'static Mutex<HashMap<String, Option<PathBuf>>> {
    PATH_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn invalidate_cache() {
    if let Some(c) = PATH_CACHE.get()
        && let Ok(mut map) = c.lock()
    {
        map.clear();
    }
}

pub fn is_executable(name: &str) -> bool {
    find_in_path(name).is_some()
}

pub fn find_in_path(name: &str) -> Option<PathBuf> {
    // On Windows the filesystem is case-insensitive, so normalise the key.
    // On Unix keep the original case to avoid confusing Foo and foo.
    let key = if cfg!(windows) {
        name.to_lowercase()
    } else {
        name.to_string()
    };

    // Check cache first
    if let Ok(map) = cache().lock()
        && let Some(cached) = map.get(&key)
    {
        return cached.clone();
    }

    let result = search_path(name);

    // Store in cache
    if let Ok(mut map) = cache().lock() {
        map.insert(key, result.clone());
    }

    result
}

fn search_path(name: &str) -> Option<PathBuf> {
    let path_var = std::env::var("PATH").ok()?;
    let dirs = std::env::split_paths(&path_var);

    // Determine which extensions to try
    let extensions = get_extensions(name);

    for dir in dirs {
        if extensions.is_empty() {
            // Name already has an extension or we're on Unix
            let candidate = dir.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        } else {
            for ext in &extensions {
                let candidate = dir.join(format!("{name}{ext}"));
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
            // Also try the bare name
            let candidate = dir.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    None
}

/// Returns the list of extensions to try when searching for an executable.
/// On Windows, uses PATHEXT; on other platforms, returns empty (no extensions needed).
fn get_extensions(name: &str) -> Vec<String> {
    // If the file name portion already has an extension, don't append more.
    // We check only the file name to avoid false positives from path separators
    // like the `.` in `./program`.
    let file_name = std::path::Path::new(name)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or(name);
    if file_name.contains('.') {
        return Vec::new();
    }

    #[cfg(windows)]
    {
        if let Ok(pathext) = std::env::var("PATHEXT") {
            pathext
                .split(';')
                .map(|s| s.to_lowercase())
                .collect()
        } else {
            vec![
                ".exe".to_string(),
                ".cmd".to_string(),
                ".bat".to_string(),
                ".com".to_string(),
                ".ps1".to_string(),
            ]
        }
    }

    #[cfg(not(windows))]
    {
        Vec::new()
    }
}

/// Scans all PATH directories and returns a sorted list of executable names.
/// This performs filesystem I/O and may be slow — intended to run on a background thread.
pub fn scan_path_executables() -> Vec<String> {
    let mut names = HashSet::new();
    let path_var = match std::env::var("PATH") {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let extensions = {
        #[cfg(windows)]
        {
            if let Ok(pathext) = std::env::var("PATHEXT") {
                pathext
                    .split(';')
                    .map(|s| s.to_lowercase())
                    .collect::<Vec<_>>()
            } else {
                vec![
                    ".exe".to_string(),
                    ".cmd".to_string(),
                    ".bat".to_string(),
                    ".com".to_string(),
                    ".ps1".to_string(),
                ]
            }
        }
        #[cfg(not(windows))]
        {
            Vec::<String>::new()
        }
    };

    for dir in std::env::split_paths(&path_var) {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if extensions.is_empty() {
                names.insert(file_name);
            } else {
                let lower = file_name.to_lowercase();
                if extensions.iter().any(|ext| lower.ends_with(ext.as_str())) {
                    names.insert(file_name);
                }
            }
        }
    }

    let mut sorted: Vec<String> = names.into_iter().collect();
    sorted.sort();
    sorted
}

/// Returns the current list of PATH executables.
/// On first call, performs a synchronous scan. Subsequent calls return the cached list.
pub fn list_executables() -> Vec<String> {
    let lock = EXECUTABLE_NAMES.get_or_init(|| RwLock::new(scan_path_executables()));
    match lock.read() {
        Ok(guard) => guard.clone(),
        Err(_) => Vec::new(),
    }
}

/// Replaces the cached list of PATH executables with a fresh scan result.
pub fn replace_executables(new_list: Vec<String>) {
    let lock = EXECUTABLE_NAMES.get_or_init(|| RwLock::new(Vec::new()));
    if let Ok(mut guard) = lock.write() {
        *guard = new_list;
    }
}

/// Returns `true` if the command name contains a path separator (`/` or `\`),
/// indicating it should be resolved directly as a filesystem path.
pub fn has_path_separator(name: &str) -> bool {
    name.contains('/') || name.contains('\\')
}

/// Resolves a command name that contains a path separator (e.g. `./foo`,
/// `../bar`, `/absolute/path`, `folder\program`) to an actual file path.
///
/// Relative paths are joined with the current working directory.
/// On Windows, tries PATHEXT extensions if the name has no extension.
/// Results are NOT cached because the working directory can change.
pub fn resolve_direct_path(name: &str) -> Option<PathBuf> {
    let path = PathBuf::from(name);

    let absolute = if path.is_relative() {
        std::env::current_dir().ok()?.join(&path)
    } else {
        path
    };

    if absolute.is_file() {
        return Some(absolute);
    }

    // On Windows, try PATHEXT extensions if name has no file extension
    for ext in &get_extensions(name) {
        let candidate = absolute.with_file_name(format!(
            "{}{}",
            absolute.file_name()?.to_str()?,
            ext
        ));
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    None
}
