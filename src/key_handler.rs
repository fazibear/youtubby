use std::collections::HashMap;

use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, GlobalHotKeyEventReceiver, GlobalHotKeyManager,
};

pub struct KeyHandler {
    pub manager: GlobalHotKeyManager,
    pub channel: &'static GlobalHotKeyEventReceiver,
    pub keys: HashMap<u32, &'static str>,
}

impl KeyHandler {
    pub fn new() -> KeyHandler {
        let keys = HashMap::new();
        let manager = GlobalHotKeyManager::new().unwrap();
        let channel = GlobalHotKeyEvent::receiver();

        KeyHandler {
            manager,
            channel,
            keys,
        }
    }

    pub fn register_keys(mut self) -> KeyHandler {
        let modifiers = None;
        let play_pause_key = {
            #[cfg(target_os = "macos")]
            {
                HotKey::new(modifiers, Code::MediaPlayPause)
            }
            #[cfg(not(target_os = "macos"))]
            {
                HotKey::new(modifiers, Code::MediaPlay)
            }
        };
        self.register_key(play_pause_key, "PlayPauseClick()");
        //self.register_key(HotKey::new(modifiers, Code::MediaTrackNext), "");
        //self.register_key(HotKey::new(modifiers, Code::MediaTrackPrevious), "");
        self
    }

    fn register_key(&mut self, key: HotKey, js: &'static str) {
        if let Ok(()) = self.manager.register(key) {
            self.keys.insert(key.id, js);
        }
    }
}
