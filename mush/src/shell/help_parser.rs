use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptionKind {
    Flag,
    Subcommand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandOption {
    pub name: String,
    pub description: Option<String>,
    pub kind: OptionKind,
}

pub fn parse_help_output(text: &str) -> Vec<CommandOption> {
    let mut options = Vec::new();
    let mut section = Section::Unknown;

    for line in text.lines() {
        let trimmed = line.trim_end();

        if is_section_header(trimmed) {
            section = classify_section(trimmed);
            continue;
        }

        match section {
            Section::Commands => {
                if let Some(opt) = parse_subcommand_line(trimmed) {
                    options.push(opt);
                }
            }
            Section::Options => {
                if let Some(opt) = parse_option_line(trimmed) {
                    options.push(opt);
                }
            }
            Section::Unknown => {
                if let Some(opt) = parse_option_line(trimmed) {
                    options.push(opt);
                }
            }
        }
    }

    options
}

#[derive(Clone, Copy)]
enum Section {
    Unknown,
    Commands,
    Options,
}

fn is_section_header(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.ends_with(':')
        && !trimmed.starts_with('-')
        && trimmed.len() < 40
        && line.len() - line.trim_start().len() < 4
}

fn classify_section(line: &str) -> Section {
    let lower = line.trim().trim_end_matches(':').to_lowercase();
    match lower.as_str() {
        "commands" | "subcommands" | "available commands" | "available subcommands"
        | "subcommand" => Section::Commands,
        "options" | "flags" | "global options" | "optional arguments" | "optional flags"
        | "global flags" | "arguments" => Section::Options,
        _ if lower.contains("command") || lower.contains("subcommand") => Section::Commands,
        _ if lower.contains("option") || lower.contains("flag") || lower.contains("argument") => {
            Section::Options
        }
        _ => Section::Unknown,
    }
}

fn parse_subcommand_line(line: &str) -> Option<CommandOption> {
    let trimmed = line.trim_start();
    if trimmed.is_empty() || trimmed.starts_with('-') {
        return None;
    }

    let indent = line.len() - trimmed.len();
    if indent < 2 {
        return None;
    }

    let (name_part, desc_part) = split_on_double_space(trimmed)?;

    // Handle comma-separated aliases like "build, b"
    let name = name_part
        .split(',')
        .next()
        .unwrap_or(name_part)
        .trim();

    if name.is_empty() || name.len() > 30 || name.contains(' ') {
        return None;
    }

    // Skip lines that look like continuation text (start with lowercase long phrase)
    if name.len() > 20 {
        return None;
    }

    let description = if desc_part.is_empty() {
        None
    } else {
        Some(desc_part.to_string())
    };

    Some(CommandOption {
        name: name.to_string(),
        description,
        kind: OptionKind::Subcommand,
    })
}

fn parse_option_line(line: &str) -> Option<CommandOption> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('-') {
        return None;
    }

    let (flag_part, desc_part) = match split_on_double_space(trimmed) {
        Some((f, d)) => (f, Some(d)),
        None => (trimmed, None),
    };

    let name = extract_flag_name(flag_part);
    if name.is_empty() {
        return None;
    }

    let description = desc_part
        .map(|d| d.to_string())
        .filter(|d| !d.is_empty());

    Some(CommandOption {
        name,
        description,
        kind: OptionKind::Flag,
    })
}

fn split_on_double_space(line: &str) -> Option<(&str, &str)> {
    if let Some(pos) = line.find("  ") {
        let left = line[..pos].trim();
        let right = line[pos..].trim();
        if left.is_empty() {
            None
        } else {
            Some((left, right))
        }
    } else {
        None
    }
}

fn extract_flag_name(flag_part: &str) -> String {
    // Given "-v, --verbose" or "--output <FILE>" or "--flag=VALUE"
    // Prefer the long form (--xxx)
    for part in flag_part.split(',').rev() {
        let token = part.split_whitespace().next().unwrap_or("");
        let token = token.split('=').next().unwrap_or(token);
        if token.starts_with("--") {
            return token.to_string();
        }
    }

    // No long form, use the short form
    let token = flag_part.split_whitespace().next().unwrap_or(flag_part);
    token.split('=').next().unwrap_or(token).split(',').next().unwrap_or(token).trim().to_string()
}
