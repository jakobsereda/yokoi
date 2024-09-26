pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

pub enum Flags {
    Z = 0x80,
    N = 0x40,
    H = 0x20,
    C = 0x10,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            // TODO: find vals for register initialization and/or boot rom
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn get_af(&self) -> u16 {
        // TODO
        0
    }

    pub fn set_af(&mut self, val: u16) {
        // TODO
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = ((val & 0xff00) >> 8) as u8;
        self.c = (val & 0xff) as u8;
    }

    pub fn get_de(&self) -> u16 {
        // TODO
        0
    }

    pub fn set_de(&mut self, val: u16) {
        // TODO
    }

    pub fn get_hl(&self) -> u16 {
        // TODO
        0
    }

    pub fn set_hl(&mut self, val: u16) {
        // TODO
    }
}