use anyhow::{Context, Result};

#[derive(Debug)]
pub struct Bus {
    data: Vec<u8>,
}

impl Bus {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn read_byte(&self, address: u16) -> Result<u8> {
        let index = address as usize;
        self.data
            .get(index)
            .copied()
            .with_context(|| format!("Address out of bounds: 0x{:04X}", address))
    }

    pub fn read_word(&self, address: u16) -> Result<u16> {
        let lo = self.read_byte(address)? as u16;
        let hi = self.read_byte(address + 1)? as u16;
        Ok((hi << 8) | lo)
    }

    pub fn write_byte(&self, val: u8, address: u16) -> Result<()> {
        // TODO
        Ok(())
    }
}