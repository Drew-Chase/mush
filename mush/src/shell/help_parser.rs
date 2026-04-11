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
    #[serde(default)]
    pub args: Option<String>,
    #[serde(default)]
    pub default_value: Option<String>,
    #[serde(default)]
    pub possible_values: Option<Vec<String>>,
}

pub fn parse_help_output(text: &str) -> Vec<CommandOption> {
    let mut options = Vec::new();
    let mut section = Section::Unknown;

    for line in text.lines() {
        let trimmed = line.trim_end();

        // Check for inline usage pattern: "Usage: cmd [OPTIONS] ..."
        if section == Section::Unknown && is_usage_line(trimmed) {
            options.extend(parse_usage_line(trimmed));
            continue;
        }

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
            Section::HelpInfo => {
                if let Some(opt) = parse_help_info_line(trimmed) {
                    options.push(opt);
                }
            }
            Section::Usage => {
                options.extend(parse_usage_line(trimmed));
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    Unknown,
    Commands,
    Options,
    HelpInfo,
    Usage,
}

fn is_section_header(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.ends_with(':')
        && !trimmed.starts_with('-')
        && trimmed.len() < 80
        && line.len() - line.trim_start().len() < 4
}

fn classify_section(line: &str) -> Section {
    let lower = line.trim().trim_end_matches(':').to_lowercase();
    match lower.as_str() {
        "commands" | "subcommands" | "available commands" | "available subcommands"
        | "subcommand" => Section::Commands,
        "options" | "flags" | "global options" | "optional arguments" | "optional flags"
        | "global flags" | "arguments" => Section::Options,
        "usage" => Section::Usage,
        _ if lower.contains("command") || lower.contains("subcommand") => Section::Commands,
        _ if lower.contains("help") || lower.contains("information") || lower.contains("capabilities") => {
            Section::HelpInfo
        }
        _ if lower.contains("option") || lower.contains("flag") || lower.contains("argument") => {
            Section::Options
        }
        _ => Section::Unknown,
    }
}

fn is_usage_line(line: &str) -> bool {
    let trimmed = line.trim();
    let lower = trimmed.to_lowercase();
    lower.starts_with("usage:")
}

fn parse_usage_line(line: &str) -> Vec<CommandOption> {
    let trimmed = line.trim();
    // Strip "Usage:" or "usage:" prefix
    let rest = if let Some(r) = trimmed.strip_prefix("Usage:") {
        r
    } else if let Some(r) = trimmed.strip_prefix("usage:") {
        r
    } else {
        trimmed
    };

    let mut results = Vec::new();
    for token in rest.split_whitespace() {
        let lower = token.to_lowercase();
        // Skip the command name, [OPTIONS], and bare flags
        if lower == "[options]" || lower == "[option]" || token.starts_with('-') {
            continue;
        }
        // Positional args: <FILE>, [NEW_VERSION], etc.
        if (token.starts_with('<') && token.ends_with('>'))
            || (token.starts_with('[') && token.ends_with(']'))
        {
            let is_required = token.starts_with('<');
            let desc = if is_required {
                "required positional argument"
            } else {
                "optional positional argument"
            };
            results.push(CommandOption {
                name: token.to_string(),
                description: Some(desc.to_string()),
                kind: OptionKind::Subcommand,
                args: None,
                default_value: None,
                possible_values: None,
            });
        }
    }
    results
}

/// Parse lines in "Getting help:" / "Print help / information / capabilities:" sections.
/// These often use ` -- ` as separator (ffmpeg style) or standard double-space.
fn parse_help_info_line(line: &str) -> Option<CommandOption> {
    let trimmed = line.trim_start();
    if trimmed.is_empty() {
        return None;
    }

    // Skip prose lines that don't look like options
    if !trimmed.starts_with('-') {
        return None;
    }

    // Try ` -- ` separator first (ffmpeg style: "-h long  -- print more options")
    let (flag_part, desc_part) = if let Some(pos) = trimmed.find(" -- ") {
        (&trimmed[..pos], Some(trimmed[pos + 4..].trim()))
    } else {
        match split_on_double_space(trimmed) {
            Some((f, d)) => (f, Some(d)),
            None => (trimmed, None),
        }
    };

    // Keep the full flag_part as name (e.g. "-h long", "-h full", "-h type=name")
    let name = flag_part.trim().to_string();
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
        args: None,
        default_value: None,
        possible_values: None,
    })
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
        args: None,
        default_value: None,
        possible_values: None,
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

    let (name, args) = extract_flag_and_args(flag_part);
    if name.is_empty() {
        return None;
    }

    let raw_desc = desc_part.map(|d| d.to_string()).filter(|d| !d.is_empty());

    let (description, default_value, possible_values) = match raw_desc {
        Some(d) => {
            let (desc, def, vals) = extract_metadata(&d);
            (Some(desc), def, vals)
        }
        None => (None, None, None),
    };

    Some(CommandOption {
        name,
        description,
        kind: OptionKind::Flag,
        args,
        default_value,
        possible_values,
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

/// Extracts the flag name and optional argument annotation from a flag part.
///
/// Examples:
/// - `-f <fmt>` → (`-f`, Some(`<fmt>`))
/// - `-t, --types <SUPPORTED_TYPES>` → (`--types`, Some(`<SUPPORTED_TYPES>`))
/// - `--flag=VALUE` → (`--flag`, Some(`VALUE`))
/// - `-v, --verbose` → (`--verbose`, None)
/// - `-codec[:<stream_spec>] <codec>` → (`-codec[:<stream_spec>]`, Some(`<codec>`))
fn extract_flag_and_args(flag_part: &str) -> (String, Option<String>) {
    // Prefer the long form (--xxx). Scan comma-separated segments in reverse.
    for part in flag_part.split(',').rev() {
        let tokens: Vec<&str> = part.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        let raw_flag = tokens[0];
        let flag = raw_flag.split('=').next().unwrap_or(raw_flag);

        if flag.starts_with("--") {
            let args = collect_args(raw_flag, &tokens[1..]);
            return (flag.to_string(), args);
        }
    }

    // No long form found — use the first segment's first token
    let tokens: Vec<&str> = flag_part.split_whitespace().collect();
    if tokens.is_empty() {
        return (String::new(), None);
    }

    let raw_flag = tokens[0];
    // Strip trailing comma if present (e.g. "-v," from "-v, --verbose")
    let raw_flag = raw_flag.trim_end_matches(',');
    let flag = raw_flag.split('=').next().unwrap_or(raw_flag);

    let args = collect_args(raw_flag, &tokens[1..]);
    (flag.to_string(), args)
}

/// Collects argument annotations from the flag token and remaining tokens.
/// Handles `=VALUE`, `<arg>`, `[arg]`, and bracket suffixes like `[:<spec>]`.
fn collect_args(raw_flag: &str, remaining: &[&str]) -> Option<String> {
    let mut parts = Vec::new();

    // Check for =VALUE in the flag itself
    if let Some(eq_pos) = raw_flag.find('=') {
        let val = &raw_flag[eq_pos + 1..];
        if !val.is_empty() {
            parts.push(val.to_string());
        }
    }

    // Collect remaining tokens that look like args: <...>, [...], UPPER_CASE
    for &token in remaining {
        // Skip commas and other flags
        if token == "," || token.starts_with('-') {
            break;
        }
        parts.push(token.to_string());
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

/// Extracts `[default: ...]` and `[possible values: ...]` from a description string.
/// Returns (cleaned description, default_value, possible_values).
fn extract_metadata(description: &str) -> (String, Option<String>, Option<Vec<String>>) {
    let mut default_val = None;
    let mut possible_vals = None;

    // Extract [default: ...]
    if let Some(start) = description.find("[default:")
        && let Some(end) = description[start..].find(']')
    {
        let val = description[start + 9..start + end].trim().to_string();
        if !val.is_empty() {
            default_val = Some(val);
        }
    }

    // Extract [possible values: ...]
    if let Some(start) = description.find("[possible values:")
        && let Some(end) = description[start..].find(']')
    {
        let vals_str = &description[start + 17..start + end];
        let vals: Vec<String> = vals_str
            .split(',')
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .collect();
        if !vals.is_empty() {
            possible_vals = Some(vals);
        }
    }

    // Keep the description as-is (metadata stays visible to the user)
    (description.to_string(), default_val, possible_vals)
}
