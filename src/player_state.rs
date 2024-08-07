use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerStateMetaData {
    pub artist: Option<String>,
    pub track: Option<String>,
    pub album: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerState {
    pub timestamp: u64,
    pub position: Option<u64>,
    pub duration: Option<u64>,
    pub state: State,
    pub metadata: PlayerStateMetaData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum State {
    Playing,
    Paused,
    Stoped,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        let start = SystemTime::now();
        let timestamp = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Self {
            timestamp,
            position: None,
            duration: None,
            state: State::Stoped,
            metadata: PlayerStateMetaData {
                artist: None,
                track: None,
                album: None,
            },
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new()
    }
}
