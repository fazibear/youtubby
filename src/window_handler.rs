use tao::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::macos::WindowBuilderExtMacOS,
    window::{Icon, Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};

use crate::assets;

pub struct WindowHandler {
    pub window: Window,
    pub webview: WebView,
}

impl WindowHandler {
    pub fn new(event_loop: &EventLoop<()>) -> WindowHandler {
        let (icon, icon_width, icon_height) = assets::get_image(assets::ICON);
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(1024, 2048))
            //.with_titlebar_transparent(true)
            //.with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_window_icon(Some(
                Icon::from_rgba(icon, icon_width, icon_height).unwrap(),
            ))
            .build(event_loop)
            .unwrap();

        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))]
        let builder = WebViewBuilder::new(&window);

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        let builder = {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            let vbox = window.default_vbox().unwrap();
            WebViewBuilder::new_gtk(vbox)
        };

        let webview = builder
        .with_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .with_url("https://music.youtube.com")
        .with_devtools(true)
        .with_initialization_script(assets::INIT_SCRIPT)
        .build()
        .unwrap();

        WindowHandler { window, webview }
    }

    pub fn try_recv(&self, control_flow: &mut ControlFlow, event: Event<()>) {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    }
}
