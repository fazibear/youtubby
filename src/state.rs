use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub show_song_in_tray: bool,
    pub show_song_in_tooltip: bool,
    pub hide_unfocused_window: bool,
    pub lastfm_api_key: String,
}

impl State {
    pub fn load() -> State {
        if let Some(file) = Self::config_file() {
            let reader = BufReader::new(file);
            if let Ok(state) = serde_json::from_reader(reader) {
                return state;
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(file) = Self::config_file() {
            let writer = BufWriter::new(file);
            let _ = serde_json::to_writer(writer, self);
        }
    }

    fn default() -> Self {
        Self {
            show_song_in_tray: true,
            show_song_in_tooltip: true,
            hide_unfocused_window: true,
            lastfm_api_key: String::from(""),
        }
    }

    fn config_file() -> Option<File> {
        if let Some(dirs) = ProjectDirs::from("com", "fazibear", "Youtubby") {
            let path = dirs.config_dir().join("config.json");
            if let Ok(file) = File::open(path) {
                return Some(file);
            }
        }
        None
    }
}
