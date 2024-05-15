use serde::{Deserialize, Serialize};
use tao::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event_loop::EventLoop,
    window::{Icon, Window, WindowBuilder},
};
use wry::{http::Request, WebView, WebViewBuilder};

use crate::assets;

use tao::platform::windows::{EventLoopBuilderExtWindows, WindowExtWindows};

use crate::window_handler::{PlayerState, UserEvent, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct WindowHandler {
    pub window: Window,
    pub webview: WebView,
}

impl WindowHandler {
    pub fn new(event_loop: &mut EventLoop<UserEvent>) -> WindowHandler {
        #[cfg(target_os = "macos")]
        event_loop.set_activation_policy(tao::platform::macos::ActivationPolicy::Accessory);

        let (icon, icon_width, icon_height) = assets::get_image(assets::ICON);
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .with_min_inner_size(LogicalSize::new(320, 0))
            //.with_titlebar_transparent(true)
            //.with_fullsize_content_view(true)
            //.with_title_hidden(true)
            //.with_titlebar_buttons_hidden(true)
            //.with_visible(false)
            .with_focused(true)
            .with_window_icon(Some(
                Icon::from_rgba(icon, icon_width, icon_height).unwrap(),
            ))
            .build(event_loop)
            .unwrap();

        let builder = WebViewBuilder::new(&window);
        let proxy = event_loop.create_proxy();

        let ipc = move |req: Request<String>| {
            let p: PlayerState = serde_json::from_str(req.body()).unwrap();
            proxy.send_event(UserEvent::PlayerStateUpdated(p)).unwrap();
        };

        let webview = builder
        .with_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .with_url("https://music.youtube.com")
        .with_devtools(true)
        .with_initialization_script(assets::INIT_SCRIPT)
        .with_ipc_handler(ipc)
        .build()
        .unwrap();

        WindowHandler { window, webview }
    }
}
