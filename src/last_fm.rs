use crate::app::App;
use serde::{Deserialize, Serialize};
use url::Url;

use serde_json_path::JsonPath;

const API_KEY: &str = "0418be880444b5a60329196d88a4909d";
const API_SECRET: &str = "8dca20779af23f6eb67f5ea424042059";

const AUTH_URL: &str = "https://www.last.fm/api/auth";
const API_URL: &str = "https://ws.audioscrobbler.com/2.0/?";

#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    None,
    Waiting(String),
    Connected(String),
}

pub fn menu_click(app: &mut App) {
    match app.preferences.last_fm {
        State::None => {
            if let Some(token) = token() {
                app.window_handler.open_url(&auth_url(&token));
                app.preferences.last_fm = State::Waiting(token.to_string());
                app.menu_handler
                    .last_fm
                    .set_text("LastFM Continue authentication");
            }
        }
        State::Waiting(ref token) => {
            if let Some(key) = session_token(token) {
                app.preferences.last_fm = State::Connected(key.to_string());
                app.menu_handler.last_fm.set_text("LastFM Logout");
            }
        }
        State::Connected(_) => {
            app.preferences.last_fm = State::None;
        }
    }
}

fn auth_url(token: &str) -> String {
    let mut url = Url::parse(AUTH_URL).expect("OK");
    url.query_pairs_mut()
        .append_pair("api_key", API_KEY)
        .append_pair("token", token);
    url.to_string()
}

fn session_token(token: &str) -> Option<String> {
    let mut url = Url::parse(API_URL).expect("OK");
    url.query_pairs_mut()
        .append_pair("method", "auth.getSession")
        .append_pair("token", token);

    fetch_key_from_json(&mut url, "$.session.key")
}

fn token() -> Option<String> {
    let mut url = Url::parse(API_URL).expect("OK");
    url.query_pairs_mut().append_pair("method", "auth.getToken");

    fetch_key_from_json(&mut url, "$.token")
}

fn fetch_key_from_json(url: &mut Url, path_str: &str) -> Option<String> {
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
