use muda::{
    AboutMetadata, CheckMenuItem, Menu, MenuEvent, MenuEventReceiver, MenuId, MenuItem,
    PredefinedMenuItem, Submenu,
};
use tao::event::Event;
use tao::event_loop::{ControlFlow, EventLoopProxy};

use crate::assets;
use crate::window_handler::UserEvent;

pub struct MenuHandler {
    pub channel: &'static MenuEventReceiver,
    pub menu: Menu,
    pub event_loop_proxy: &'static EventLoopProxy<UserEvent>,
}

impl MenuHandler {
    pub fn new(event_loop_proxy: &EventLoopProxy<UserEvent>) -> Self {
        let menu = Menu::new();

        let (icon_data, icon_width, icon_height) = assets::get_image(assets::LOGO);
        let icon = tray_icon::menu::Icon::from_rgba(icon_data, icon_width, icon_height).unwrap();

        let prefs = Submenu::new("Preferences", true);
        prefs
            .append_items(&[
                &CheckMenuItem::with_id("check-custom-1", "Check Custom 1", true, true, None),
                &CheckMenuItem::with_id("check-custom-2", "Check Custom 1", true, true, None),
                &CheckMenuItem::with_id("check-custom-3", "Check Custom 1", true, true, None),
                &CheckMenuItem::with_id("check-custom-4", "Check Custom 1", true, true, None),
                &CheckMenuItem::with_id("check-custom-5", "Check Custom 1", true, true, None),
            ])
            .unwrap();

        menu.append_items(&[
            &MenuItem::with_id(MenuId::new("playstop"), "Play/Stop", true, None),
            &MenuItem::with_id(MenuId::new("next"), "Next Song", true, None),
            &MenuItem::with_id(MenuId::new("prev"), "Previous Song", true, None),
            &PredefinedMenuItem::separator(),
            &prefs,
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
            &MenuItem::with_id(MenuId::new("quit"), "Quit Youtubby", true, None),
        ])
        .unwrap();

        let channel = MenuEvent::receiver();

        Self {
            channel,
            menu,
            event_loop_proxy,
        }
    }

    pub fn resend(&self) {
        if let Ok(event) = self.channel.try_recv() {
            self.event_loop_proxy
                .send_event(UserEvent::MenuEvent(event))
                .unwrap();
        }
    }

    pub fn on_event(&self, event: &Event<UserEvent>) {}
}
