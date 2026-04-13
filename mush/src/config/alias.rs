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
    /// Look up an alias and return its raw value as a single command string.
    /// The shell parser handles `&&`, `||`, `;`, and pipes correctly, so we
    /// must NOT pre-split — that would destroy conditional semantics.
    /// Multiline aliases are joined with ` ; ` so the parser sees them as
    /// sequential commands.
    pub fn get_commands(&self, name: &str) -> Option<String> {
        self.entries.get(name).map(|raw| {
            let trimmed = raw.trim();
            if trimmed.contains('\n') {
                trimmed
                    .lines()
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ; ")
            } else {
                trimmed.to_string()
            }
        })
    }

    pub fn has(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }
}

/// Formats the raw alias value into a compact single-line description
/// suitable for autocomplete display.
pub fn format_description(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.contains('\n') {
        trimmed
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join("; ")
    } else {
        trimmed.to_string()
    }
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
