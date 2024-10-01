pub struct Registers {
    pub a: u8,
    f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

pub enum Flags {
    Z = 0x80,
    N = 0x40,
    H = 0x20,
    C = 0x10,
}

impl Registers {
    pub fn new() -> Self {
        use Flags::*;
        Self {
            a: 0x01,
            f: Z as u8 | H as u8 | C as u8,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            h: 0x01,
            l: 0x4d,
            pc: 0x0100,
            sp: 0xfffe,
        }
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f & 0xf0) as u16
    }

    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xf0) as u8;
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0xff) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val & 0xff) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0xff) as u8;
    }

    pub fn hli(&mut self) -> u16 {
        let tmp = self.get_hl();
        self.set_hl(tmp + 1);
        tmp
    }

    pub fn hld(&mut self) -> u16 {
        let tmp = self.get_hl();
        self.set_hl(tmp - 1);
        tmp
    }

    pub fn get_flag(&self, flags: Flags) -> bool {
        let mask = flags as u8;
        self.f & mask > 0
    }

    pub fn set_flag(&mut self, flags: Flags, set: bool) {
        let mask = flags as u8;
        match set {
            true  => self.f |= mask,
            false => self.f &= !mask,
        }
        self.f &= 0xf0;
    }
}