use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Aliases {
    #[serde(flatten)]
    pub entries: HashMap<String, String>,
}

impl Default for Aliases {
    fn default() -> Self {
        let mut entries = HashMap::new();
        entries.insert("ll".to_string(), "ls -la --color".to_string());
        Self { entries }
    }
}

impl Aliases {
    /// Look up an alias and parse its value into individual commands.
    pub fn get_commands(&self, name: &str) -> Option<Vec<String>> {
        self.entries.get(name).map(|raw| parse_commands(raw))
    }

    pub fn has(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }
}

/// Splits a raw alias value into individual commands.
/// Supports three formats:
/// - Multiline (each line is a command)
/// - Semicolon-separated (`cmd1; cmd2`)
/// - `&&`-separated (`cmd1 && cmd2`)
pub fn parse_commands(raw: &str) -> Vec<String> {
    let trimmed = raw.trim();

    // If multiline, split on newlines first
    if trimmed.contains('\n') {
        return trimmed
            .lines()
            .flat_map(|line| split_line(line.trim()))
            .filter(|s| !s.is_empty())
            .collect();
    }

    // Single line: split on ; and &&
    split_line(trimmed)
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect()
}

/// Splits a single line on `;` and `&&` separators.
fn split_line(line: &str) -> Vec<String> {
    // First split on &&, then split each part on ;
    line.split("&&")
        .flat_map(|part| part.split(';'))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Formats the raw alias value into a compact single-line description
/// suitable for autocomplete display.
pub fn format_description(raw: &str) -> String {
    let commands = parse_commands(raw);
    commands.join("; ")
}

impl fmt::Display for Aliases {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# Command aliases")?;
        writeln!(f, "# Aliases can use newlines, semicolons, or && to chain commands")?;
        let mut entries: Vec<_> = self.entries.iter().collect();
        entries.sort_by_key(|(k, _)| (*k).clone());
        for (name, value) in entries {
            if value.contains('\n') {
                writeln!(f, "{} = '''", name)?;
                writeln!(f, "{}", value.trim())?;
                writeln!(f, "'''")?;
            } else {
                writeln!(f, "{} = \"{}\"", name, value)?;
            }
        }
        Ok(())
    }
}
