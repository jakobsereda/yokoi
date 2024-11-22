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
        Self { 
            registers, bus 
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        let op = self.fetch()?;
        self.execute(op)?;
        Ok(())
    }

    fn fetch(&mut self) -> Result<u8> {
        let pc = self.registers.pc;
        let op = self.bus.read_byte(pc)
            .with_context(|| format!("Failed to read byte at address: {:#X}", pc))?;
        self.registers.pc = pc.wrapping_add(1);
        Ok(op)
    }

    fn execute(&mut self, op: u8) -> Result<()> {
        let pc = self.registers.pc;
        match op {
            // -- NOP --
            0x00 => { }

            // -- LD BC,u16 --
            0x01 => {
                // TODO: handle error / decide how errors should be handled in execute phase
                let val = self.bus.read_word(pc)
                    .context("Opcode 0x01 failed to read from ROM")?;
                self.registers.pc = pc.wrapping_add(2);
                self.registers.set_bc(val);
            }

            // -- LD BC, A --
            0x02 => {
                let val = self.registers.a;
                let addr = self.registers.get_bc();
                self.bus.write_byte(val, addr)
                    .context("Opcode 0x02 failed to write to ROM")?;
            }

            // -- INC BC --
            0x03 => {
                let bc = self.registers.get_bc();
                self.registers.set_bc(bc + 1);
            }

            // -- INC B --
            0x04 => {
                // TODO: flag checking??
                self.registers.b = self.registers.b.wrapping_add(1);
            }

            // -- DEC B --
            0x05 => { 
                self.registers.b = self.registers.b.wrapping_sub(1); 
            }

            // -- LD B, u8 --
            0x06 => {
                let val = self.bus.read_byte(pc)
                    .context("Opcode 0x06 failed to read from ROM")?;
                self.registers.pc = pc.wrapping_add(1);
                self.registers.b = val;
            }

            // -- RLCA --
            0x07 => {
                // Rotate the contents of register A to the left. That is, the contents of bit 0 are 
                // copied to bit 1, and the previous contents of bit 1 (before the copy operation) are 
                // copied to bit 2. The same operation is repeated in sequence for the rest of the register. 
                // The contents of bit 7 are placed in both the CY flag and bit 0 of register A.
                // 
                // https://gist.github.com/bberak/ca001281bb8431d2706afd31401e802b
            }

            // -- LD (u16), SP --
            0x08 => {
                
            }

            _ => unimplemented!("Unimplemented opcode: {:#04X}", op)
        }

        Ok(())
    }
}
