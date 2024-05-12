use tao::event::Event;
use tao::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use tray_icon::{
    ClickType, Icon, TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconEventReceiver, TrayIconId,
};

use crate::window_handler::{UserEvent, WindowHandler};
use crate::{assets, menu_handler::MenuHandler};

pub struct TrayHandler {
    icon: TrayIcon,
    channel: &'static TrayIconEventReceiver,
    event_loop_proxy: &'static EventLoopProxy<UserEvent>,
}

impl TrayHandler {
    pub fn new(
        event_loop_proxy: &EventLoopProxy<UserEvent>,
        menu_handler: &MenuHandler,
    ) -> TrayHandler {
        let (icon_data, icon_width, icon_height) = assets::get_image(assets::ICON);
        let icon_data = Icon::from_rgba(icon_data, icon_width, icon_height).unwrap();

        let icon = TrayIconBuilder::new()
            .with_id("0")
            .with_menu(Box::new(menu_handler.menu.clone()))
            .with_menu_on_left_click(false)
            .with_icon(icon_data)
            .build()
            .unwrap();

        let channel = TrayIconEvent::receiver();

        Self {
            channel,
            icon,
            event_loop_proxy,
        }
    }

    pub fn resend(&self) {
        if let Ok(event) = self.channel.try_recv() {
            self.event_loop_proxy
                .send_event(UserEvent::TrayIconEvent(event))
                .unwrap();
        }
    }

    pub fn on_event(&self, event: &Event<UserEvent>) {}
}
