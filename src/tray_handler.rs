use crate::window_handler::PlayerState;
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

    pub fn set_title(&self, meta: &PlayerState) {
       self.icon.set_title(Self::song_info(meta));
    }

    pub fn set_tooltip(&self, meta: &PlayerState) {
       self.icon.set_tooltip(Self::song_info(meta)).unwrap();

    }

    fn song_info(meta: &PlayerState) -> Option<String> {
       let play = if meta.state == "playing" {
           "▶"
       } else {
           "⏸"
       };
       Some(format!("{} {} - {}", play, meta.artist, meta.title))
    }
}
