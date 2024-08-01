use anyhow::Result;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};
use std::collections::HashMap;

pub struct KeyHandler {
    pub manager: GlobalHotKeyManager,
    pub keys: HashMap<u32, &'static str>,
}

impl KeyHandler {
    pub fn init() -> Result<KeyHandler> {
        let keys = HashMap::new();
        let manager = GlobalHotKeyManager::new()?;

        Ok(KeyHandler { manager, keys })
    }

    pub fn register_keys(mut self) -> KeyHandler {
        let modifiers = Some(Modifiers::SHIFT);
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
        self.register_key(play_pause_key, "Youtubby.playPauseClick()");
        //self.register_key(HotKey::new(modifiers, Code::MediaTrackNext), "");
        //self.register_key(HotKey::new(modifiers, Code::MediaTrackPrevious), "");
        self
    }

    fn register_key(&mut self, key: HotKey, js: &'static str) {
        if let Ok(()) = self.manager.register(key) {
            self.keys.insert(key.id, js);
        } else {
            log::error!("Can't register key!")
        }
    }
}
