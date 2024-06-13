use crate::player_state::PlayerStateMetaData;
use anyhow::{Context, Error, Result};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum PlayerStateChanged {
    Play(PlayerStateMetaData),
    Stop,
    Pause,
    Seeked,
    Emptied,
    TimeUpdate(f64),
    DurationChange(i64),
    Waiting,
    MetaDataUpdate(PlayerStateMetaData),
}

impl PlayerStateChanged {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let meta: HashMap<&str, Value> = serde_json::from_str(json)?;

        match meta.get("type").context("need type field")? {
            Value::String(t) if t == "stop" => Ok(Self::Stop),
            Value::String(t) if t == "pause" => Ok(Self::Pause),
            Value::String(t) if t == "seeked" => Ok(Self::Seeked),
            Value::String(t) if t == "waiting" => Ok(Self::Waiting),
            Value::String(t) if t == "emptied" => Ok(Self::Emptied),

            Value::String(t) if t == "play" => {
                if let Some(Value::Object(metadata)) = meta.get("metadata") {
                    return Ok(Self::Play(PlayerStateMetaData {
                        artist: Self::to_option_string(metadata.get("artist")),
                        track: Self::to_option_string(metadata.get("title")),
                        album: Self::to_option_string(metadata.get("album")),
                    }));
                }
                Err(Error::msg("can't parse metadata"))
            }

            Value::String(t) if t == "durationchange" => {
                if let Some(Value::Number(duration)) = meta.get("duration") {
                    log::info!("{}", duration);
                    if let Some(duration_as_int) = duration.as_i64() {
                        return Ok(Self::DurationChange(duration_as_int));
                    }
                }
                Err(Error::msg("can't parse duration"))
            }
            Value::String(t) if t == "timeupdate" => {
                if let Some(Value::Number(time)) = meta.get("time") {
                    if let Some(time_as_float) = time.as_f64() {
                        return Ok(Self::TimeUpdate(time_as_float));
                    }
                }
                Err(Error::msg("can't parse time"))
            }

            Value::String(t) if t == "metadataupdate" => {
                if let Some(Value::Object(metadata)) = meta.get("metadata") {
                    return Ok(Self::MetaDataUpdate(PlayerStateMetaData {
                        artist: Self::to_option_string(metadata.get("artist")),
                        track: Self::to_option_string(metadata.get("title")),
                        album: Self::to_option_string(metadata.get("album")),
                    }));
                }
                Err(Error::msg("can't parse metadata"))
            }

            _ => Err(Error::msg(format!("unknown event: {json}"))),
        }
    }

    fn to_option_string(data: Option<&Value>) -> Option<String> {
        if let Some(Value::String(string)) = data {
            if !string.is_empty() {
                return Some(string.clone());
            }
        }
        None
    }
}
