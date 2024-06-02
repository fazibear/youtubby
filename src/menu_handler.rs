use crate::assets;
use crate::preferences::Preferences;
use muda::{
    AboutMetadata, CheckMenuItem, Menu, MenuEvent, MenuEventReceiver, MenuId, MenuItem,
    PredefinedMenuItem, Submenu,
};

pub struct MenuHandler {
    pub channel: &'static MenuEventReceiver,
    pub menu: Menu,
    pub last_fm: MenuItem,
}

impl MenuHandler {
    pub fn new(preferences: &Preferences) -> Self {
        let last_fm = MenuItem::with_id(
            MenuId::new("lastfm_auth"),
            "Authenticate Last.fm",
            true,
            None,
        );

        let menu = Menu::new();

        let prefs = Submenu::new("Preferences", true);
        prefs
            .append_items(&[
                &CheckMenuItem::with_id(
                    "hide_unfocused_window",
                    "Hide unfocused window",
                    true,
                    preferences.hide_unfocused_window,
                    None,
                ),
                &CheckMenuItem::with_id(
                    "show_info_in_tray",
                    "Show info in tray",
                    true,
                    preferences.show_info_in_tray,
                    None,
                ),
                &CheckMenuItem::with_id(
                    "show_info_in_tooltip",
                    "Show info in tooltip",
                    true,
                    preferences.show_info_in_tooltip,
                    None,
                ),
            ])
            .unwrap();

        let (icon_data, icon_width, icon_height) = assets::get_image(assets::LOGO);
        let icon = tray_icon::menu::Icon::from_rgba(icon_data, icon_width, icon_height).unwrap();

        let about = AboutMetadata {
            name: Some(env!("CARGO_PKG_NAME").to_string()),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
            copyright: Some(format!("{} 󰗦  2024", env!("CARGO_PKG_AUTHORS"))),
            website: Some(env!("CARGO_PKG_HOMEPAGE").to_string()),
            icon: Some(icon),
            ..Default::default()
        };

        menu.append_items(&[
            &MenuItem::with_id(MenuId::new("show"), "Show", true, None),
            &MenuItem::with_id(MenuId::new("playstop"), "Play/Stop", true, None),
            &MenuItem::with_id(MenuId::new("next"), "Next Song", true, None),
            &MenuItem::with_id(MenuId::new("prev"), "Previous Song", true, None),
            &PredefinedMenuItem::separator(),
            &prefs,
            &last_fm,
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::about(None, Some(about)),
            &PredefinedMenuItem::separator(),
            &MenuItem::with_id(MenuId::new("quit"), "Quit Youtubby", true, None),
        ])
            .unwrap();

        let channel = MenuEvent::receiver();

        Self {
            channel,
            menu,
            last_fm,
        }
    }
}
