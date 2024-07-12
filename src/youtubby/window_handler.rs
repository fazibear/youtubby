#[cfg(target_os = "windows")]
#[path = "window_handler/windows.rs"]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::WindowHandler;

#[cfg(target_os = "linux")]
#[path = "window_handler/linux.rs"]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::WindowHandler;

#[cfg(target_os = "macos")]
#[path = "window_handler/macos.rs"]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::WindowHandler;

use super::Youtubby;
use anyhow::Result;
use wry::dpi::{LogicalSize, PhysicalPosition, PhysicalSize};

pub static WINDOW_WIDTH: u32 = 896;
pub static WINDOW_HEIGHT: u32 = 1536;
pub static USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";
pub static URL: &str = "https://music.youtube.com";

pub static WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
pub static WINDOW_MIN_SIZE: LogicalSize<u32> = LogicalSize::new(320, 0);

impl WindowHandler {
    pub fn show_hide(&self, position: PhysicalPosition<f64>) {
        if self.window.is_visible().unwrap_or(false) {
            self.hide();
        } else {
            self.set_position(position);
            self.show();
        }
    }

    pub fn hide(&self) {
        self.window.set_visible(false);
    }

    pub fn set_position(&self, position: PhysicalPosition<f64>) {
        self.window.set_outer_position(PhysicalPosition::new(
            position.x - (WINDOW_WIDTH / 2) as f64,
            100.,
        ));
    }

    pub fn show(&self) {
        self.window.set_visible(true);
        //todo!("show on all workspaces")
        self.window.focus_window();
    }
}

pub fn refresh(_app: &mut Youtubby) -> Result<()> {
    //todo!("Always on top")
    Ok(())
}
