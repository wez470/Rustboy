#[derive(Clone, Copy, Debug)]
pub enum Reg8 { A, B, C, D, E, H, L }

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)] // Reg16::PC is only used in tests right now.
pub enum Reg16 { AF, BC, DE, HL, SP, PC }

/// Represents bit indexes of flags in the flags register.
// TODO(solson): Use BitFlags for this.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Flag {
    Zero = 7,
    Sub = 6,
    HalfCarry = 5,
    Carry = 4,
}

/// Represents a 16-bit register in the Game Boy CPU.
#[derive(Clone, Copy, Debug)]
pub struct Register(u16);

impl Register {
    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, val: u16) {
        self.0 = val;
    }

    pub fn low(&self) -> u8 {
        self.0 as u8
    }

    pub fn high(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn set_low(&mut self, low: u8) {
        self.0 = u16::from_le_bytes([low, self.high()]);
    }

    pub fn set_high(&mut self, high: u8) {
        self.0 = u16::from_le_bytes([self.low(), high]);
    }
}

impl std::ops::AddAssign<u16> for Register {
    fn add_assign(&mut self, val: u16) {
        self.0 = self.0.wrapping_add(val);
    }
}

impl std::ops::SubAssign<u16> for Register {
    fn sub_assign(&mut self, val: u16) {
        self.0 = self.0.wrapping_sub(val);
    }
}

impl std::ops::AddAssign<i8> for Register {
    fn add_assign(&mut self, offset: i8) {
        // Two's complement addition handles negative values of `offset` correctly.
        *self += offset as i16 as u16;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Registers {
    /// Register `A`, the high half of `AF`, also known as the accumulator.
    pub a: u8,

    /// Register `F`, the low half of `AF`, also known as the flags register.
    pub f: u8,

    /// Register `BC`. Also accessible in 8-bit halves, `B` (high) and `C` (low).
    pub bc: Register,

    /// Register `DE`. Also accessible in 8-bit halves, `D` (high) and `E` (low).
    pub de: Register,

    /// Register `HL`, which is typically used for memory addressing. Also accessible in 8-bit
    /// halves, `H` (high) and `L` (low).
    pub hl: Register,

    /// Register `SP`, which contains the stack pointer.
    pub sp: Register,

    /// Register `PC`, which contains the program counter.
    pub pc: Register,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x01,
            f: 0xB0,
            bc: Register(0x1300),
            de: Register(0xD800),
            hl: Register(0x4D01),
            sp: Register(0xFFFE),
            pc: Register(0x0100),
        }
    }

    pub fn set_8(&mut self, reg: Reg8, val: u8) {
        match reg {
            Reg8::A => self.a = val,
            Reg8::B => self.bc.set_high(val),
            Reg8::C => self.bc.set_low(val),
            Reg8::D => self.de.set_high(val),
            Reg8::E => self.de.set_low(val),
            Reg8::H => self.hl.set_high(val),
            Reg8::L => self.hl.set_low(val),
        }
    }

    pub fn get_8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.a,
            Reg8::B => self.bc.high(),
            Reg8::C => self.bc.low(),
            Reg8::D => self.de.high(),
            Reg8::E => self.de.low(),
            Reg8::H => self.hl.high(),
            Reg8::L => self.hl.low(),
        }
    }

    pub fn set_16(&mut self, reg: Reg16, val: u16) {
        let [low, high] = val.to_le_bytes();
        match reg {
            // The four low bits of F, the flag register, must always be zero.
            Reg16::AF => { self.a = high; self.f = low & 0xF0; }
            Reg16::BC => self.bc.set(val),
            Reg16::DE => self.de.set(val),
            Reg16::HL => self.hl.set(val),
            Reg16::SP => self.sp.set(val),
            Reg16::PC => self.pc.set(val),
        }
    }

    pub fn get_16(&self, reg: Reg16) -> u16 {
        match reg {
            // The four low bits of F, the flag register, must always be zero.
            Reg16::AF => u16::from_le_bytes([self.f & 0xF0, self.a]),
            Reg16::BC => self.bc.get(),
            Reg16::DE => self.de.get(),
            Reg16::HL => self.hl.get(),
            Reg16::SP => self.sp.get(),
            Reg16::PC => self.pc.get(),
        }
    }
}
