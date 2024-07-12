use super::{
    assets,
    menu_handler::MenuHandler,
    player_state::{self, PlayerState, PlayerStateMetaData},
    Youtubby,
};
use anyhow::Result;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

const MAX_PLAYER_INFO_STRING_LENGTH: usize = 46;

pub struct TrayHandler {
    pub icon: TrayIcon,
}

impl TrayHandler {
    pub fn init(menu_handler: &MenuHandler) -> Result<TrayHandler> {
        let (icon_data, icon_width, icon_height) = assets::get_image(assets::ICON)?;
        let icon_data = Icon::from_rgba(icon_data, icon_width, icon_height)?;
        let icon = TrayIconBuilder::new()
            .with_id("0")
            .with_menu(Box::new(menu_handler.menu.clone()))
            .with_menu_on_left_click(false)
            .with_icon(icon_data)
            .build()?;

        Ok(Self { icon })
    }
}

pub fn refresh(app: &mut Youtubby) -> Result<()> {
    let tray = if app.preferences.show_info_in_tray {
        player_info(&app.player_state)
    } else {
        None
    };

    let tooltip = if app.preferences.show_info_in_tooltip {
        player_info(&app.player_state)
    } else {
        None
    };

    app.tray_handler.icon.set_title(tray);
    app.tray_handler.icon.set_tooltip(tooltip)?;
    Ok(())
}

pub fn player_info(state: &PlayerState) -> Option<String> {
    let icon = match state.state {
        player_state::State::Playing => "󰐊",
        player_state::State::Paused => "󰏤",
        player_state::State::Stoped => "󰓛",
    };

    if let PlayerStateMetaData {
        artist: Some(ref artist),
        track: Some(ref track),
        ..
    } = state.metadata
    {
        let info = format!("{icon} {artist} - {track}");

        if info.chars().count() > MAX_PLAYER_INFO_STRING_LENGTH {
            let short = truncate(&info, MAX_PLAYER_INFO_STRING_LENGTH);
            Some(format!("{short}..."))
        } else {
            Some(info)
        }
    } else {
        None
    }
}

fn truncate(s: &String, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}