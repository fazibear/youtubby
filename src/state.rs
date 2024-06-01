pub struct State {
    pub show_song_in_tray: bool,
    pub show_song_in_tooltip: bool,
    pub hide_unfocused_window: bool,
    pub lastfm_api_key: String,
}

impl State {
    pub fn load_or_default() -> Self {
        if let Some(state) = Self::load() {
            state
        }else{
            Self::default()
        }
    }

    pub fn save(&self) {
        todo!()
    }

    fn default() -> Self {
        Self {
            show_song_in_tray: true,
            show_song_in_tooltip: true,
            hide_unfocused_window: true,
            lastfm_api_key: String::from(""),
        }
    }

    fn load() -> Option<State> {
        Some(Self::default())
    }
}
