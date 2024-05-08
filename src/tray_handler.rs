use muda::MenuId;
use tao::event_loop::ControlFlow;
use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuEventReceiver, MenuItem, PredefinedMenuItem},
    Icon, TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconEventReceiver,
};

use crate::assets;

pub struct TrayHandler {
    icon: TrayIcon,
    menu_channel: &'static MenuEventReceiver,
    tray_channel: &'static TrayIconEventReceiver,
}

impl TrayHandler {
    pub fn new() -> TrayHandler {
        let menu = Menu::new();

        let quit_i = MenuItem::with_id(MenuId::new("quit"), "Quit", true, None);

        let (icon_data, icon_width, icon_height) = assets::get_image(assets::LOGO);
        let icon = tray_icon::menu::Icon::from_rgba(icon_data, icon_width, icon_height).unwrap();

        menu.append_items(&[
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

        let (icon_data, icon_width, icon_height) = assets::get_image(assets::ICON);

        let icon_data = Icon::from_rgba(icon_data, icon_width, icon_height).unwrap();

        let icon = TrayIconBuilder::new()
            .with_tooltip("system-tray - tray icon library!")
            .with_menu(Box::new(menu.clone()))
            .with_icon(icon_data)
            .build()
            .unwrap();

        let menu_channel = MenuEvent::receiver();
        let tray_channel = TrayIconEvent::receiver();

        TrayHandler {
            menu_channel,
            tray_channel,
            icon,
        }
    }

    pub fn try_recv(&self, control_flow: &mut ControlFlow) {
        if let Ok(event) = self.menu_channel.try_recv() {
            if event.id == "quit" {
                *control_flow = ControlFlow::Exit;
            }
        }

        if let Ok(event) = self.tray_channel.try_recv() {
            //todo!()
        }
    }
}
