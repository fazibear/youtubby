use tray_icon::{
    menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

use crate::assets;

pub struct TrayHandler {
    menu: Menu,
    icon: TrayIcon,
}

impl TrayHandler {
    pub fn new() -> TrayHandler {
        let menu = Menu::new();

        let quit_i = MenuItem::new("Quit", true, None);
        let (icon_data, icon_width, icon_height) = assets::get_image(assets::LOGO);
        let icon = tray_icon::menu::Icon::from_rgba(icon_data, icon_width, icon_height).unwrap();

        menu.append_items(&[
            &PredefinedMenuItem::about(
                None,
                Some(AboutMetadata {
                    name: Some("Youtubby".to_string()),
                    copyright: Some("Copyright 2024".to_string()),
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

        TrayHandler { menu, icon }
    }

    pub fn try_recv(&self) {}
}
