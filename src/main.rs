mod app;
mod assets;
mod events_handler;
mod key_handler;
mod last_fm;
mod menu_handler;
mod player_state;
mod preferences;
mod tray_handler;
mod window_handler;

use anyhow::Result;
use tao::event_loop::EventLoopBuilder;
use window_handler::UserEvent;

fn main() -> Result<()> {
    let mut event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let mut app = app::App::new(&mut event_loop)?;

    event_loop.run(move |event, _, control_flow| app.tick(&event, control_flow))
}
