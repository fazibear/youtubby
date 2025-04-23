mod youtubby;

use anyhow::{Context, Result};
use youtubby::Youtubby;

fn main() -> Result<()> {
    let mut event_loop = Youtubby::build_event_loop()?;
    let mut app = Youtubby::new(&mut event_loop)?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    event_loop.run_app(&mut app).context("Failed to run app")
}
