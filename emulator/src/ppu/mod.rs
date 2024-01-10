pub struct PPU {
    dot: u16,  // 0-340
    line: u16, // 0-261, 0-239=visible, 240=post, 241-260=vblank, 261=pre

    // PPU Registers
    ppuctrl: u8,   // $2000
    ppumask: u8,   // $2001
    ppustatus: u8, // $2002
    oamaddr: u8,   // $2003
    oamdata: u8,   // $2004
    ppuscroll: u8, // $2005
    ppuaddr: u8,   // $2006
    ppudata: u8,   // $2007

    // Loopy Registers
    v: u16,    // Current VRAM address (15 bits)
    t: u16,    // Temporary VRAM address (15 bits)
    x: u8,     // Fine X scroll (3 bits)
    w: bool,   // First or second write toggle (1 bit)
    
    odd: bool, // odd frame flag
    frame_counter: u64,
}

impl PPU {
    pub fn new() -> Self {
        todo!("Implement PPU::new")
    }
}

impl PPU {
    pub fn step(&mut self) {
        // todo: implement rendering

        // todo: implement background data fetching

        // todo: implement sprite data fetching

        // todo: implement vblank entering

        // todo: implement vblank exiting
        
        // todo: implement nmi handling
                              
        // todo: implement increment dot, line and frame counters
    }
}
