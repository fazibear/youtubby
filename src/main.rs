mod assets;
mod key_handler;
mod tray_handler;
mod window_handler;

use key_handler::KeyHandler;
use tao::event_loop::EventLoop;
use tray_handler::TrayHandler;
use window_handler::WindowHandler;

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let key_handler = KeyHandler::new().register_keys();
    let window_handler = WindowHandler::new(&event_loop);
    let tray_handler = TrayHandler::new();

    event_loop.run(move |event, _, control_flow| {
        key_handler.try_recv(&window_handler);
        window_handler.try_recv(control_flow, event);
        tray_handler.try_recv(control_flow);
    })
}
