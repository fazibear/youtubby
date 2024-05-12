use muda::{
    AboutMetadata, CheckMenuItem, Menu, MenuEvent, MenuEventReceiver, MenuId, MenuItem,
    PredefinedMenuItem,
};
use tao::event_loop::ControlFlow;

use crate::assets;

pub struct MenuHandler {
    pub channel: &'static MenuEventReceiver,
    pub menu: Menu,
}

impl MenuHandler {
    pub fn new() -> Self {
        let menu = Menu::new();

        let quit_i = MenuItem::with_id(MenuId::new("quit"), "Quit", true, None);

        let (icon_data, icon_width, icon_height) = assets::get_image(assets::LOGO);
        let icon = tray_icon::menu::Icon::from_rgba(icon_data, icon_width, icon_height).unwrap();

        menu.append_items(&[
            &CheckMenuItem::with_id("check-custom-1", "Check Custom 1", true, true, None),
            &PredefinedMenuItem::about(
                None,
                Some(AboutMetadata {
                    name: Some("Youtubby".to_string()),
                    version: Some("0.1".to_string()),
                    copyright: Some("Copyright 2024".to_string()),
                    website: Some("https://youtubby.fazibear.me".to_string()),
                    icon: Some(icon),
                    ..Default::default()
                }),
            ),
            &PredefinedMenuItem::separator(),
            &quit_i,
        ])
        .unwrap();

        let channel = MenuEvent::receiver();

        Self { channel, menu }
    }

    pub fn try_recv(&self, control_flow: &mut ControlFlow) {
        if let Ok(event) = self.channel.try_recv() {
            if event.id == "quit" {
                *control_flow = ControlFlow::Exit;
            }
        }
    }
}
