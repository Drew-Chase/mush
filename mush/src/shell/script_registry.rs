use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use serde::Deserialize;

#[derive(Deserialize)]
struct PackageJson {
    name: Option<String>,
    description: Option<String>,
    main: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ScriptEntry {
    pub name: String,
    pub description: String,
    pub entry_point: PathBuf,
    pub script_dir: PathBuf,
}

static SCRIPT_REGISTRY: OnceLock<Mutex<Vec<ScriptEntry>>> = OnceLock::new();

fn registry() -> &'static Mutex<Vec<ScriptEntry>> {
    SCRIPT_REGISTRY.get_or_init(|| Mutex::new(Vec::new()))
}

/// Scans `scripts_dir` for subdirectories containing a `package.json`.
/// Each valid entry is registered by its package name.
pub fn scan_scripts(scripts_dir: &Path) {
    let entries = match std::fs::read_dir(scripts_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    let mut scripts = Vec::new();

    for entry in entries.flatten() {
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }

        let pkg_path = dir.join("package.json");
        if !pkg_path.is_file() {
            continue;
        }

        let content = match std::fs::read_to_string(&pkg_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let pkg: PackageJson = match serde_json::from_str(&content) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let name = match pkg.name {
            Some(n) if !n.is_empty() => n,
            _ => continue,
        };

        let description = pkg.description.unwrap_or_default();

        let entry_point = match pkg.main {
            Some(ref m) if !m.is_empty() => dir.join(m),
            _ => dir.join("index.ts"),
        };

        // Guard against path traversal: ensure entry_point stays within script dir
        if let Ok(canonical) = entry_point.canonicalize()
            && let Ok(canonical_dir) = dir.canonicalize()
            && !canonical.starts_with(&canonical_dir)
        {
            continue;
        }

        scripts.push(ScriptEntry {
            name,
            description,
            entry_point,
            script_dir: dir,
        });
    }

    if let Ok(mut reg) = registry().lock() {
        *reg = scripts;
    }
}

/// Finds a script by name (exact match).
pub fn find_script(name: &str) -> Option<ScriptEntry> {
    let reg = registry().lock().ok()?;
    reg.iter().find(|s| s.name == name).cloned()
}

/// Returns all registered scripts for autocomplete.
pub fn list_scripts() -> Vec<ScriptEntry> {
    registry()
        .lock()
        .ok()
        .map(|r| r.clone())
        .unwrap_or_default()
}

/// Quick check whether `name` matches a registered script.
pub fn is_script(name: &str) -> bool {
    registry()
        .lock()
        .ok()
        .is_some_and(|r| r.iter().any(|s| s.name == name))
}
