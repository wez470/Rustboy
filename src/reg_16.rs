#[derive(Default)]
pub struct Reg16 {
    pub high: u8,
    pub low: u8
}

impl Reg16 {
    pub fn set(&mut self, val: u16) {
        self.low = (val & 0xFF) as u8;
        self.high = ((val >> 8) & 0xFF) as u8;
    }

    pub fn get(&self) -> u16 {
        ((self.high as u16) << 8) | (self.low as u16)
    }

    pub fn inc(&mut self) {
        let val: u16 = self.get();
        self.set(val + 1);
    }

    pub fn set_bit_8(&mut self, set: bool) {
        if set {
            self.low |= 128;
        }
        else {
            self.low &= 127;
        }
    }

    pub fn set_bit_7(&mut self, set: bool) {
        if set {
            self.low |= 64;
        }
        else {
            self.low &= 191;
        }
    }

    pub fn set_bit_6(&mut self, set: bool) {
        if set {
            self.low |= 32;
        }
        else {
            self.low &= 223;
        }
    }

    pub fn set_bit_5(&mut self, set: bool) {
        if set {
            self.low |= 16;
        }
        else {
            self.low &= 239;
        }
    }
}

#[test]
fn test_set_16_bit_reg() {
    let mut reg = Reg16::new();
    let expected_low = 96;
    let expected_high = 234;
    reg.set(60000);
    assert!(reg.low == expected_low);
    assert!(reg.high == expected_high);
}

#[test]
fn test_get_16_bit_reg() {
    let mut reg = Reg16::new();
    reg.low = 96;
    reg.high = 234;
    let expected = 60000;
    assert!(reg.get() == expected);
}

#[test]
fn test_inc() {
    let mut reg = Reg16::new();
    let val = 0;
    let expected = val + 1;
    reg.set(0);
    reg.inc();
    assert!(reg.get() == expected);
}

#[test]
#[should_panic]
fn test_inc_overflow() {
    let mut reg = Reg16::new();
    let val = 65535;
    reg.set(val);
    reg.inc();
}