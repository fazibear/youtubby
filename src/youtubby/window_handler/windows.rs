use crate::Youtubby::{
    assets,
    player_state_changed::PlayerStateChanged,
    window_handler::{URL, USER_AGENT, WINDOW_MIN_SIZE, WINDOW_SIZE},
};
use anyhow::Result;
use winit::platform::windows::{WindowAttributesExtWindows, WindowExtWindows};
use winit::window::{Icon, Window, WindowAttributes};
use wry::{http::Request, WebView, WebViewBuilder};

pub struct WindowHandler {
    pub window: Window,
    pub webview: WebView,
}

impl WindowHandler {
    pub fn init(event_loop: &mut YoutubbyEventLoop) -> Result<WindowHandler> {
        let (icon, icon_width, icon_height) = assets::get_image(assets::ICON)?;
        let window = WindowBuilder::new()
            .with_title("Youtubby")
            .with_inner_size(WINDOW_SIZE)
            .with_min_inner_size(WINDOW_MIN_SIZE)
            .with_visible(false)
            .with_focused(true)
            .with_window_icon(Some(Icon::from_rgba(icon, icon_width, icon_height)?))
            .build(event_loop)?;

        let proxy = event_loop.create_proxy();

        let ipc = move |req: Request<String>| {
            if let Ok(event) = PlayerStateChanged::from_json_string(req.body()) {
                let _ = proxy.send_event(event);
            }
        };

        let webview = WebViewBuilder::new()
            .with_user_agent(USER_AGENT)
            .with_url(URL)
            .with_devtools(true)
            .with_initialization_script(assets::INIT_SCRIPT)
            .with_ipc_handler(ipc)
            .with_autoplay(true)
            .build(&window)?;

        Ok(WindowHandler { window, webview })
    }

    pub fn open_url(&self, url: &str) {
        extern crate shell32;
        extern crate winapi;

        use std::ffi::CString;
        use std::ptr;

        unsafe {
            shell32::ShellExecuteA(
                ptr::null_mut(),
                CString::new("open").unwrap().as_ptr(),
                CString::new(url.replace("\n", "%0A")).unwrap().as_ptr(),
                ptr::null(),
                ptr::null(),
                1,
            );
        }
    }
}
