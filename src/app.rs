use crate::events_handler;
use crate::key_handler::KeyHandler;
use crate::menu_handler::MenuHandler;
use crate::player_state::PlayerState;
use crate::preferences::Preferences;
use crate::tray_handler::TrayHandler;
use crate::window_handler::{UserEvent, WindowHandler};

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
}

impl App {
    pub fn new(event_loop: &mut EventLoop<UserEvent>) -> App {
        let preferences = Preferences::load();
        let window_handler = WindowHandler::new(event_loop);
        let key_handler = KeyHandler::new().register_keys();
        let menu_handler = MenuHandler::new(&preferences);
        let tray_handler = TrayHandler::new(&menu_handler);
        let player_state = PlayerState::new();

        Self {
            window_handler,
            key_handler,
            menu_handler,
            tray_handler,
            preferences,
            player_state,
        }
    }

    pub fn tick(&mut self, event: &Event<UserEvent>, control_flow: &mut ControlFlow) {
        events_handler::callback(self, event, control_flow);
    }
}
