use crate::app::App;
use crate::player_state::{self, PlayerState};
use serde::{Deserialize, Serialize};
use serde_json_path::JsonPath;
use url::Url;
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

pub fn track_update_now_playing(app: &mut App) {
    if let PlayerState {
        state: player_state::State::Playing(_),
        track: Some(ref track),
        artist: Some(ref artist),
        ref album,
        ..
    } = app.player_state
    {
        if let State::Connected(ref sk) = app.preferences.last_fm {
            let mut url = Url::parse(API_URL).expect("OK");
            url.query_pairs_mut()
                .append_pair("api_key", API_KEY)
                .append_pair("method", "track.updateNowPlaying")
                .append_pair("artist", artist)
                .append_pair("track", track)
                .append_pair("sk", sk);

            if let Some(ref album) = album {
                url.query_pairs_mut().append_pair("album", album);
            };

            let sig = generate_signature(&mut url);

            url.query_pairs_mut()
                .append_pair("api_sig", &sig)
                .append_pair("format", "json");

            let query = url.query().unwrap();
            let encoded = UrlEncodedData::from(query).to_string();

            println!("{}", encoded);

            let client = reqwest::blocking::Client::new();
            let res = client.post(API_URL).body(encoded).send().expect("ok");
            let text = res.text().expect("ok");

            println!("response: {:?}", text);
        }
    }
}

pub fn track_scrobble(app: &mut App) {
    if let PlayerState {
        state: player_state::State::Playing(timestamp),
        track: Some(ref track),
        artist: Some(ref artist),
        ref album,
        ..
    } = app.player_state
    {
        if let State::Connected(ref sk) = app.preferences.last_fm {
            let mut url = Url::parse(API_URL).expect("OK");
            url.query_pairs_mut()
                .append_pair("api_key", API_KEY)
                .append_pair("method", "track.scrobble")
                .append_pair("artist[0]", artist)
                .append_pair("track[0]", track)
                .append_pair("timestamp[0]", &timestamp.as_secs().to_string())
                .append_pair("sk", sk);

            if let Some(ref album) = album {
                url.query_pairs_mut().append_pair("album[0]", album);
            };

            let sig = generate_signature(&mut url);

            url.query_pairs_mut()
                .append_pair("api_sig", &sig)
                .append_pair("format", "json");

            let query = url.query().unwrap();
            let encoded = UrlEncodedData::from(query).to_string();

            println!("{}", encoded);

            let client = reqwest::blocking::Client::new();
            let res = client.post(API_URL).body(encoded).send().expect("ok");
            let text = res.text().expect("ok");

            println!("response: {:?}", text);
        }
    }
}

pub fn menu_click(app: &mut App) {
    match app.preferences.last_fm {
        State::None => {
            if let Some(token) = auth_get_token() {
                app.window_handler.open_url(&auth_url(&token));
                app.preferences.last_fm = State::Waiting(token.to_string());
                app.menu_handler
                    .last_fm
                    .set_text("LastFM Continue authentication");
            }
        }
        State::Waiting(ref token) => {
            if let Some(key) = auth_get_session(token) {
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

fn auth_get_session(token: &str) -> Option<String> {
    let mut url = Url::parse(API_URL).expect("OK");
    url.query_pairs_mut()
        .append_pair("api_key", API_KEY)
        .append_pair("method", "auth.getSession")
        .append_pair("token", token);

    let sig = generate_signature(&mut url);

    url.query_pairs_mut()
        .append_pair("api_sig", &sig)
        .append_pair("format", "json");

    let client = reqwest::blocking::Client::new();
    let json = client.get(url).send().expect("ok").text().expect("ok");

    fetch_key_from_json(json, "$.session.key")
}

fn auth_get_token() -> Option<String> {
    let mut url = Url::parse(API_URL).expect("OK");
    url.query_pairs_mut()
        .append_pair("api_key", API_KEY)
        .append_pair("method", "auth.getToken");

    let sig = generate_signature(&mut url);

    url.query_pairs_mut()
        .append_pair("api_sig", &sig)
        .append_pair("format", "json");

    let client = reqwest::blocking::Client::new();
    let json = client.get(url).send().expect("ok").text().expect("ok");

    fetch_key_from_json(json, "$.token")
}

fn generate_signature(url: &mut Url) -> String {
    let mut query = url
        .query_pairs()
        .map(|(a, b)| format!("{}{}", a, b))
        .collect::<Vec<String>>();

    query.sort();

    format!(
        "{:x}",
        md5::compute(format!("{}{}", query.join(""), API_SECRET))
    )
}

fn fetch_key_from_json(resp: String, path_str: &str) -> Option<String> {
    println!("{}", resp);

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
