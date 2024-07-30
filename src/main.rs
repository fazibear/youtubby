mod assets;
mod events_handler;
mod key_handler;
mod last_fm;
mod menu_handler;
mod platform;
mod player_state;
mod player_state_changed;
mod preferences;
mod tray_handler;
mod window_handler;
mod youtubby;

use anyhow::Result;
use youtubby::Youtubby;

fn main() -> Result<()> {
    let mut event_loop = Youtubby::build_event_loop();
    let mut app = Youtubby::new(&mut event_loop)?;

    event_loop.run(move |event, _, control_flow| app.tick(&event, control_flow))
}
