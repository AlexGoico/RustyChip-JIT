//! Where config data lives

use crate::render::keymaps;

/// megastruct containing all the config data
#[derive(Debug)]
pub struct Settings {
    pub keymap: keymaps::KeyMap,
}

impl Settings {
    pub fn new() -> Self {
        let keymap_name = std::env::var("RCJ_KEYMAP")
            .unwrap_or("qwerty".to_string())
            .to_lowercase();
        let keymap = match keymap_name.as_ref() {
            "qwerty" => (*keymaps::QWERTY_KEYMAP).clone(),
            "dvorak" => (*keymaps::DVORAK_KEYMAP).clone(),
            "colemak" => (*keymaps::COLEMAK_KEYMAP).clone(),
            _ => panic!("Unsupported key map \"{:?}\" found"),
        };
        Self { keymap }
    }
}
