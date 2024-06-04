use std::time::{SystemTime, UNIX_EPOCH};

use crate::app::App;
use crate::player_state::{self, PlayerState};
use serde::{Deserialize, Serialize};
use url::Url;

use serde_json_path::JsonPath;
use url_encoded_data::UrlEncodedData;

const API_KEY: &str = "0418be880444b5a60329196d88a4909d";
const API_SECRET: &str = "8dca20779af23f6eb67f5ea424042059";

const AUTH_URL: &str = "https://www.last.fm/api/auth";
const API_URL: &str = "https://ws.audioscrobbler.com/2.0/";

#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    None,
    Waiting(String),
    Connected(String),
}

pub fn submit(app: &mut App) {
    if let PlayerState {
        state: player_state::State::PLAYING,
        track: Some(ref track),
        artist: Some(ref artist),
        ref album,
        ..
    } = app.player_state
    {
        if let State::Connected(ref sk) = app.preferences.last_fm {
            let start = SystemTime::now();
            let timestamp = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            let mut url = Url::parse(API_URL).expect("OK");
            url.query_pairs_mut()
                .append_pair("method", "track.scrobble")
                .append_pair("artist[0]", artist)
                .append_pair("track[0]", track)
                .append_pair("timestamp[0]", &timestamp.as_secs().to_string())
                .append_pair("sk", sk);

            if let Some(ref album) = album {
                url.query_pairs_mut().append_pair("album[0]", album);
            };

            let (_, query2) = prepare_url(&mut url);
            let test = query2.as_str();
            let encoded = UrlEncodedData::from(test).to_string();

            println!("{}", query2);
            println!("{}", encoded);

            let client = reqwest::blocking::Client::new();
            let res = client
                .post(url.to_string())
                .body(encoded)
                .send()
                .expect("ok");

            let text = res.text().expect("ok");

            println!("response: {:?}", text);
        }
    }
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

    let (final_url, _) = prepare_url(&mut url);
    fetch_key_from_json_url(final_url, "$.session.key")
}

fn token() -> Option<String> {
    let mut url = Url::parse(API_URL).expect("OK");
    url.query_pairs_mut().append_pair("method", "auth.getToken");

    let (final_url, _) = prepare_url(&mut url);
    fetch_key_from_json_url(final_url, "$.token")
}

fn prepare_url(url: &mut Url) -> (String, String) {
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
    println!("{} = {}", query.join(""), sig);

    url.query_pairs_mut()
        .append_pair("api_sig", &sig)
        .append_pair("format", "json");

    (url.to_string(), url.query().unwrap().to_string())
}

fn fetch_key_from_json_url(url: String, path_str: &str) -> Option<String> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).send().expect("ok").text().expect("ok");

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
