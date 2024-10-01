const VRAM_SIZE: usize = 0x4000;

struct Tile {
    
}

struct GPU {
    tile_set: [Tile; 384],
    vram: [u8; VRAM_SIZE],
}