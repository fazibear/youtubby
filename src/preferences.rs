use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Preferences {
    pub show_info_in_tray: bool,
    pub show_info_in_tooltip: bool,
    pub hide_unfocused_window: bool,
    pub last_fm_session_token: Option<String>,
}

impl Preferences {
    pub fn new() -> Preferences {
        Preferences {
            show_info_in_tray: true,
            show_info_in_tooltip: true,
            hide_unfocused_window: true,
            last_fm_session_token: None,
        }
    }

    pub fn load() -> Preferences {
        if let Ok(ref mut file) = File::open(Self::config_file()) {
            let mut buf = String::new();
            file.read_to_string(&mut buf).expect("ok");
            if let Ok(state) = serde_json::from_str(&buf) {
                file.sync_all().expect("ok");
                return state;
            }
        }
        Preferences::new()
    }

    pub fn save(&self) {
        if let Ok(ref mut file) = File::create(Self::config_file()) {
            let data = serde_json::to_string_pretty(&self).expect("OK");
            file.write_all(data.as_bytes()).expect("ok");
            file.sync_all().expect("ok");
        }
    }

    fn config_file() -> PathBuf {
        let dirs = ProjectDirs::from("com", "fazibear", "Youtubby").expect("ok");
        let config = dirs.config_dir();
        fs::create_dir_all(config).expect("ok");
        config.join("config.json")
    }
}
