use anyhow::Result;
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
        match (&self, new) {
            (
                PlayerState {
                    state: State::Playing(duration),
                    artist: old_a,
                    track: old_t,
                    ..
                },
                PlayerState {
                    state: State::Paused(_),
                    artist: new_a,
                    track: new_t,
                    ..
                },
            ) if old_a == new_a && old_t == new_t => {
                newnew.state = State::Paused(*duration);
            }
            (
                PlayerState {
                    state: State::Paused(duration),
                    artist: old_a,
                    track: old_t,
                    ..
                },
                PlayerState {
                    state: State::Playing(_),
                    artist: new_a,
                    track: new_t,
                    ..
                },
            ) if old_a == new_a && old_t == new_t => {
                newnew.state = State::Playing(*duration);
            }
            _ => {}
        }
        *self = newnew;
    }

    pub fn from_json_string(json: &str) -> Result<PlayerState> {
        let meta: HashMap<&str, &str> = serde_json::from_str(json)?;

        let start = SystemTime::now();
        let timestamp = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Ok(Self {
            artist: Self::to_option_string(meta["artist"]),
            track: Self::to_option_string(meta["title"]),
            album: Self::to_option_string(meta["album"]),
            state: match meta["state"] {
                "playing" => State::Playing(timestamp),
                "paused" => State::Paused(timestamp),
                _ => State::Stop,
            },
        })
    }

    fn to_option_string(data: &str) -> Option<String> {
        if data.is_empty() {
            None
        } else {
            Some(String::from(data))
        }
    }
}
