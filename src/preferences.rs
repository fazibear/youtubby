use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::last_fm;

#[derive(Serialize, Deserialize, Debug)]
pub struct Preferences {
    pub show_info_in_tray: bool,
    pub show_info_in_tooltip: bool,
    pub hide_unfocused_window: bool,
    pub always_on_top: bool,
    pub last_fm: last_fm::State,
}

impl Preferences {
    pub fn new() -> Preferences {
        Preferences {
            show_info_in_tray: true,
            show_info_in_tooltip: true,
            hide_unfocused_window: true,
            always_on_top: true,
            last_fm: last_fm::State::None,
        }
    }

    pub fn load() -> Result<Preferences> {
        if let Ok(ref mut file) = File::open(Self::config_file()?) {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            if let Ok(state) = serde_json::from_str(&buf) {
                file.sync_all()?;
                return Ok(state);
            }
        }
        Ok(Preferences::new())
    }

    pub fn save(&self) -> Result<()> {
        if let Ok(ref mut file) = File::create(Self::config_file()?) {
            let data = serde_json::to_string_pretty(&self)?;
            file.write_all(data.as_bytes())?;
            file.sync_all()?;
        }
        Ok(())
    }

    fn config_file() -> Result<PathBuf> {
        let dirs = ProjectDirs::from("com", "fazibear", "Youtubby")
            .context("project directory missing")?;
        let config = dirs.config_dir();
        fs::create_dir_all(config)?;
        Ok(config.join("config.json"))
    }
}
