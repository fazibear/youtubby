pub mod platform {
    use crate::player_state_changed::PlayerStateChanged;
    use tao::{
        event_loop::EventLoop,
        platform::unix::{WindowBuilderExtUnix, WindowExtUnix},
        window::{Window, WindowBuilder},
    };
    use wry::{WebViewBuilder, WebViewBuilderExtUnix};

    pub fn window_builder() -> WindowBuilder {
        WindowBuilder::new()
            //.with_decorations(false)
            .with_skip_taskbar(true)
            .with_visible_on_all_workspaces(true)
            .with_skip_taskbar(true)
    }

    pub fn webview_build(window: &Window, builder: WebViewBuilder) -> wry::Result<WebView> {
        let vbox = window.default_vbox().expect("no default vbox");
        builder.build_gtk(vbox)
    }

    pub fn init_event_loop(_event_loop: &mut EventLoop<PlayerStateChanged>) {}

    pub fn open_url(url: &str) {
        let _ = std::process::Command::new("xdg-open").arg(url).output();
    }

    pub fn set_control_flow(control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Poll;
    }
}
