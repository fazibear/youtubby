mod assets;
mod events_handler;
mod key_handler;
mod menu_handler;
mod tray_handler;
mod window_handler;
mod state;

use events_handler::EventsHandler;
use key_handler::KeyHandler;
use menu_handler::MenuHandler;
use tao::event_loop::EventLoopBuilder;
use tray_handler::TrayHandler;
use window_handler::{UserEvent, WindowHandler};
use state::State;

fn main() -> wry::Result<()> {
    let mut state = State::load_or_default();
    let mut event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let mut window_handler = WindowHandler::new(&mut event_loop);
    let mut key_handler = KeyHandler::new().register_keys();
    let mut menu_handler = MenuHandler::new();
    let mut tray_handler = TrayHandler::new(&menu_handler);

    event_loop.run(move |event, _, control_flow| {
        EventsHandler::callback(
            &event,
            control_flow,
            &mut window_handler,
            &mut key_handler,
            &mut menu_handler,
            &mut tray_handler,
            &mut state,
        );
    })
}
