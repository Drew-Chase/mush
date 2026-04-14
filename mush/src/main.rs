//! Mush - Multi-Unified Shell
//!
//! A cross-platform shell interpreter with a TUI interface, 78 bundled Unix utilities,
//! and smart autocomplete. Built with [Ratatui](https://ratatui.rs) for live-streaming
//! terminal output.

mod config;
mod db;
mod shell;
mod widgets;

use clap::Parser;

use crate::config::Config;
use crate::widgets::App;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mush", version, about = "A cross-platform shell interpreter")]
struct MushArgs {
    /// Open config file in default editor
    #[arg(long)]
    config: bool,

    /// Print install directory
    #[arg(long)]
    install_dir: bool,

    /// Open GitHub repository in browser
    #[arg(long)]
    github: bool,

    /// Check for available updates
    #[arg(long)]
    check_updates: bool,

    /// Install available updates
    #[arg(long)]
    install_updates: bool,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = MushArgs::parse();

    if args.config {
        let config_path = get_appdata_path().join("config.toml");
        // Ensure config file exists before opening
        std::fs::create_dir_all(get_appdata_path())?;
        Config::load_or_default(config_path.clone())?;
        Config::get().save()?;
        open::that(&config_path)?;
        return Ok(());
    }
    if args.install_dir {
        let dir = std::env::current_exe()?
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_default();
        println!("{}", dir.display());
        return Ok(());
    }
    if args.github {
        open::that("https://github.com/drew-chase/mush")?;
        return Ok(());
    }
    if args.check_updates {
        println!("Update checking is not yet implemented.");
        return Ok(());
    }
    if args.install_updates {
        println!("Update installation is not yet implemented.");
        return Ok(());
    }

    std::fs::create_dir_all(get_appdata_path())?;
    std::fs::create_dir_all(get_appdata_path().join("scripts"))?;
    shell::script_registry::scan_scripts(&get_appdata_path().join("scripts"));
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
    } else if let Some(config) = dirs::config_dir() {
        config.join("mush")
    } else {
        std::env::temp_dir().join("mush")
    }
}
