use crate::window_handler::{UserEvent, URL, USER_AGENT, WINDOW_MIN_SIZE, WINDOW_SIZE};
use crate::{assets, player_state::PlayerState};
use tao::platform::windows::{EventLoopBuilderExtWindows, WindowExtWindows};
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
    pub fn new(event_loop: &mut EventLoop<UserEvent>) -> WindowHandler {
        let (icon, icon_width, icon_height) = assets::get_image(assets::ICON);
        let window = WindowBuilder::new()
            .with_inner_size(WINDOW_SIZE)
            .with_min_inner_size(WINDOW_MIN_SIZE)
            .with_visible(false)
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
            .with_user_agent(USER_AGENT)
            .with_url(URL)
            .with_devtools(true)
            .with_initialization_script(assets::INIT_SCRIPT)
            .with_ipc_handler(ipc)
            .with_autoplay(true)
            .build()
            .unwrap();

        WindowHandler { window, webview }
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
                winapi::SW_SHOWNORMAL,
            );
        }
    }
}
