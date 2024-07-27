pub mod platform {
    extern crate shell32;
    extern crate winapi;

    use std::ffi::CString;
    use std::ptr;
    use tao::event_loop::EventLoop;
    use tao::platform::windows::{EventLoopBuilderExtWindows, WindowExtWindows};
    use tao::window::{Window, WindowBuilder};
    use wry::WebViewBuilder;

    use crate::player_state_changed::PlayerStateChanged;

    pub fn window_builder(window_builder: WindowBuilder) -> WindowBuilder {
        window_builder
    }

    pub fn webview_builder(window: &Window) -> WebViewBuilder {
        WebViewBuilder::new(&window);
    }

    pub fn init_event_loop(_event_loop: &mut EventLoop<PlayerStateChanged>) {}

    pub unsafe fn open_url(url: &str) {
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
