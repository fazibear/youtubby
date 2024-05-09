use tao::event_loop::ControlFlow;
use tray_icon::{
    ClickType, Icon, TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconEventReceiver, TrayIconId,
};

use crate::window_handler::WindowHandler;
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
            .with_id("0")
            .with_menu(Box::new(menu_handler.menu.clone()))
            .with_menu_on_left_click(false)
            .with_icon(icon_data)
            .build()
            .unwrap();

        let channel = TrayIconEvent::receiver();

        Self { channel, _icon }
    }

    pub fn try_recv(&self, window: &WindowHandler, _control_flow: &mut ControlFlow) {
        if let Ok(TrayIconEvent {
            id: TrayIconId(id),
            click_type: ClickType::Left,
            position,
            ..
        }) = self.channel.try_recv()
        {
            if &id == "0" {
                window.show_hide(position);
            }
        }
    }
}
