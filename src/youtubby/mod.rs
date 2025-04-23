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

use anyhow::{Context, Result};
use key_handler::KeyHandler;
use menu_handler::MenuHandler;
use player_state::PlayerState;
use player_state_changed::PlayerStateChanged;
use preferences::Preferences;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use tray_handler::TrayHandler;
use window_handler::WindowHandler;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
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

impl ApplicationHandler<PlayerStateChanged> for Youtubby {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        events_handler::handle_window_events(self, &event, event_loop).unwrap();
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: PlayerStateChanged) {
        events_handler::handle_user_events(self, &event, event_loop).unwrap();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        events_handler::handle_menu_events(self, event_loop).unwrap();
        events_handler::handle_tray_events(self, event_loop).unwrap();
        events_handler::handle_hotkey_events(self, event_loop).unwrap();
    }
}

impl Youtubby {
    pub fn build_event_loop() -> Result<YoutubbyEventLoop> {
        EventLoop::<PlayerStateChanged>::with_user_event()
            .build()
            .context("Failed to build event loop")
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

    fn init_logger() -> Result<()> {
        SimpleLogger::new()
            .with_level(
                #[cfg(not(debug_assertions))]
                {
                    log::LevelFilter::Off
                },
                #[cfg(debug_assertions)]
                {
                    log::LevelFilter::Info
                },
            )
            .init()?;

        Ok(())
    }
}
