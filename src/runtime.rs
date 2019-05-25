//! Data types and functions for the rust side of the runtime

/// The CHIP-8 context
#[repr(C)]
pub struct Ctx {
    pc: u16,
    reg_i: u16,
    registers: [u8; 0xF],
    memory: [u8; 0xFFF],
    stack: [u16; 0xF],
    keys: u16,
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    screen: [u64; 32],
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            pc: 0x200,
            reg_i: 0,
            registers: [0; 0xF],
            memory: [0; 0xFFF],
            stack: [0; 0xF],
            keys: 0,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            screen: [0; 32],
        }
    }

    /// Updates the internal data of whether or not a key is pressed down
    pub fn set_key(&mut self, key: u8, pressed: bool) {
        assert!(key < 0x10);

        let bit = (pressed as u16) << (key as u16);
        self.keys &= !bit;
        self.keys |= bit;
    }

    /// The offset of screen from the start of the struct
    pub fn screen_offset(&self) -> usize {
        (&self.screen as *const _ as usize) - (self as *const Ctx as usize)
    }

    pub const fn screen_size(&self) -> usize {
        32 * 8
    }
}
