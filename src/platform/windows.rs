pub mod platform {
    extern crate shell32;
    extern crate winapi;

    use crate::player_state_changed::PlayerStateChanged;
    use std::{ffi::CString, ptr};
    use tao::{
        event_loop::EventLoop,
        platform::windows::{EventLoopBuilderExtWindows, WindowExtWindows},
        window::{Window, WindowBuilder},
    };
    use wry::WebViewBuilder;

    pub fn window_builder() -> WindowBuilder {
        WindowBuilder::new()
    }

    pub fn webview_build(window: &Window, builder: WebViewBuilder) -> wry::Result<WebView> {
        builder.build(window)
    }

    pub fn init_event_loop(_event_loop: &mut EventLoop<PlayerStateChanged>) {}

    pub fn open_url(url: &str) {
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

    pub fn set_control_flow(control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Wait;
    }
}
