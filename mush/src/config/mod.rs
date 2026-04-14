pub mod alias;
pub mod layout;
pub mod startup;

use color_eyre::Result;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock, RwLockReadGuard};

static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();

/// Global shell configuration loaded from TOML on startup.
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
    /// Returns a read guard to the global config.
    /// Panics if `load_or_default` has not been called.
    pub fn get() -> RwLockReadGuard<'static, Config> {
        CONFIG
            .get()
            .expect("Config not initialized — call Config::load_or_default() first")
            .read()
            .expect("Config RwLock poisoned")
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
            .set(RwLock::new(config))
            .map_err(|_| eyre!("Config already initialized"))?;
        Ok(())
    }

    /// Re-reads config from disk and replaces the current in-memory config.
    /// On parse failure the old config is kept and an error is returned.
    pub fn reload() -> Result<()> {
        let lock = CONFIG
            .get()
            .ok_or_else(|| eyre!("Config not initialized"))?;

        // Read path from current config (short read lock)
        let path = {
            let current = lock.read().map_err(|_| eyre!("Config RwLock poisoned"))?;
            current
                ._save_path
                .clone()
                .ok_or_else(|| eyre!("No config path set"))?
        };

        // Parse new config from disk (no lock held during I/O)
        let mut new_config = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            toml::from_str::<Config>(&content)?
        } else {
            Config::default()
        };
        new_config._save_path = Some(path);

        // Swap in the new config
        let mut writer = lock.write().map_err(|_| eyre!("Config RwLock poisoned"))?;
        *writer = new_config;
        Ok(())
    }

    /// Acquires a write lock, calls the closure with mutable config, then saves.
    pub fn write_with<F, R>(f: F) -> Result<R>
    where
        F: FnOnce(&mut Config) -> R,
    {
        let lock = CONFIG
            .get()
            .ok_or_else(|| eyre!("Config not initialized"))?;
        let mut writer = lock.write().map_err(|_| eyre!("Config RwLock poisoned"))?;
        let result = f(&mut writer);
        writer.save()?;
        Ok(result)
    }

    /// Returns the path to the SQLite history database, derived from the config file location.
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

        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| eyre!("config serialize: {e}"))?;
        std::fs::write(path, toml_str)?;
        Ok(())
    }
}
