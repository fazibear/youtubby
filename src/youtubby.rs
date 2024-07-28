use crate::{
    events_handler,
    key_handler::KeyHandler,
    last_fm,
    menu_handler::MenuHandler,
    player_state::PlayerState,
    player_state_changed::PlayerStateChanged,
    preferences::Preferences,
    tray_handler::{self, TrayHandler},
    window_handler::{self, WindowHandler},
};
use anyhow::Result;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
};

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

        tray_handler::refresh(&mut app)?;
        window_handler::refresh(&mut app)?;
        last_fm::set_menu(&mut app);

        log::info!("Started");

        Ok(app)
    }

    pub fn tick(&mut self, event: &Event<PlayerStateChanged>, control_flow: &mut ControlFlow) {
        events_handler::callback(self, event, control_flow).expect("tick");
    }

    #[cfg(not(debug_assertions))]
    fn init_logger() -> Result<()> {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Off)
            .init()?;

        Ok(())
    }

    #[cfg(debug_assertions)]
    fn init_logger() -> Result<()> {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .init()?;
        Ok(())
    }
}
