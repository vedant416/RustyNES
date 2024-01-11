// mod rendering;

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
    v: u16,  // Current VRAM address (15 bits)
    t: u16,  // Temporary VRAM address (15 bits)
    x: u8,   // Fine X scroll (3 bits)
    w: bool, // First or second write toggle (1 bit)

    // frame stuff
    odd: bool, // odd frame flag
    frame_counter: u64,
    frame_buffer: Box<[u8; 256 * 240 * 4]>,
}

impl PPU {
    pub fn new() -> Self {
        todo!("Implement PPU::new")
    }
}

impl PPU {
    pub fn step(&mut self) {
        // which line?
        let visible_line = self.line < 240;
        let preline = self.line == 261;
        let fetch_line = visible_line || preline;

        // which dot?
        let visible_dot = self.dot >= 1 && self.dot <= 256;
        let pre_fetch_dot = self.dot >= 321 && self.dot <= 336;
        let fetch_dot = visible_dot || pre_fetch_dot;

        let render_time = visible_line && visible_dot;
        let fetch_time = fetch_line && fetch_dot;

        let rendering_enabled = true; // todo!("Implement rendering_enabled");

        if rendering_enabled {
            if render_time {
                todo!("Implement rendering")
            }
            if fetch_time {
                todo!("Implement background data fetching")
            }

            ///// x-scroll, y-scroll (increment) and (reset) /////
            if fetch_line {
                if fetch_dot && self.dot & 7 == 0 {
                    ///// increment coarse x //////
                    // if coarse X == 31
                    if self.v & 0x001F == 31 {
                        self.v &= !0x001F; // coarse X = 0
                        self.v ^= 0x0400; // switch horizontal nametable
                    } else {
                        self.v += 1; // else: increment coarse X
                    }
                }

                if self.dot == 256 {
                    ////// increment fine y ///////
                    // if fine Y < 7
                    if self.v & 0x7000 != 0x7000 {
                        self.v += 0x1000; // increment fine Y
                    } else {
                        self.v &= !0x7000; // fine Y = 0
                        let mut y = (self.v & 0x03E0) >> 5; // let y = coarse Y
                        if y == 29 {
                            y = 0; // coarse Y = 0
                            self.v ^= 0x0800; // switch vertical nametable
                        } else if y == 31 {
                            y = 0; // coarse Y = 0, nametable not switched
                        } else {
                            y += 1; // increment coarse Y
                        }

                        self.v = (self.v & !0x03E0) | (y << 5); // put coarse Y back into v
                    }
                }

                if self.dot == 257 {
                    ///// reset x bits //////
                    self.v = (self.v & 0xFBE0) | (self.t & 0x041F);
                }
            }

            if preline && self.dot >= 280 && self.dot <= 304 {
                ///// to "reset y bits" ///////
                self.v = (self.v & 0x841F) | (self.t & 0x7BE0);
            }

            ////// sprite evaluation //////
            if self.dot == 257 {
                if visible_line {
                    todo!("fetch sprites");
                } else {
                    todo!("clear sprite memory");
                }
            }
        }

        ////// enter and exit vblank //////
        if self.line == 241 && self.dot == 1 {
            todo!("Implement vblank entering")
        }

        if self.line == 261 && self.dot == 1 {
            todo!("Implement vblank exiting")
        }

        ////// nmi handling //////

        ////// dot, line and frame counters (increment) and (reset) //////
    }
}
