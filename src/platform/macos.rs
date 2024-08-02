pub mod platform {
    use crate::player_state_changed::PlayerStateChanged;
    use tao::{
        event_loop::{ControlFlow, EventLoop},
        platform::macos::{EventLoopExtMacOS, WindowBuilderExtMacOS},
        window::{Window, WindowBuilder},
    };
    use wry::WebViewBuilder;

    pub fn window_builder(window_builder: WindowBuilder) -> WindowBuilder {
        window_builder
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_titlebar_buttons_hidden(true)
    }

    pub fn webview_builder(window: &Window) -> WebViewBuilder {
        WebViewBuilder::new(window)
    }

    pub fn init_event_loop(event_loop: &mut EventLoop<PlayerStateChanged>) {
        event_loop.set_activation_policy(tao::platform::macos::ActivationPolicy::Accessory);
    }

    pub fn open_url(url: &str) {
        let _ = std::process::Command::new("open").arg(url).output();
    }

    pub fn set_control_flow(control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Wait;
    }
}
