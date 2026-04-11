use std::env;
use std::path::{Path, PathBuf};

use crate::cli::RealpathConfig;

pub fn resolve_path(path: &str, config: &RealpathConfig) -> Result<PathBuf, String> {
    let p = Path::new(path);

    if config.no_symlinks {
        // Don't resolve symlinks, just normalize the path
        let absolute = if p.is_absolute() {
            p.to_path_buf()
        } else {
            env::current_dir()
                .map_err(|e| format!("{e}"))?
                .join(p)
        };
        return Ok(normalize_path(&absolute));
    }

    if config.canonicalize_existing {
        // All components must exist
        std::fs::canonicalize(p).map_err(|e| format!("{path}: {e}"))
    } else if config.canonicalize_missing {
        // No components need exist - resolve what we can, normalize the rest
        resolve_missing(p)
    } else {
        // Default: all but the last component must exist
        std::fs::canonicalize(p)
            .or_else(|_| {
                // Try resolving parent, then append the last component
                if let Some(parent) = p.parent() {
                    let resolved_parent = if parent.as_os_str().is_empty() {
                        env::current_dir().map_err(|e| format!("{e}"))?
                    } else {
                        std::fs::canonicalize(parent)
                            .map_err(|e| format!("{path}: {e}"))?
                    };
                    if let Some(file_name) = p.file_name() {
                        Ok(resolved_parent.join(file_name))
                    } else {
                        Ok(resolved_parent)
                    }
                } else {
                    Err(format!("{path}: No such file or directory"))
                }
            })
    }
}

fn resolve_missing(p: &Path) -> Result<PathBuf, String> {
    let absolute = if p.is_absolute() {
        p.to_path_buf()
    } else {
        env::current_dir()
            .map_err(|e| format!("{e}"))?
            .join(p)
    };

    // Try to canonicalize; if that fails, normalize what we have
    match std::fs::canonicalize(&absolute) {
        Ok(resolved) => Ok(resolved),
        Err(_) => Ok(normalize_path(&absolute)),
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                if !components.is_empty() {
                    components.pop();
                }
            }
            std::path::Component::CurDir => {}
            other => components.push(other),
        }
    }

    if components.is_empty() {
        PathBuf::from("/")
    } else {
        components.iter().collect()
    }
}
