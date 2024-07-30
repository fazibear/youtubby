use crate::{platform::platform, player_state_changed::PlayerStateChanged, Youtubby};
use anyhow::Result;
use tao::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use wry::{http::Request, WebView};

pub static WINDOW_TITLE: &str = "Youtubby";
pub static WINDOW_WIDTH: u32 = 896;
pub static WINDOW_HEIGHT: u32 = 1536;
pub static USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";
pub static URL: &str = "https://music.youtube.com";

pub static WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
pub static WINDOW_MIN_SIZE: LogicalSize<u32> = LogicalSize::new(320, 0);

pub struct WindowHandler {
    pub window: Window,
    pub webview: WebView,
}

impl WindowHandler {
    pub fn init(event_loop: &mut EventLoop<PlayerStateChanged>) -> Result<Self> {
        platform::init_event_loop(event_loop);

        let window = platform::window_builder(
            WindowBuilder::new()
                .with_title(WINDOW_TITLE)
                .with_inner_size(WINDOW_SIZE)
                .with_min_inner_size(WINDOW_MIN_SIZE)
                .with_focused(true)
                .with_visible(false)
                .with_window_icon(crate::assets::window_icon().ok()),
        )
        .build(event_loop)?;

        let proxy = event_loop.create_proxy();

        let ipc = move |req: Request<String>| {
            if let Ok(event) = PlayerStateChanged::from_json_string(req.body()) {
                let _ = proxy.send_event(event);
            }
        };

        let builder = platform::webview_builder(&window);

        let webview = builder
            .with_user_agent(USER_AGENT)
            .with_url(URL)
            .with_devtools(true)
            .with_initialization_script(crate::assets::INIT_SCRIPT)
            .with_ipc_handler(ipc)
            .with_autoplay(true)
            .build()?;

        Ok(Self { window, webview })
    }

    pub fn show_hide(&self, position: PhysicalPosition<f64>) {
        if self.window.is_visible() {
            self.hide();
        } else {
            self.set_position(position);
            self.show();
        }
    }

    pub fn hide(&self) {
        self.window.set_visible(false);
        self.window.set_visible_on_all_workspaces(false);
    }

    pub fn set_position(&self, position: PhysicalPosition<f64>) {
        self.window.set_outer_position(PhysicalPosition::new(
            position.x - (WINDOW_WIDTH / 2) as f64,
            100.,
        ));
    }

    pub fn show(&self) {
        self.window.set_visible(true);
        self.window.set_visible_on_all_workspaces(true);
        self.window.set_focus();
    }

    pub fn open_url(&self, url: &str) {
        platform::open_url(url);
    }
}

pub fn refresh(app: &mut Youtubby) -> Result<()> {
    app.window_handler
        .window
        .set_always_on_top(app.preferences.always_on_top);
    Ok(())
}
