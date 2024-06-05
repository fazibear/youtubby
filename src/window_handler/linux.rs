use crate::window_handler::{UserEvent, URL, USER_AGENT, WINDOW_MIN_SIZE, WINDOW_SIZE};
use crate::{assets, player_state::PlayerState};
use anyhow::{Context, Result};
use tao::platform::unix::{WindowBuilderExtUnix, WindowExtUnix};
use tao::{
    event_loop::EventLoop,
    window::{Icon, Window, WindowBuilder},
};
use wry::{http::Request, WebView, WebViewBuilder, WebViewBuilderExtUnix};

pub struct WindowHandler {
    pub window: Window,
    pub webview: WebView,
}

impl WindowHandler {
    pub fn init(event_loop: &mut EventLoop<UserEvent>) -> Result<WindowHandler> {
        let (icon, icon_width, icon_height) = assets::get_image(assets::ICON)?;
        let window = WindowBuilder::new()
            .with_title("Youtubby")
            .with_inner_size(WINDOW_SIZE)
            .with_min_inner_size(WINDOW_MIN_SIZE)
            //.with_decorations(false)
            .with_visible_on_all_workspaces(true)
            .with_skip_taskbar(true)
            .with_visible(false)
            .with_focused(true)
            .with_window_icon(Some(Icon::from_rgba(icon, icon_width, icon_height)?))
            .build(event_loop)?;

        let builder = {
            let vbox = window.default_vbox().context("no default vbox");
            WebViewBuilder::new_gtk(vbox)
        };
        let proxy = event_loop.create_proxy();

        let ipc = move |req: Request<String>| {
            let _ = proxy.send_event(UserEvent::PlayerStateUpdated(
                PlayerState::from_json_string(req.body()).expect("failed parse player state"),
            ));
        };

        let webview = builder
            .with_user_agent(USER_AGENT)
            .with_url(URL)
            .with_devtools(true)
            .with_initialization_script(assets::INIT_SCRIPT)
            .with_ipc_handler(ipc)
            .with_autoplay(true)
            .build()?;

        Ok(WindowHandler { window, webview })
    }

    pub fn open_url(&self, url: &str) {
        let _ = std::process::Command::new("xdg-open").arg(url).output();
    }
}
