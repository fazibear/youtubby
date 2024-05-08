use std::collections::HashMap;

use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, GlobalHotKeyEventReceiver, GlobalHotKeyManager, HotKeyState,
};

pub struct KeyHandler {
    manager: GlobalHotKeyManager,
    channel: &'static GlobalHotKeyEventReceiver,
    keys: HashMap<u32, &'static str>,
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
        self.manager.register(key).unwrap();
        self.keys.insert(key.id, js);
    }

    pub fn try_recv(&self, window_handler: &crate::WindowHandler) {
        if let Ok(GlobalHotKeyEvent {
            id,
            state: HotKeyState::Pressed,
        }) = self.channel.try_recv()
        {
            if let Some(&js) = self.keys.get(&id) {
                window_handler.webview.evaluate_script(js).unwrap();
            }
        }
    }
}
