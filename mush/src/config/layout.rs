use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Layout {
    pub command_history_height: u16,
    pub line_wrap: bool,
    pub truncate_command_width: u8,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            command_history_height: 500,
            line_wrap: false,
            truncate_command_width: 200,
        }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# The command output history max height")?;
        writeln!(f, "# Set it to 0 and the height will fill the entire console window")?;
        writeln!(f, "command_history_height = {}", self.command_history_height)?;
        writeln!(f, "# Enable line wrapping in command output")?;
        writeln!(f, "line_wrap = {}", self.line_wrap)?;
        writeln!(f, "# Maximum display width for commands before truncation")?;
        writeln!(f, "truncate_command_width = {}", self.truncate_command_width)
    }
}
