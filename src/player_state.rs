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
    pub timestamp: Option<u64>,
    pub position: Option<f32>,
    pub duration: Option<u32>,
    pub state: State,
    pub metadata: PlayerStateMetaData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum State {
    Playing,
    Paused,
    Stoped,
    Waiting,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        Self {
            timestamp: None,
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

    pub fn update_timestamp(&mut self) {
        let start = SystemTime::now();
        let timestamp = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        self.timestamp = Some(timestamp.as_secs())
    }
}
