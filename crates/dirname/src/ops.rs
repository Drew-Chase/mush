use std::path::Path;

pub fn dirname(name: &str) -> String {
    if name.is_empty() {
        return ".".to_string();
    }

    match Path::new(name).parent() {
        Some(p) if p.as_os_str().is_empty() => ".".to_string(),
        Some(p) => p.to_string_lossy().into_owned(),
        None => {
            // Root path like "/" returns itself
            name.to_string()
        }
    }
}
