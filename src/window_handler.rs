#[cfg(target_os = "macos")]
#[path = "window_handler/macos.rs"]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::WindowHandler;
#[cfg(target_os = "linux")]
#[path = "window_handler/linux.rs"]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::WindowHandler;
#[cfg(target_os = "windows")]
#[path = "window_handler/window.rs"]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::WindowsHandler;

pub static WINDOW_WIDTH: u32 = 896;
pub static WINDOW_HEIGHT: u32 = 1536;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerState {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub state: String,
}

#[derive(Debug)]
pub enum UserEvent {
    PlayerStateUpdated(PlayerState),
}
