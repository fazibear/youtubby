use serde::{Deserialize, Serialize};
use tao::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Icon, Window, WindowBuilder},
};
use wry::{http::Request, WebView, WebViewBuilder};

use crate::assets;

#[cfg(target_os = "macos")]
use tao::platform::macos::{EventLoopExtMacOS, WindowBuilderExtMacOS};
#[cfg(target_os = "linux")]
use tao::platform::unix::WindowExtUnix;
#[cfg(target_os = "windows")]
use tao::platform::windows::{EventLoopBuilderExtWindows, WindowExtWindows};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerState {
    title: String,
    artist: String,
    album: String,
    state: String,
}

#[derive(Debug)]
pub enum UserEvent {
    PlayerStateUpdated(PlayerState),
}

pub struct WindowHandler {
    pub window: Window,
    pub webview: WebView,
}

pub static WINDOW_WIDTH: u32 = 896;
pub static WINDOW_HEIGHT: u32 = 1536;

impl WindowHandler {
    pub fn new(event_loop: &mut EventLoop<UserEvent>) -> WindowHandler {
        #[cfg(target_os = "macos")]
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

    pub fn try_recv(&self, control_flow: &mut ControlFlow, event: Event<UserEvent>) {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Focused(false) => self.window.set_visible(false),
                _ => {}
            }
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
