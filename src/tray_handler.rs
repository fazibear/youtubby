use tao::event_loop::ControlFlow;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconEventReceiver};

use crate::{assets, menu_handler::MenuHandler};

pub struct TrayHandler {
    _icon: TrayIcon,
    channel: &'static TrayIconEventReceiver,
}

impl TrayHandler {
    pub fn new(menu_handler: &MenuHandler) -> TrayHandler {
        let (icon_data, icon_width, icon_height) = assets::get_image(assets::ICON);
        let icon_data = Icon::from_rgba(icon_data, icon_width, icon_height).unwrap();

        let _icon = TrayIconBuilder::new()
            .with_tooltip("system-tray - tray icon library!")
            .with_menu(Box::new(menu_handler.menu.clone()))
            .with_icon(icon_data)
            .build()
            .unwrap();

        let channel = TrayIconEvent::receiver();

        Self { channel, _icon }
    }

    pub fn try_recv(&self, _control_flow: &mut ControlFlow) {
        if let Ok(_event) = self.channel.try_recv() {
            //todo!()
        }
    }
}
