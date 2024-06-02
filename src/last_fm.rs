pub struct LastFm();

use std::collections::HashMap;

use url::Url;

const API_KEY: &str = "0418be880444b5a60329196d88a4909d";
const AUTH_URL: &str = "https://www.last.fm/api/auth";
const TOKEN_URL: &str = "https://ws.audioscrobbler.com/2.0/?method=auth.gettoken&format=json";

impl LastFm {
    pub fn new() -> LastFm {
        LastFm()
    }

    pub fn auth_url(&self) -> String {
        let token = self.get_token();
        let mut url = Url::parse(AUTH_URL).expect("OK");
        url.query_pairs_mut()
            .append_pair("api_key", API_KEY)
            .append_pair("token", &token);
        url.to_string()
    }

    fn get_token(&self) -> String {
        let mut url = Url::parse(TOKEN_URL).expect("OK");
        url.query_pairs_mut().append_pair("api_key", API_KEY);

        let json = reqwest::blocking::get(url.to_string())
            .expect("ok")
            .text()
            .expect("ok");

        serde_json::from_str::<HashMap<&str, &str>>(&json)
            .expect("ok")
            .get("token")
            .expect("ok")
            .to_string()
    }
}
