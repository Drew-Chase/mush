mod config;
mod db;
mod shell;
mod widgets;

use crate::config::Config;
use crate::widgets::App;
use std::path::PathBuf;


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    std::fs::create_dir_all(get_appdata_path())?;
    Config::load_or_default(get_appdata_path().join("config.toml"))?;
    Config::get().save()?;
    ratatui::run(|terminal| -> color_eyre::Result<()> {
        let mut instance = App::new()?;
        instance.run(terminal)?;
        Ok(())
    })?;
    Ok(())
}

pub fn get_appdata_path() -> PathBuf {
    if let Ok(config_path) = std::env::var("MUSH_APPDATA") {
        PathBuf::from(config_path)
    } else if let Some(local_appdata) = dirs::config_local_dir() {
        local_appdata.join("mush")
    } else {
        dirs::config_dir().unwrap().join("mush")
    }
}
