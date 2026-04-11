use std::path::Path;

pub fn basename(name: &str, suffix: Option<&str>) -> String {
    if name.is_empty() {
        return String::new();
    }

    let file_name = Path::new(name)
        .file_name()
        .map(|f| f.to_string_lossy().into_owned())
        .unwrap_or_else(|| {
            // For paths like "/" or "//", file_name() returns None
            // In that case basename should return "/"
            let trimmed = name.trim_end_matches('/');
            if trimmed.is_empty() {
                "/".to_string()
            } else {
                Path::new(trimmed)
                    .file_name()
                    .map(|f| f.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "/".to_string())
            }
        });

    if let Some(suffix) = suffix
        && !suffix.is_empty()
        && file_name.len() > suffix.len()
        && file_name.ends_with(suffix)
    {
        return file_name[..file_name.len() - suffix.len()].to_string();
    }

    file_name
}
