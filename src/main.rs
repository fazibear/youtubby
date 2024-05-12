mod assets;
mod key_handler;
mod menu_handler;
mod tray_handler;
mod window_handler;

use key_handler::KeyHandler;
use menu_handler::MenuHandler;
use muda::MenuEvent;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_handler::TrayHandler;
use window_handler::WindowHandler;

use self::window_handler::UserEvent;

fn main() -> wry::Result<()> {
    let mut event_loop = EventLoopBuilder::<window_handler::UserEvent>::with_user_event().build();
    let event_loop_proxy = event_loop.create_proxy();
    let window_handler = WindowHandler::new(&mut event_loop);

    let key_handler = KeyHandler::new(&event_loop_proxy);
    let menu_handler = MenuHandler::new(&event_loop_proxy);
    let tray_handler = TrayHandler::new(&event_loop_proxy, &menu_handler);

    event_loop.run(move |event, _, mut control_flow| {
        handle_control_flow(&event, &mut control_flow);

        menu_handler.resend();
        key_handler.resend();
        tray_handler.resend();

        tray_handler.on_event(&event);
        key_handler.on_event(&event);
        menu_handler.on_event(&event);
        window_handler.on_event(&event)
    })
}

fn handle_control_flow(event: &Event<UserEvent>, control_flow: &mut ControlFlow) {
    *control_flow = ControlFlow::Wait;
    match event {
        Event::UserEvent(UserEvent::MenuEvent(event)) if event.id == "quit" => {
            *control_flow = ControlFlow::Exit
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        _ => {}
    }
}
