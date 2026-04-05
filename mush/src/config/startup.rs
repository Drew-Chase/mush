use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    pub default_cwd: Option<String>,
    pub theme: String,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            default_cwd: None,
            theme: String::from("dark.joker"),
        }
    }
}

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# Default working directory on startup")?;
        match &self.default_cwd {
            Some(cwd) => writeln!(f, "default_cwd = \"{}\"", cwd)?,
            None => writeln!(f, "# default_cwd = \"~/\"")?,
        }
        writeln!(f, "# Color theme name")?;
        writeln!(f, "theme = \"{}\"", self.theme)
    }
}