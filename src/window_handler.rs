use tao::{
    dpi::{PhysicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::macos::WindowBuilderExtMacOS,
    window::{Icon, Window, WindowBuilder},
};
use wry::{WebView, WebViewBuilder};

pub struct WindowHandler {
    pub window: Window,
    pub webview: WebView,
}

impl WindowHandler {
    pub fn new(event_loop: &EventLoop<()>) -> WindowHandler {
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(1024, 2048))
            .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_window_icon(Some(Self::load_icon()))
            .build(event_loop)
            .unwrap();

        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))]
        let builder = WebViewBuilder::new(&window);

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        let builder = {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            let vbox = window.default_vbox().unwrap();
            WebViewBuilder::new_gtk(vbox)
        };

        let webview = builder
        .with_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .with_url("https://music.youtube.com")
        .with_devtools(true)
        .with_initialization_script("
          window.addEventListener('load', () => {
            console.log('tadam');
            document.head.insertAdjacentHTML(
              'beforeend',
              `<style>
                 html {
                    overflow: hidden !important;
                    height: 100% !important;
                 }
                 body {
                   overscroll-behavior: none !important;
                   height: 100% !important;
                   overflow: auto !important;
                 }
                 ytmusic-nav-bar {
                   margin-top: 15px;
                    }
                #guide-content {
                    padding-top: 15px;
                    }
              </style>`
            );
          });
        ")
        .with_drag_drop_handler(|e| {
            match e {
                wry::DragDropEvent::Enter { paths, position } => {
                    println!("DragEnter: {position:?} {paths:?} ")
                }
                wry::DragDropEvent::Over { position } => println!("DragOver: {position:?} "),
                wry::DragDropEvent::Drop { paths, position } => {
                    println!("DragDrop: {position:?} {paths:?} ")
                }
                wry::DragDropEvent::Leave => println!("DragLeave"),
                _ => {}
            }

            true
        })
        .build()
        .unwrap();

        WindowHandler { window, webview }
    }

    pub fn try_recv(&self, control_flow: &mut ControlFlow, event: Event<()>) {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    }
    fn load_icon() -> Icon {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png");
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open(path)
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
    }
}
