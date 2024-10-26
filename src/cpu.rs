use crate::registers::Registers;
use crate::bus::Bus;
use anyhow::{
    Context, 
    Result
};

pub struct CPU {
    registers: Registers,
    bus: Bus,
}

impl CPU {
    pub fn new(data: &[u8]) -> Self {
        let registers = Registers::new();
        let bus = Bus::new(data.to_vec());
        Self { registers, bus }
    }

    pub fn tick(&mut self) -> Result<()>{
        let op = self.fetch()?;
        self.execute(op);
        Ok(())
    }

    fn fetch(&mut self) -> Result<u8> {
        let pc = self.registers.pc;
        let op = self.bus.read_byte(pc)
            .with_context(|| format!("Failed to read byte at address: {:#X}", pc))?;
        self.registers.pc = pc.wrapping_add(1);
        Ok(op)
    }

    fn execute(&mut self, op: u8) {
        match op {
            // -- NOP --
            0x00 => { }

            // -- LD BC,u16 --
            0x01 => {

            }

            _ => unimplemented!("Unimplemented opcode: {:#04X}", op)
        }
    }
}