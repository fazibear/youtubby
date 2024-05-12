use std::collections::HashMap;

use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, GlobalHotKeyEventReceiver, GlobalHotKeyManager, HotKeyState,
};
use tao::{event::Event, event_loop::EventLoopProxy};

use crate::window_handler::UserEvent;

pub struct KeyHandler {
    manager: GlobalHotKeyManager,
    channel: &'static GlobalHotKeyEventReceiver,
    keys: HashMap<u32, &'static str>,
    event_loop_proxy: &'static EventLoopProxy<UserEvent>,
}

impl KeyHandler {
    pub fn new(event_loop_proxy: &EventLoopProxy<UserEvent>) -> KeyHandler {
        let keys = HashMap::new();
        let manager = GlobalHotKeyManager::new().unwrap();
        let channel = GlobalHotKeyEvent::receiver();

        KeyHandler {
            manager,
            channel,
            keys,
            event_loop_proxy,
        }
    }

    fn register_key(&mut self, key: HotKey, js: &'static str) {
        self.manager.register(key).unwrap();
        self.keys.insert(key.id, js);
    }

    pub fn resend(&self) {
        if let Ok(event) = self.channel.try_recv() {
            self.event_loop_proxy
                .send_event(UserEvent::HotKeyEvent(event))
                .unwrap();
        }
    }

    pub fn on_event(&self, event: &Event<UserEvent>) {}

    pub fn play_pause_key() -> u32 {
        let playpause = {
            #[cfg(target_os = "macos")]
            {
                HotKey::new(None, Code::MediaPlayPause)
            }
            #[cfg(not(target_os = "macos"))]
            {
                HotKey::new(None, Code::MediaPlay)
            }
        };
        playpause.id
    }
}
