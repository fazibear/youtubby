use crate::state::State;
use crate::{assets, menu_handler::MenuHandler};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconEventReceiver};

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

    pub fn refresh(&self, state: &State) {
        let tray = if state.preferences.show_info_in_tray {
            Some(state.player_info.clone())
        } else {
            None
        };

        let tooltip = if state.preferences.show_info_in_tooltip {
            Some(state.player_info.clone())
        } else {
            None
        };

        self.icon.set_title(tray);
        self.icon.set_tooltip(tooltip).expect("set tooltip value");
    }
}
