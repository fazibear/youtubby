use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerState {
    pub artist: Option<String>,
    pub track: Option<String>,
    pub album: Option<String>,
    pub state: State,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum State {
    Playing(Duration),
    Paused(Duration),
    Stop,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        Self {
            artist: None,
            track: None,
            album: None,
            state: State::Stop,
        }
    }

    pub fn update(&mut self, new: &PlayerState) {
        let mut newnew = new.clone();
        match (&self.state, &new.state) {
            (State::Playing(duration), State::Paused(_)) => {
                newnew.state = State::Paused(*duration);
            }
            (State::Paused(duration), State::Playing(_)) => {
                newnew.state = State::Playing(*duration);
            }
            _ => {}
        }
        *self = newnew;
    }

    pub fn from_json_string(json: &str) -> PlayerState {
        let meta: HashMap<&str, &str> = serde_json::from_str(json).unwrap();

        let start = SystemTime::now();
        let timestamp = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Self {
            artist: Self::to_option_string(meta["artist"]),
            track: Self::to_option_string(meta["title"]),
            album: Self::to_option_string(meta["album"]),
            state: match meta["state"] {
                "playing" => State::Playing(timestamp),
                "paused" => State::Paused(timestamp),
                _ => State::Stop,
            },
        }
    }

    fn to_option_string(data: &str) -> Option<String> {
        if data.is_empty() {
            None
        } else {
            Some(String::from(data))
        }
    }
}
