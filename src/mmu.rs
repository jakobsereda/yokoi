#[derive(Debug)]
pub struct MMU {
    memory: [u8; 0xFFFF],
}

impl MMU {
    pub fn new() -> Self {
        Self { 
            memory: vec![u8; 0xFFFF] 
        }
    }
}
