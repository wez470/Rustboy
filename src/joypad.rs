use bitflags::bitflags;
use crate::interrupts::Interrupt;

bitflags! {
    pub struct ButtonKeys: u8 {
        const A = 1 << 0;
        const B = 1 << 1;
        const SELECT = 1 << 2;
        const START = 1 << 3;
    }
}

bitflags! {
    pub struct DirKeys: u8 {
        const RIGHT = 1 << 0;
        const LEFT = 1 << 1;
        const UP = 1 << 2;
        const DOWN = 1 << 3;
    }
}

#[derive(Clone, Debug)]
pub struct Joypad {
    /// Whether the joypad register should reflect which button keys are pressed.
    select_button_keys: bool,

    /// Whether the joypad register should reflect which direction keys are pressed.
    select_dir_keys: bool,

    /// Bit flags of which button keys are currently held down.
    button_keys_pressed: ButtonKeys,

    /// Bit flags of which direction keys are currently held down.
    dir_keys_pressed: DirKeys,

    /// Whether to request a Joypad interrupt on the next CPU step.
    should_interrupt: bool,
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            select_button_keys: true,
            select_dir_keys: true,
            button_keys_pressed: ButtonKeys::empty(),
            dir_keys_pressed: DirKeys::empty(),
            should_interrupt: false,
        }
    }

    pub fn button_key_down(&mut self, button: ButtonKeys) {
        let before = self.read_reg();
        self.button_keys_pressed.insert(button);
        let after = self.read_reg();
        // Request an interrupt if a 1 bit in `before` became a 0 bit in `after`.
        self.should_interrupt = before & !after != 0;
    }

    pub fn button_key_up(&mut self, button: ButtonKeys) {
        self.button_keys_pressed.remove(button);
    }

    pub fn dir_key_down(&mut self, dir: DirKeys) {
        let before = self.read_reg();
        self.dir_keys_pressed.insert(dir);
        let after = self.read_reg();
        // Request an interrupt if a 1 bit in `before` became a 0 bit in `after`.
        self.should_interrupt = before & !after != 0;
    }

    pub fn dir_key_up(&mut self, dir: DirKeys) {
        self.dir_keys_pressed.remove(dir);
    }

    pub fn read_reg(&self) -> u8 {
        // For all the used bits in this register, 0 actually represents `true` values of the
        // corresponding fields. I found it easiest to construct the opposite and then negate at
        // the end.
        //
        // NOTE: The top two bits of this register are unused and should always set to 1 according
        // to Mooneye tests.
        let mut bits = 0;
        bits |= (self.select_button_keys as u8) << 5;
        bits |= (self.select_dir_keys as u8) << 4;
        if self.select_button_keys {
            bits |= self.button_keys_pressed.bits();
        }
        if self.select_dir_keys {
            bits |= self.dir_keys_pressed.bits();
        }
        !bits
    }

    pub fn write_reg(&mut self, bits: u8) {
        // Bits 0-3 are read-only and bits 6-7 are unused and unwritable according to Mooneye.
        // Also, the meaning of these bits is negated (0 means `true`).
        self.select_button_keys = bits >> 5 & 1 == 0;
        self.select_dir_keys = bits >> 4 & 1 == 0;
        // TODO(solson): Enabling these bits can trigger the Joypad interrupt if some keys were
        // already being held, so we should handle interrupts here, too. (Or, more likely, in a way
        // that lets us do the check in a single place.)
    }

    /// Called by the CPU when executing an instruction. Returns whether to request a Joypad
    /// interrupt.
    pub fn step(&mut self) -> Option<Interrupt> {
        if self.should_interrupt {
            self.should_interrupt = false;
            Some(Interrupt::Joypad)
        } else {
            None
        }
    }
}