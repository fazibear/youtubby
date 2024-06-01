use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::window_handler::PlayerState;

const MAX_PLAYER_INFO_STRING_LENGTH: usize = 46;

#[derive(Serialize, Deserialize, Debug)]
pub struct Preferences {
    pub show_info_in_tray: bool,
    pub show_info_in_tooltip: bool,
    pub hide_unfocused_window: bool,
    pub lastfm_api_key: String,
}

pub struct State {
    pub preferences: Preferences,
    pub player_info: String,
}

impl State {
    pub fn default() -> State {
        Self {
            preferences: Self::load_preferences(),
            player_info: String::from(""),
        }
    }

    pub fn update_player_info(&mut self, meta: &PlayerState) {
        let play = if meta.state == "playing" {
            "▶"
        } else {
            "⏸"
        };
        let mut info = format!("{} {} - {}", play, meta.artist, meta.title);

        if info.len() > MAX_PLAYER_INFO_STRING_LENGTH {
            info.truncate(MAX_PLAYER_INFO_STRING_LENGTH);
            info.push_str("...");
        }

        self.player_info = info;
    }

    pub fn load_preferences() -> Preferences {
        if let Ok(ref mut file) = File::open(Self::config_file()) {
            let mut buf = String::new();
            file.read_to_string(&mut buf).expect("ok");
            if let Ok(state) = serde_json::from_str(&buf) {
                file.sync_all().expect("ok");
                return state;
            }
        }
        Preferences {
            show_info_in_tray: true,
            show_info_in_tooltip: true,
            hide_unfocused_window: true,
            lastfm_api_key: String::from(""),
        }
    }

    pub fn save_preferences(&self) {
        if let Ok(ref mut file) = File::create(Self::config_file()) {
            let data = serde_json::to_string_pretty(&self.preferences).expect("OK");
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
