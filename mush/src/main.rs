mod app;
mod command_input;
mod config;

use std::env::set_current_dir;
use crate::app::App;
use crate::config::Config;

fn main() -> color_eyre::Result<()> {
    #[cfg(debug_assertions)]
    set_current_dir("target/")?;
    color_eyre::install()?;
    Config::load_or_default("./config.toml")?;
    Config::get().save()?;
//    ratatui::run(|terminal| -> color_eyre::Result<()> {
//        let mut instance = App::default();
//        instance.run(terminal)?;
//        Ok(())
//    })?;
    Ok(())
}
