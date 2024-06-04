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
    PLAYING,
    STOP,
    PAUSED,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        Self {
            artist: None,
            track: None,
            album: None,
            state: State::STOP,
        }
    }

    pub fn from_json(json: &str) -> PlayerState {
        serde_json::from_str(json).unwrap()
    }
}
