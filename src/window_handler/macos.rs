use crate::window_handler::{UserEvent, URL, USER_AGENT, WINDOW_MIN_SIZE, WINDOW_SIZE};
use crate::{assets, player_state::PlayerState};
use anyhow::Result;
use tao::platform::macos::{EventLoopExtMacOS, WindowBuilderExtMacOS};
use tao::{
    event_loop::EventLoop,
    window::{Icon, Window, WindowBuilder},
};
use wry::{http::Request, WebView, WebViewBuilder};

pub struct WindowHandler {
    pub window: Window,
    pub webview: WebView,
}

impl WindowHandler {
    pub fn init(event_loop: &mut EventLoop<UserEvent>) -> Result<WindowHandler> {
        event_loop.set_activation_policy(tao::platform::macos::ActivationPolicy::Accessory);

        let (icon, icon_width, icon_height) = assets::get_image(assets::ICON)?;
        let window = WindowBuilder::new()
            .with_title("Youtubby")
            .with_inner_size(WINDOW_SIZE)
            .with_min_inner_size(WINDOW_MIN_SIZE)
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_titlebar_buttons_hidden(true)
            .with_visible(false)
            .with_focused(true)
            .with_window_icon(Some(Icon::from_rgba(icon, icon_width, icon_height)?))
            .build(event_loop)?;

        let builder = WebViewBuilder::new(&window);

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
        let _ = std::process::Command::new("open").arg(url).output();
    }
}
