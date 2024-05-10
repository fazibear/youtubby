use tao::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::macos::{EventLoopExtMacOS, WindowBuilderExtMacOS},
    window::{Icon, Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};

use crate::assets;

pub struct WindowHandler {
    pub window: Window,
    pub webview: WebView,
}

pub static WINDOW_WIDTH: u32 = 896;
pub static WINDOW_HEIGHT: u32 = 1536;

impl WindowHandler {
    pub fn new(event_loop: &mut EventLoop<()>) -> WindowHandler {
        event_loop.set_activation_policy(tao::platform::macos::ActivationPolicy::Accessory);

        let (icon, icon_width, icon_height) = assets::get_image(assets::ICON);
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_titlebar_buttons_hidden(true)
            .with_visible(false)
            .with_focused(true)
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
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Focused(false),
                ..
            } => self.window.set_visible(false),
            _ => {} //println!("{:?}", event),
        }
    }

    pub fn show_hide(&self, position: PhysicalPosition<f64>) {
        if self.window.is_visible() {
            self.window.set_visible(false);
            self.window.set_visible_on_all_workspaces(false);
        } else {
            self.window.set_outer_position(PhysicalPosition::new(
                position.x - (WINDOW_WIDTH / 2) as f64,
                100.,
            ));
            self.window.set_visible(true);
            self.window.set_visible_on_all_workspaces(true);
            self.window.set_focus();
        }
    }
}
