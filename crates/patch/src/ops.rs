use std::fs;
use std::io::{BufRead, Write};
use std::path::PathBuf;

use crate::cli::PatchConfig;

#[derive(Debug, Clone)]
pub struct Hunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone)]
pub enum DiffLine {
    Context(String),
    Add(String),
    Remove(String),
}

#[derive(Debug, Clone)]
pub struct PatchFile {
    pub old_file: String,
    pub new_file: String,
    pub hunks: Vec<Hunk>,
}

pub fn parse_patch(input: &mut dyn BufRead) -> std::io::Result<Vec<PatchFile>> {
    let mut patches = Vec::new();
    let mut lines = Vec::new();
    let mut line = String::new();

    loop {
        line.clear();
        if input.read_line(&mut line)? == 0 {
            break;
        }
        lines.push(line.trim_end_matches('\n').trim_end_matches('\r').to_string());
    }

    let mut i = 0;
    while i < lines.len() {
        // Look for --- header
        if lines[i].starts_with("--- ") && i + 1 < lines.len() && lines[i + 1].starts_with("+++ ") {
            let old_file = lines[i].strip_prefix("--- ").unwrap_or("").to_string();
            let new_file = lines[i + 1].strip_prefix("+++ ").unwrap_or("").to_string();
            // Strip tab-separated timestamps if present
            let old_file = old_file.split('\t').next().unwrap_or("").to_string();
            let new_file = new_file.split('\t').next().unwrap_or("").to_string();
            i += 2;

            let mut hunks = Vec::new();
            while i < lines.len() && lines[i].starts_with("@@ ") {
                if let Some(hunk) = parse_hunk_header(&lines[i]) {
                    i += 1;
                    let mut diff_lines = Vec::new();
                    while i < lines.len()
                        && !lines[i].starts_with("@@ ")
                        && !lines[i].starts_with("--- ")
                    {
                        let l = &lines[i];
                        if let Some(rest) = l.strip_prefix('+') {
                            diff_lines.push(DiffLine::Add(rest.to_string()));
                        } else if let Some(rest) = l.strip_prefix('-') {
                            diff_lines.push(DiffLine::Remove(rest.to_string()));
                        } else if let Some(rest) = l.strip_prefix(' ') {
                            diff_lines.push(DiffLine::Context(rest.to_string()));
                        } else if l == "\\ No newline at end of file" {
                            // skip
                        } else if l.is_empty() {
                            // empty context line
                            diff_lines.push(DiffLine::Context(String::new()));
                        } else {
                            break;
                        }
                        i += 1;
                    }
                    hunks.push(Hunk {
                        old_start: hunk.0,
                        old_count: hunk.1,
                        new_start: hunk.2,
                        new_count: hunk.3,
                        lines: diff_lines,
                    });
                } else {
                    i += 1;
                }
            }

            patches.push(PatchFile {
                old_file,
                new_file,
                hunks,
            });
        } else {
            i += 1;
        }
    }

    Ok(patches)
}

fn parse_hunk_header(line: &str) -> Option<(usize, usize, usize, usize)> {
    // @@ -old_start,old_count +new_start,new_count @@
    let line = line.strip_prefix("@@ ")?;
    let line = line.split(" @@").next()?;
    let parts: Vec<&str> = line.split(' ').collect();
    if parts.len() < 2 {
        return None;
    }

    let old_part = parts[0].strip_prefix('-')?;
    let new_part = parts[1].strip_prefix('+')?;

    let (old_start, old_count) = parse_range(old_part);
    let (new_start, new_count) = parse_range(new_part);

    Some((old_start, old_count, new_start, new_count))
}

fn parse_range(s: &str) -> (usize, usize) {
    if let Some((start, count)) = s.split_once(',') {
        (
            start.parse().unwrap_or(1),
            count.parse().unwrap_or(0),
        )
    } else {
        (s.parse().unwrap_or(1), 1)
    }
}

pub fn strip_path(path: &str, strip: usize) -> PathBuf {
    let components: Vec<&str> = path.split('/').collect();
    if strip >= components.len() {
        PathBuf::from(components.last().unwrap_or(&""))
    } else {
        PathBuf::from(components[strip..].join("/"))
    }
}

pub fn apply_hunk(original_lines: &[String], hunk: &Hunk, reverse: bool) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    let start = if hunk.old_start == 0 { 0 } else { hunk.old_start - 1 };
    let mut orig_idx = 0;

    // Copy lines before the hunk
    while orig_idx < start && orig_idx < original_lines.len() {
        result.push(original_lines[orig_idx].clone());
        orig_idx += 1;
    }

    // Apply the hunk
    for diff_line in &hunk.lines {
        match diff_line {
            DiffLine::Context(text) => {
                if orig_idx < original_lines.len() {
                    // Verify context matches
                    if original_lines[orig_idx] != *text {
                        return Err(format!(
                            "context mismatch at line {}: expected '{}', got '{}'",
                            orig_idx + 1,
                            text,
                            original_lines[orig_idx]
                        ));
                    }
                    result.push(original_lines[orig_idx].clone());
                    orig_idx += 1;
                } else {
                    result.push(text.clone());
                }
            }
            DiffLine::Remove(text) => {
                if reverse {
                    result.push(text.clone());
                } else {
                    if orig_idx < original_lines.len() && original_lines[orig_idx] != *text {
                        return Err(format!(
                            "remove mismatch at line {}: expected '{}', got '{}'",
                            orig_idx + 1,
                            text,
                            original_lines[orig_idx]
                        ));
                    }
                    orig_idx += 1; // skip the removed line
                }
            }
            DiffLine::Add(text) => {
                if reverse {
                    if orig_idx < original_lines.len() && original_lines[orig_idx] != *text {
                        return Err(format!(
                            "reverse remove mismatch at line {}: expected '{}', got '{}'",
                            orig_idx + 1,
                            text,
                            original_lines[orig_idx]
                        ));
                    }
                    orig_idx += 1;
                } else {
                    result.push(text.clone());
                }
            }
        }
    }

    // Copy remaining lines
    while orig_idx < original_lines.len() {
        result.push(original_lines[orig_idx].clone());
        orig_idx += 1;
    }

    Ok(result)
}

pub fn apply_patch(
    patch_file: &PatchFile,
    config: &PatchConfig,
    output: &mut dyn Write,
) -> Result<(), String> {
    let target_path = if config.reverse {
        strip_path(&patch_file.old_file, config.strip)
    } else {
        strip_path(&patch_file.new_file, config.strip)
    };

    // If original_file is specified, use it instead
    let source_path = if let Some(ref orig) = config.original_file {
        PathBuf::from(orig)
    } else if config.reverse {
        strip_path(&patch_file.new_file, config.strip)
    } else {
        strip_path(&patch_file.old_file, config.strip)
    };

    let content = fs::read_to_string(&source_path)
        .map_err(|e| format!("cannot open {}: {e}", source_path.display()))?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    for hunk in &patch_file.hunks {
        lines = apply_hunk(&lines, hunk, config.reverse)?;
    }

    let new_content = lines.join("\n") + if content.ends_with('\n') { "\n" } else { "" };

    if config.dry_run {
        writeln!(output, "patching file {} (dry run)", target_path.display())
            .map_err(|e| e.to_string())?;
        return Ok(());
    }

    if config.backup {
        let backup_path = format!("{}.orig", source_path.display());
        fs::copy(&source_path, &backup_path)
            .map_err(|e| format!("cannot create backup {backup_path}: {e}"))?;
    }

    fs::write(&target_path, &new_content)
        .map_err(|e| format!("cannot write {}: {e}", target_path.display()))?;
    writeln!(output, "patching file {}", target_path.display())
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn apply_patches_from_input(
    input: &mut dyn BufRead,
    config: &PatchConfig,
    output: &mut dyn Write,
) -> Result<(), String> {
    let patches = parse_patch(input).map_err(|e| e.to_string())?;
    if patches.is_empty() {
        return Err("no patch data found".to_string());
    }
    for patch_file in &patches {
        apply_patch(patch_file, config, output)?;
    }
    Ok(())
}

/// Apply a hunk to content provided as a string (useful for testing without files).
pub fn apply_patch_to_string(
    original: &str,
    patch_text: &str,
    reverse: bool,
) -> Result<String, String> {
    let mut cursor = std::io::Cursor::new(patch_text.as_bytes());
    let patches = parse_patch(&mut cursor).map_err(|e| e.to_string())?;
    if patches.is_empty() {
        return Err("no patch data found".to_string());
    }

    let mut lines: Vec<String> = original.lines().map(String::from).collect();
    for patch_file in &patches {
        for hunk in &patch_file.hunks {
            lines = apply_hunk(&lines, hunk, reverse)?;
        }
    }

    Ok(lines.join("\n"))
}
