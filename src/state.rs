use crate::last_fm;
use crate::window_handler::PlayerState;
use crate::Preferences;

const MAX_PLAYER_INFO_STRING_LENGTH: usize = 46;

pub struct State {
    pub preferences: Preferences,
    pub player_info: String,
    pub last_fm_auth_state: last_fm::AuthState,
}

impl State {
    pub fn new() -> State {
        Self {
            preferences: Preferences::load(),
            player_info: String::from(""),
            last_fm_auth_state: last_fm::AuthState::None,
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
}
