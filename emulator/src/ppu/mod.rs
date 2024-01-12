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

    // memory
    frame_palette: [u8; 32],

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
            ///// render //////
            if render_time {
                self.render();
            }
            
            ///// fetch background //////
            if fetch_time {
                self.fetch_bg()
            }

            ///// x-scroll, y-scroll (increment) and (reset) /////
            self.increment_and_reset(fetch_line, fetch_dot, preline);

            ////// sprite evaluation //////
            if self.dot == 257 {
                if visible_line {
                    self.fetch_sprites();
                } else {
                    todo!("clear sprite memory");
                }
            }
        }

        ////// enter vblank //////
        if self.line == 241 && self.dot == 1 {
            todo!("Implement vblank entering")
        }

        ////// exit vblank  //////
        if self.line == 261 && self.dot == 1 {
            todo!("Implement vblank exiting")
        }

        ////// nmi handling //////

        
        ////// dot, line and frame counters (increment) and (reset) //////
        if rendering_enabled && self.odd && self.line == 261 && self.dot == 339 {
            // skip cycle 339 of pre-render scanline when odd frame
            self.dot = 0;
            self.line = 0;
            self.odd = !self.odd;
            self.frame_counter += 1;
            return;
        }

        // increment dot, reset at 341
        self.dot += 1;
        if self.dot > 340 {
            self.dot = 0;

            // increment line, reset at 262
            self.line += 1;
            if self.line > 261 {
                self.line = 0;
                self.odd = !self.odd;
                self.frame_counter += 1;
            }
        }
    }

    fn increment_and_reset(&mut self, fetch_line: bool, fetch_dot: bool, preline: bool) {
        if fetch_line {
            ///// increment coarse x //////
            if fetch_dot && self.dot & 7 == 0 {
                // if coarse X == 31
                if self.v & 0x001F == 31 {
                    self.v &= !0x001F; // coarse X = 0
                    self.v ^= 0x0400; // switch horizontal nametable
                } else {
                    self.v += 1; // else: increment coarse X
                }
            }

            ////// increment fine y ///////
            if self.dot == 256 {
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

            ///// reset x bits //////
            if self.dot == 257 {
                self.v = (self.v & 0xFBE0) | (self.t & 0x041F);
            }
        }

        ///// to "reset y bits" ///////
        if preline && self.dot >= 280 && self.dot <= 304 {
            self.v = (self.v & 0x841F) | (self.t & 0x7BE0);
        }
    }

    pub fn render(&mut self) {
        todo!("Implement PPU::render")
    }

    pub fn fetch_bg(&mut self) {
        todo!("Implement PPU::fetch_bg")
    }

    pub fn fetch_sprites(&mut self) {
        todo!("Implement PPU::fetch_sprites")
    }
}
