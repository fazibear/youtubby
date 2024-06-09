use crate::events_handler;
use crate::key_handler::KeyHandler;
use crate::last_fm;
use crate::menu_handler::MenuHandler;
use crate::player_state::PlayerState;
use crate::player_state_changed::PlayerStateChanged;
use crate::preferences::Preferences;
use crate::tray_handler::TrayHandler;
use crate::window_handler::WindowHandler;
use anyhow::Result;
use simple_logger::SimpleLogger;
use std::collections::HashMap;

use tao::event::Event;
use tao::event_loop::ControlFlow;
use tao::event_loop::EventLoop;

pub struct App {
    pub preferences: Preferences,
    pub window_handler: WindowHandler,
    pub key_handler: KeyHandler,
    pub menu_handler: MenuHandler,
    pub tray_handler: TrayHandler,
    pub player_state: PlayerState,
    pub cache: HashMap<String, String>,
}

impl App {
    pub fn new(event_loop: &mut EventLoop<PlayerStateChanged>) -> Result<App> {
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
