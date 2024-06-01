use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub show_song_in_tray: bool,
    pub show_song_in_tooltip: bool,
    pub hide_unfocused_window: bool,
    pub lastfm_api_key: String,
}

impl State {
    pub fn load() -> State {
        if let Ok(ref mut file) = File::open(Self::config_file()) {
            let mut buf = String::new();
            file.read_to_string(&mut buf).expect("ok");
            if let Ok(state) = serde_json::from_str(&buf) {
                file.sync_all().expect("ok");
                println!("{:?}", state);
                return state;
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Ok(ref mut file) = File::create(Self::config_file()) {
            let data = serde_json::to_string_pretty(self).expect("OK");
            file.write_all(data.as_bytes()).expect("ok");
            file.sync_all().expect("ok");
        }
    }

    fn default() -> Self {
        Self {
            show_song_in_tray: true,
            show_song_in_tooltip: true,
            hide_unfocused_window: true,
            lastfm_api_key: String::from("123testxxx123"),
        }
    }

    fn config_file() -> PathBuf {
        let dirs = ProjectDirs::from("com", "fazibear", "Youtubby").expect("ok");
        let config = dirs.config_dir();
        fs::create_dir_all(config).expect("ok");
        config.join("config.json")
    }
}
