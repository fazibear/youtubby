pub mod platform {
    use tao::event_loop::EventLoop;
    use tao::platform::unix::{WindowBuilderExtUnix, WindowExtUnix};
    use tao::window::{Window, WindowBuilder};
    use wry::{WebViewBuilder, WebViewBuilderExtUnix};

    use crate::player_state_changed::PlayerStateChanged;

    pub fn window_builder(window_builder: WindowBuilder) -> WindowBuilder {
        window_builder
            //.with_decorations(false)
            .with_skip_taskbar(true)
            .with_visible_on_all_workspaces(true)
            .with_skip_taskbar(true)
    }

    pub fn webview_builder(window: &Window) -> WebViewBuilder {
        let vbox = window.default_vbox().expect("no default vbox");
        WebViewBuilder::new_gtk(vbox)
    }

    pub fn init_event_loop(_event_loop: &mut EventLoop<PlayerStateChanged>) {}

    pub fn open_url(url: &str) {
        let _ = std::process::Command::new("xdg-open").arg(url).output();
    }
}
