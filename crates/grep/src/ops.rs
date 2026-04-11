use std::collections::VecDeque;
use std::fs;
use std::io::{BufRead, Write};
use std::path::Path;

use regex::Regex;

use crate::cli::GrepConfig;

#[derive(Debug, Default)]
pub struct GrepResult {
    pub match_count: usize,
    pub had_match: bool,
}

impl GrepResult {
    pub fn merge(&mut self, other: &GrepResult) {
        self.match_count += other.match_count;
        if other.had_match {
            self.had_match = true;
        }
    }
}

/// Build the regex from the config pattern.
pub fn build_regex(config: &GrepConfig) -> Result<Regex, regex::Error> {
    let pat = if config.fixed_strings {
        regex::escape(&config.pattern)
    } else {
        config.pattern.clone()
    };

    let pat = if config.word_regexp {
        format!(r"\b(?:{pat})\b")
    } else if config.line_regexp {
        format!("^(?:{pat})$")
    } else {
        pat
    };

    regex::RegexBuilder::new(&pat)
        .case_insensitive(config.ignore_case)
        .build()
}

/// Search a reader for matching lines.
pub fn grep_reader(
    input: &mut dyn BufRead,
    filename: Option<&str>,
    config: &GrepConfig,
    re: &Regex,
    writer: &mut dyn Write,
) -> GrepResult {
    let mut result = GrepResult::default();
    let mut line_num: usize = 0;
    let mut match_count: usize = 0;

    let before_size = if config.context > 0 {
        config.context
    } else {
        config.before_context
    };
    let after_size = if config.context > 0 {
        config.context
    } else {
        config.after_context
    };
    let use_context = before_size > 0 || after_size > 0;

    let mut before_buf: VecDeque<(usize, String)> = VecDeque::new();
    let mut after_countdown: usize = 0;
    let mut printed_any = false;

    let show_filename = if config.no_filename {
        false
    } else if config.with_filename {
        true
    } else {
        filename.is_some() && filename != Some("-")
    };

    let mut line = String::new();

    loop {
        line.clear();
        match input.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }

        let line_content = line.trim_end_matches(['\n', '\r']);
        line_num += 1;

        let is_match = re.is_match(line_content);
        let selected = if config.invert { !is_match } else { is_match };

        if selected {
            if let Some(max) = config.max_count
                && match_count >= max
            {
                break;
            }

            match_count += 1;
            result.had_match = true;

            if config.quiet {
                result.match_count = match_count;
                return result;
            }

            if config.count || config.files_with_matches || config.files_without_match {
                // defer output
            } else {
                // Print separator between context groups
                if use_context && printed_any && after_countdown == 0 && !before_buf.is_empty() {
                    // Check if the before_buf is contiguous with previous output
                    let _ = writeln!(writer, "--");
                }

                // Print before-context lines
                for (bnum, bline) in &before_buf {
                    print_line(
                        writer,
                        filename,
                        show_filename,
                        config,
                        *bnum,
                        bline,
                        '-',
                        re,
                    );
                }
                before_buf.clear();

                if config.only_matching && !config.invert {
                    // Print only the matched parts
                    for mat in re.find_iter(line_content) {
                        let matched = mat.as_str();
                        print_match_only(
                            writer,
                            filename,
                            show_filename,
                            config,
                            line_num,
                            matched,
                        );
                    }
                } else {
                    print_line(
                        writer,
                        filename,
                        show_filename,
                        config,
                        line_num,
                        line_content,
                        ':',
                        re,
                    );
                }

                printed_any = true;
                after_countdown = after_size;
            }
        } else if use_context && !config.count && !config.files_with_matches && !config.files_without_match && !config.quiet {
            if after_countdown > 0 {
                print_line(
                    writer,
                    filename,
                    show_filename,
                    config,
                    line_num,
                    line_content,
                    '-',
                    re,
                );
                after_countdown -= 1;
            } else {
                before_buf.push_back((line_num, line_content.to_string()));
                if before_buf.len() > before_size {
                    before_buf.pop_front();
                }
            }
        } else {
            after_countdown = after_countdown.saturating_sub(1);
        }
    }

    result.match_count = match_count;

    if config.quiet {
        return result;
    }

    if config.files_with_matches {
        if result.had_match {
            if let Some(name) = filename {
                let _ = writeln!(writer, "{name}");
            } else {
                let _ = writeln!(writer, "(standard input)");
            }
        }
    } else if config.files_without_match {
        if !result.had_match {
            if let Some(name) = filename {
                let _ = writeln!(writer, "{name}");
            } else {
                let _ = writeln!(writer, "(standard input)");
            }
        }
    } else if config.count {
        if show_filename {
            if let Some(name) = filename {
                let _ = writeln!(writer, "{name}:{match_count}");
            } else {
                let _ = writeln!(writer, "{match_count}");
            }
        } else {
            let _ = writeln!(writer, "{match_count}");
        }
    }

    result
}

#[allow(clippy::too_many_arguments)]
fn print_line(
    writer: &mut dyn Write,
    filename: Option<&str>,
    show_filename: bool,
    config: &GrepConfig,
    line_num: usize,
    line_content: &str,
    sep: char,
    re: &Regex,
) {
    let mut prefix = String::new();
    if show_filename
        && let Some(name) = filename
    {
        prefix.push_str(name);
        prefix.push(sep);
    }
    if config.line_number {
        prefix.push_str(&line_num.to_string());
        prefix.push(sep);
    }

    if config.color && !config.invert {
        let colored = colorize_matches(re, line_content);
        let _ = writeln!(writer, "{prefix}{colored}");
    } else {
        let _ = writeln!(writer, "{prefix}{line_content}");
    }
}

fn print_match_only(
    writer: &mut dyn Write,
    filename: Option<&str>,
    show_filename: bool,
    config: &GrepConfig,
    line_num: usize,
    matched: &str,
) {
    let mut prefix = String::new();
    if show_filename
        && let Some(name) = filename
    {
        prefix.push_str(name);
        prefix.push(':');
    }
    if config.line_number {
        prefix.push_str(&line_num.to_string());
        prefix.push(':');
    }

    if config.color {
        let _ = writeln!(writer, "{prefix}\x1b[1;31m{matched}\x1b[0m");
    } else {
        let _ = writeln!(writer, "{prefix}{matched}");
    }
}

fn colorize_matches(re: &Regex, line: &str) -> String {
    let mut result = String::new();
    let mut last_end = 0;

    for mat in re.find_iter(line) {
        result.push_str(&line[last_end..mat.start()]);
        result.push_str("\x1b[1;31m");
        result.push_str(mat.as_str());
        result.push_str("\x1b[0m");
        last_end = mat.end();
    }

    result.push_str(&line[last_end..]);
    result
}

/// Recursively search a directory.
pub fn grep_recursive(
    dir: &Path,
    config: &GrepConfig,
    re: &Regex,
    writer: &mut dyn Write,
) -> GrepResult {
    let mut result = GrepResult::default();

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("grep: {}: {e}", dir.display());
            return result;
        }
    };

    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();

        if path.is_dir() {
            let dir_name = entry.file_name();
            let dir_name = dir_name.to_string_lossy();

            if config.exclude_dir.iter().any(|pat| glob_match(pat, &dir_name)) {
                continue;
            }

            let sub = grep_recursive(&path, config, re, writer);
            result.merge(&sub);
        } else if path.is_file() {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();

            if !config.include_glob.is_empty()
                && !config.include_glob.iter().any(|pat| glob_match(pat, &file_name))
            {
                continue;
            }

            if config.exclude_glob.iter().any(|pat| glob_match(pat, &file_name)) {
                continue;
            }

            let file = match fs::File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("grep: {}: {e}", path.display());
                    continue;
                }
            };

            let mut reader = std::io::BufReader::new(file);
            let path_str = path.to_string_lossy().to_string();
            let sub = grep_reader(&mut reader, Some(&path_str), config, re, writer);
            result.merge(&sub);
        }
    }

    result
}

/// Simple glob matching supporting * and ? wildcards.
fn glob_match(pattern: &str, name: &str) -> bool {
    glob_match_inner(pattern.as_bytes(), name.as_bytes())
}

fn glob_match_inner(pat: &[u8], name: &[u8]) -> bool {
    let mut pi = 0;
    let mut ni = 0;
    let mut star_pi = usize::MAX;
    let mut star_ni = 0;

    while ni < name.len() {
        if pi < pat.len() && (pat[pi] == b'?' || pat[pi] == name[ni]) {
            pi += 1;
            ni += 1;
        } else if pi < pat.len() && pat[pi] == b'*' {
            star_pi = pi;
            star_ni = ni;
            pi += 1;
        } else if star_pi != usize::MAX {
            pi = star_pi + 1;
            star_ni += 1;
            ni = star_ni;
        } else {
            return false;
        }
    }

    while pi < pat.len() && pat[pi] == b'*' {
        pi += 1;
    }

    pi == pat.len()
}
