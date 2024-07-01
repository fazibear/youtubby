use crate::assets;
use crate::preferences::Preferences;
use anyhow::Result;
use muda::{AboutMetadata, CheckMenuItem, Menu, MenuId, MenuItem, PredefinedMenuItem, Submenu};

pub struct MenuHandler {
    pub menu: Menu,
    pub last_fm_info: MenuItem,
    pub last_fm_action: MenuItem,
}

impl MenuHandler {
    pub fn init(preferences: &Preferences) -> Result<Self> {
        let menu = Menu::new();

        let prefs = Submenu::new("Preferences", true);
        prefs.append_items(&[
            &CheckMenuItem::with_id(
                "always_on_top",
                "Always on top",
                true,
                preferences.always_on_top,
                None,
            ),
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
        ])?;

        let last_fm = Submenu::new("Last.fm", true);
        let last_fm_info = MenuItem::with_id("last_fm_info", "", false, None);
        let last_fm_action = MenuItem::with_id("last_fm_action", "", true, None);

        last_fm.append_items(&[
            &last_fm_info,
            &PredefinedMenuItem::separator(),
            &last_fm_action,
        ])?;

        let (icon_data, icon_width, icon_height) = assets::get_image(assets::LOGO)?;
        let icon = tray_icon::menu::Icon::from_rgba(icon_data, icon_width, icon_height)?;

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
        ])?;

        Ok(Self {
            menu,
            last_fm_info,
            last_fm_action,
        })
    }
}
