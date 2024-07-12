mod youtubby;

use anyhow::Result;
use youtubby::Youtubby;

fn main() -> Result<()> {
    let mut event_loop = Youtubby::build_event_loop();
    let mut app = Youtubby::new(&mut event_loop)?;

    event_loop.run(move |event, _, control_flow| app.tick(&event, control_flow))
}
