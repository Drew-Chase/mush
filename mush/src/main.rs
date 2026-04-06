mod config;
mod db;
mod shell;
mod widgets;

use std::env::set_current_dir;
use crate::config::Config;
use crate::widgets::App;

fn main() -> color_eyre::Result<()> {
    #[cfg(debug_assertions)]
    set_current_dir("target/")?;
    color_eyre::install()?;
    Config::load_or_default("./config.toml")?;
    Config::get().save()?;
    ratatui::run(|terminal| -> color_eyre::Result<()> {
        let mut instance = App::new()?;
        instance.run(terminal)?;
        Ok(())
    })?;
    Ok(())
}
