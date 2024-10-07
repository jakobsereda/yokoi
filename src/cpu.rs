use crate::registers::{self, Registers};

pub struct CPU {
    registers: Registers,
}

impl CPU {
    pub fn new() -> Self {
        let registers = Registers::new();
        Self {
            registers: registers,
        }
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    fn fetch(&mut self) -> u8 {
        0
    }

    fn execute(&mut self, op: u8) {
        match op {
            // nop
            0x00 => { }

            // ld r16, imm16
            0x01 => {

            }
            
            _ => panic!("Unimplemented opcode: {}", op)
        }

    }


}