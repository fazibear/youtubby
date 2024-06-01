use crate::assets;
use crate::state::State;
use muda::{
    AboutMetadata, CheckMenuItem, Menu, MenuEvent, MenuEventReceiver, MenuId, MenuItem,
    PredefinedMenuItem, Submenu,
};

pub struct MenuHandler {
    pub channel: &'static MenuEventReceiver,
    pub menu: Menu,
}

impl MenuHandler {
    pub fn new(state: &State) -> Self {
        let menu = Menu::new();

        let (icon_data, icon_width, icon_height) = assets::get_image(assets::LOGO);
        let icon = tray_icon::menu::Icon::from_rgba(icon_data, icon_width, icon_height).unwrap();

        let prefs = Submenu::new("Preferences", true);
        prefs
            .append_items(&[
                &CheckMenuItem::with_id(
                    "hide_unfocused_window",
                    "Hide unfocused window",
                    true,
                    state.preferences.hide_unfocused_window,
                    None,
                ),
                &CheckMenuItem::with_id(
                    "show_info_in_tray",
                    "Show info in tray",
                    true,
                    state.preferences.show_info_in_tray,
                    None,
                ),
                &CheckMenuItem::with_id(
                    "show_info_in_tooltip",
                    "Show info in tooltip",
                    true,
                    state.preferences.show_info_in_tooltip,
                    None,
                ),
            ])
            .unwrap();

        menu.append_items(&[
            &MenuItem::with_id(MenuId::new("show"), "Show", true, None),
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

        Self { channel, menu }
    }
}
