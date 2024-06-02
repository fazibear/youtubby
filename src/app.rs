use crate::events_handler;
use crate::key_handler::KeyHandler;
use crate::menu_handler::MenuHandler;
use crate::preferences::Preferences;
use crate::tray_handler::TrayHandler;
use crate::window_handler::PlayerState;
use crate::window_handler::{UserEvent, WindowHandler};
use tao::event::Event;
use tao::event_loop::ControlFlow;
use tao::event_loop::EventLoop;

const MAX_PLAYER_INFO_STRING_LENGTH: usize = 46;

pub struct App {
    pub preferences: Preferences,
    pub window_handler: WindowHandler,
    pub key_handler: KeyHandler,
    pub menu_handler: MenuHandler,
    pub tray_handler: TrayHandler,
    pub player_info: String,
}

impl App {
    pub fn new(event_loop: &mut EventLoop<UserEvent>) -> App {
        let preferences = Preferences::load();
        let window_handler = WindowHandler::new(event_loop);
        let key_handler = KeyHandler::new().register_keys();
        let menu_handler = MenuHandler::new(&preferences);
        let tray_handler = TrayHandler::new(&menu_handler);
        let player_info = String::from("");

        Self {
            window_handler,
            key_handler,
            menu_handler,
            tray_handler,
            preferences,
            player_info,
        }
    }

    pub fn tick(&mut self, event: &Event<UserEvent>, control_flow: &mut ControlFlow) {
        events_handler::callback(self, event, control_flow);
    }

    pub fn update_player_info(&mut self, meta: &PlayerState) {
        let play = if meta.state == "playing" {
            "▶"
        } else {
            "⏸"
        };
        let mut info = format!("{} {} - {}", play, meta.artist, meta.title);

        if info.len() > MAX_PLAYER_INFO_STRING_LENGTH {
            info.truncate(MAX_PLAYER_INFO_STRING_LENGTH);
            info.push_str("...");
        }

        self.player_info = info;
    }
}
