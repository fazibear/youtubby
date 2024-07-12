mod assets;
mod events_handler;
mod key_handler;
mod last_fm;
mod menu_handler;
mod player_state;
mod player_state_changed;
mod preferences;
mod tray_handler;
mod window_handler;

use anyhow::Result;
use key_handler::KeyHandler;
use menu_handler::MenuHandler;
use player_state::PlayerState;
use player_state_changed::PlayerStateChanged;
use preferences::Preferences;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
};
use tray_handler::TrayHandler;
use window_handler::WindowHandler;

type YoutubbyEventLoop = EventLoop<PlayerStateChanged>;

pub struct Youtubby {
    pub preferences: Preferences,
    pub window_handler: WindowHandler,
    pub key_handler: KeyHandler,
    pub menu_handler: MenuHandler,
    pub tray_handler: TrayHandler,
    pub player_state: PlayerState,
    pub cache: HashMap<String, String>,
}

impl Youtubby {
    pub fn build_event_loop() -> YoutubbyEventLoop {
        EventLoopBuilder::<PlayerStateChanged>::with_user_event().build()
    }

    pub fn new(event_loop: &mut YoutubbyEventLoop) -> Result<Self> {
        Self::init_logger()?;

        let preferences = Preferences::load()?;
        let window_handler = WindowHandler::init(event_loop)?;
        let key_handler = KeyHandler::init()?.register_keys();
        let menu_handler = MenuHandler::init(&preferences)?;
        let tray_handler = TrayHandler::init(&menu_handler)?;
        let player_state = PlayerState::new();
        let cache = HashMap::new();

        let mut app = Self {
            window_handler,
            key_handler,
            menu_handler,
            tray_handler,
            preferences,
            player_state,
            cache,
        };

        last_fm::set_menu(&mut app);

        log::info!("Started");

        Ok(app)
    }

    pub fn tick(&mut self, event: &Event<PlayerStateChanged>, control_flow: &mut ControlFlow) {
        events_handler::callback(self, event, control_flow).expect("tick");
    }

    #[cfg(build = "release")]
    fn init_logger() -> Result<()> {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Off)
            .init()?;

        Ok(())
    }

    #[cfg(not(build = "release"))]
    fn init_logger() -> Result<()> {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .init()?;
        Ok(())
    }
}
