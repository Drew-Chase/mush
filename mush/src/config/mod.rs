pub mod alias;
pub mod layout;
pub mod startup;

use color_eyre::Result;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    #[serde(skip)]
    _save_path: Option<PathBuf>,
    pub layout: layout::Layout,
    pub application: startup::Application,
    #[serde(default)]
    pub alias: alias::Aliases,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[layout]\n{}\n[application]\n{}\n[alias]\n{}", self.layout, self.application, self.alias)
    }
}

impl Config {
    /// Returns a reference to the global config.
    /// Panics if `load_or_default` has not been called.
    pub fn get() -> &'static Config {
        CONFIG
            .get()
            .expect("Config not initialized — call Config::load_or_default() first")
    }

    /// Loads config from `path` if it exists, otherwise uses defaults.
    /// Stores the path for subsequent `save()` calls.
    pub fn load_or_default(path: impl Into<PathBuf>) -> Result<()> {
        let path = path.into();
        let mut config = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            toml::from_str::<Config>(&content)?
        } else {
            Config::default()
        };
        config._save_path = Some(path);
        CONFIG
            .set(config)
            .map_err(|_| eyre!("Config already initialized"))?;
        Ok(())
    }

    pub fn db_path(&self) -> PathBuf {
        self._save_path
            .as_ref()
            .and_then(|p| p.parent())
            .map(|dir| dir.join("mush.db"))
            .unwrap_or_else(|| crate::get_appdata_path().join("mush.db"))
    }

    /// Saves the current config to the path it was loaded from.
    pub fn save(&self) -> Result<()> {
        let path = self
            ._save_path
            .as_ref()
            .ok_or_else(|| eyre!("No save path configured"))?;

        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, self.to_string())?;
        Ok(())
    }
}
