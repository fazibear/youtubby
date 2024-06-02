pub struct LastFm();

use muda::MenuItem;
use url::Url;

use crate::state::State;
use crate::window_handler::WindowHandler;

use serde_json_path::JsonPath;

const API_KEY: &str = "0418be880444b5a60329196d88a4909d";
const API_SECRET: &str = "8dca20779af23f6eb67f5ea424042059";

const AUTH_URL: &str = "https://www.last.fm/api/auth";
const API_URL: &str = "https://ws.audioscrobbler.com/2.0/?";

pub enum AuthState {
    None,
    Waiting(String),
    Connected,
}

impl LastFm {
    pub fn new() -> LastFm {
        LastFm()
    }

    pub fn menu_click(
        &self,
        state: &mut State,
        window_handler: &WindowHandler,
        last_fm_menu: &mut MenuItem,
    ) {
        match state.last_fm_auth_state {
            AuthState::None => {
                if let Some(token) = self.token() {
                    window_handler.open_url(&self.auth_url(&token));
                    state.last_fm_auth_state = AuthState::Waiting(token.to_string());
                    last_fm_menu.set_text("LastFM Continue authentication");
                }
            }
            AuthState::Waiting(ref token) => {
                if let Some(key) = self.session_token(token) {
                    state.preferences.last_fm_session_token = Some(key.to_string());
                    state.last_fm_auth_state = AuthState::Connected;
                    last_fm_menu.set_text("LastFM Logout");
                }
            }
            AuthState::Connected => {}
        }
    }

    fn auth_url(&self, token: &str) -> String {
        let mut url = Url::parse(AUTH_URL).expect("OK");
        url.query_pairs_mut()
            .append_pair("api_key", API_KEY)
            .append_pair("token", token);
        url.to_string()
    }

    fn session_token(&self, token: &str) -> Option<String> {
        let mut url = Url::parse(API_URL).expect("OK");
        url.query_pairs_mut()
            .append_pair("method", "auth.getSession")
            .append_pair("token", token);

        self.fetch_key_from_json(&mut url, "$.session.key")
    }

    fn token(&self) -> Option<String> {
        let mut url = Url::parse(API_URL).expect("OK");
        url.query_pairs_mut().append_pair("method", "auth.getToken");

        self.fetch_key_from_json(&mut url, "$.token")
    }

    fn fetch_key_from_json(&self, url: &mut Url, path_str: &str) -> Option<String> {
        url.query_pairs_mut().append_pair("api_key", API_KEY);

        let mut query = url
            .query_pairs()
            .map(|(a, b)| format!("{}{}", a, b))
            .collect::<Vec<String>>();

        query.sort();

        let sig = format!(
            "{:x}",
            md5::compute(format!("{}{}", query.join(""), API_SECRET))
        );

        url.query_pairs_mut()
            .append_pair("api_sig", &sig)
            .append_pair("format", "json");

        let resp = reqwest::blocking::get(url.to_string())
            .expect("ok")
            .text()
            .expect("ok");

        let json = serde_json::from_str(&resp).expect("ok");

        let path = JsonPath::parse(path_str).expect("ok");
        let val = path
            .query(&json)
            .exactly_one()
            .expect("ok")
            .as_str()
            .expect("ok");

        Some(val.to_string())
    }
}
