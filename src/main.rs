mod assets;
mod key_handler;
mod menu_handler;
mod tray_handler;
mod window_handler;

use key_handler::KeyHandler;
use menu_handler::MenuHandler;
use tao::event_loop::{ControlFlow, EventLoop};
use tray_handler::TrayHandler;
use window_handler::WindowHandler;

fn main() -> wry::Result<()> {
    let mut event_loop = EventLoop::new();
    let key_handler = KeyHandler::new().register_keys();
    let window_handler = WindowHandler::new(&mut event_loop);
    let menu_handler = MenuHandler::new();
    let tray_handler = TrayHandler::new(&menu_handler);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        window_handler.try_recv(control_flow, event);
        key_handler.try_recv(&window_handler);
        tray_handler.try_recv(&window_handler, control_flow);
        menu_handler.try_recv(control_flow);
    })
}
