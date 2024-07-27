use super::{
    assets,
    menu_handler::MenuHandler,
    player_state::{PlayerState, PlayerStateMetaData},
    Youtubby,
};
use anyhow::Result;
use tray_icon::{TrayIcon, TrayIconBuilder};

const MAX_PLAYER_INFO_STRING_LENGTH: usize = 46;

pub struct TrayHandler {
    pub icon: TrayIcon,
}

impl TrayHandler {
    pub fn init(menu_handler: &MenuHandler) -> Result<TrayHandler> {
        let icon = TrayIconBuilder::new()
            .with_id("0")
            .with_menu(Box::new(menu_handler.menu.clone()))
            .with_menu_on_left_click(false)
            .with_icon(assets::player_default_icon()?)
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

    let icon = assets::player_info_icon(&app.player_state).ok();

    app.tray_handler.icon.set_title(tray);
    app.tray_handler.icon.set_icon(icon)?;
    app.tray_handler.icon.set_tooltip(tooltip)?;
    Ok(())
}

pub fn player_info(state: &PlayerState) -> Option<String> {
    if let PlayerStateMetaData {
        artist: Some(ref artist),
        track: Some(ref track),
        ..
    } = state.metadata
    {
        let info = format!("{artist} - {track}");

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
