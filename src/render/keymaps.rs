use sdl2::keyboard::Keycode;
use std::collections::HashMap;

pub type KeyMap = HashMap<Keycode, u8>;

lazy_static! {
    pub static ref QWERTY_KEYMAP: KeyMap = {
        let mut m = HashMap::new();
        m.insert(Keycode::Num1, 0x1);
        m.insert(Keycode::Num2, 0x2);
        m.insert(Keycode::Num3, 0x3);
        m.insert(Keycode::Num4, 0xc);
        m.insert(Keycode::Q, 0x4);
        m.insert(Keycode::W, 0x5);
        m.insert(Keycode::E, 0x6);
        m.insert(Keycode::R, 0xd);
        m.insert(Keycode::A, 0x7);
        m.insert(Keycode::S, 0x8);
        m.insert(Keycode::D, 0x9);
        m.insert(Keycode::F, 0xe);
        m.insert(Keycode::Z, 0xa);
        m.insert(Keycode::X, 0x0);
        m.insert(Keycode::C, 0xb);
        m.insert(Keycode::V, 0xf);

        m
    };
    pub static ref DVORAK_KEYMAP: KeyMap = {
        let mut m = HashMap::new();
        m.insert(Keycode::Num1, 0x1);
        m.insert(Keycode::Num2, 0x2);
        m.insert(Keycode::Num3, 0x3);
        m.insert(Keycode::Num4, 0xc);
        m.insert(Keycode::Quote, 0x4);
        m.insert(Keycode::Comma, 0x5);
        m.insert(Keycode::Period, 0x6);
        m.insert(Keycode::P, 0xd);
        m.insert(Keycode::A, 0x7);
        m.insert(Keycode::O, 0x8);
        m.insert(Keycode::E, 0x9);
        m.insert(Keycode::U, 0xe);
        m.insert(Keycode::Semicolon, 0xa);
        m.insert(Keycode::Q, 0x0);
        m.insert(Keycode::J, 0xb);
        m.insert(Keycode::K, 0xf);

        m
    };
    pub static ref COLEMAK_KEYMAP: KeyMap = {
        let mut m = HashMap::new();
        m.insert(Keycode::Num1, 0x1);
        m.insert(Keycode::Num2, 0x2);
        m.insert(Keycode::Num3, 0x3);
        m.insert(Keycode::Num4, 0xc);
        m.insert(Keycode::Q, 0x4);
        m.insert(Keycode::W, 0x5);
        m.insert(Keycode::F, 0x6);
        m.insert(Keycode::P, 0xd);
        m.insert(Keycode::A, 0x7);
        m.insert(Keycode::R, 0x8);
        m.insert(Keycode::S, 0x9);
        m.insert(Keycode::T, 0xe);
        m.insert(Keycode::Z, 0xa);
        m.insert(Keycode::X, 0x0);
        m.insert(Keycode::C, 0xb);
        m.insert(Keycode::V, 0xf);

        m
    };
}
