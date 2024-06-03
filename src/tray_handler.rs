use crate::app::App;
use crate::window_handler::PlayerState;
use crate::{assets, menu_handler::MenuHandler};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconEventReceiver};

const MAX_PLAYER_INFO_STRING_LENGTH: usize = 46;

pub struct TrayHandler {
    pub icon: TrayIcon,
    pub channel: &'static TrayIconEventReceiver,
}

impl TrayHandler {
    pub fn new(menu_handler: &MenuHandler) -> TrayHandler {
        let (icon_data, icon_width, icon_height) = assets::get_image(assets::ICON);
        let icon_data = Icon::from_rgba(icon_data, icon_width, icon_height).unwrap();

        let icon = TrayIconBuilder::new()
            .with_id("0")
            .with_menu(Box::new(menu_handler.menu.clone()))
            .with_menu_on_left_click(false)
            .with_icon(icon_data)
            .build()
            .unwrap();

        let channel = TrayIconEvent::receiver();

        Self { channel, icon }
    }
}

pub fn refresh(app: &mut App) {
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
    app.tray_handler
        .icon
        .set_tooltip(tooltip)
        .expect("set tooltip value");
}

pub fn player_info(player_state: &Option<PlayerState>) -> Option<String> {
    if let Some(ref meta) = player_state {
        let play = if meta.state == "playing" {
            "▶"
        } else {
            "⏸"
        };
        let mut info = format!("{} {} - {}", play, meta.artist, meta.title);

        if info.len() > MAX_PLAYER_INFO_STRING_LENGTH {
            info.truncate(MAX_PLAYER_INFO_STRING_LENGTH);
            info.push_str("...");
        }

        Some(info)
    } else {
        None
    }
}
