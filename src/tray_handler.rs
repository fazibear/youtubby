use tray_icon::{
    menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

pub struct TrayHandler {
    menu: Menu,
    icon: TrayIcon,
}

impl TrayHandler {
    pub fn new() -> TrayHandler {
        let menu = Menu::new();

        let quit_i = MenuItem::new("Quit", true, None);

        menu.append_items(&[
            &PredefinedMenuItem::about(
                None,
                Some(AboutMetadata {
                    name: Some("tao".to_string()),
                    copyright: Some("Copyright tao".to_string()),
                    ..Default::default()
                }),
            ),
            &PredefinedMenuItem::separator(),
            &quit_i,
        ])
        .unwrap();

        let icon = TrayIconBuilder::new()
            .with_tooltip("system-tray - tray icon library!")
            .with_menu(Box::new(menu.clone()))
            .with_icon(Self::load_icon())
            .build()
            .unwrap();

        TrayHandler { menu, icon }
    }

    pub fn try_recv(&self) {}

    fn load_icon() -> Icon {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png");
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open(path)
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
    }
}
