use crate::player_state::{PlayerState, State};
use anyhow::Result;
use log::info;
use tao::window::Icon as WindowIcon;
use tray_icon::Icon as TrayIcon;

pub const INIT_SCRIPT: &str = concat!(
    "let YoutubbyCustomCSS = `",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/stylesheet.css"
    )),
    "`;\n",
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/scripts.js"))
);

pub const LOGO: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/youtubby.png"));
pub const ICON: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png"));

pub const PLAY_ICON: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/play_icon.png"));
pub const STOP_ICON: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/stop_icon.png"));
pub const PAUSE_ICON: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/pause_icon.png"
));

pub fn get_image(data: &[u8]) -> Result<(Vec<u8>, u32, u32)> {
    let image = image::load_from_memory(data)?.into_rgba8();
    let width = image.dimensions().0;
    let height = image.dimensions().1;
    Ok((image.into_raw(), width, height))
}

pub fn window_icon() -> Result<WindowIcon> {
    let (icon_data, icon_width, icon_height) = get_image(ICON)?;
    Ok(WindowIcon::from_rgba(icon_data, icon_width, icon_height)?)
}

pub fn player_info_icon(state: &PlayerState) -> Result<TrayIcon> {
    info!("Updating icon with state: {:?}", state);
    let (icon_data, icon_width, icon_height) = match state.state {
        State::Playing => get_image(PLAY_ICON)?,
        State::Paused => get_image(PAUSE_ICON)?,
        State::Stoped => get_image(STOP_ICON)?,
    };
    Ok(TrayIcon::from_rgba(icon_data, icon_width, icon_height)?)
}
