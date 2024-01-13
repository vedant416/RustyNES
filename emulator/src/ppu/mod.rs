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

    /*
        pixel color is 4 bits:
            ( px = 4 bits )

        tile is 8x8 px (single tile row is 8px)
            ( row_size = 8*4 = 32 bits )

        shift_register stores pixels for 2 tile rows
            ( 32 bits * 2 = 64 bits )
            shift_register size = 64 bits
    */
    shift_register: u64,
    
    // memory
    frame_palette: [u8; 32],

    // frame stuff
    odd: bool, // odd frame flag
    frame_counter: u64,
    frame_buffer: Box<[u8; 256 * 240 * 4]>,

    // other ppu stuff
    open_bus: u8,
    data_buffer: u8,
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

        let rendering_enabled = self.is_rendering_enabled();

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

    //// rendering /////////////////////////////
    pub fn render(&mut self) {
        /*
        pixel is not generated on dot 1
        which makes x-coord 1-indexed
        subtract by 1 to make x-coord 0-indexed
        */
        let x = (self.dot - 1) as usize;
        let y = self.line as usize;

        ///// handle clipping //////
        let mut render_sp = self.bg_rendering_allowed();
        let mut render_bg = self.sp_rendering_allowed();
        if x < 8 {
            if !self.leftmost_bg_rendering_allowed() {
                render_bg = false;
            }
            if !self.leftmost_sp_rendering_allowed() {
                render_sp = false;
            }
        }

        ///// get final color //////
        let bg_color = if render_bg { self.get_bg_color() } else { None };
        let sp_color = if render_sp { self.get_sp_color() } else { None };

        let combined_color = match (bg_color, sp_color) {
            (Some(bg), Some(sp)) => {
                todo!("Implement sprite/background priority");
            }
            (Some(bg), None) => bg,
            (None, Some(sp)) => sp,
            (None, None) => (0, 0, 0),
        };

        ///// put final color in frame buffer //////
        if x < 256 && y < 240 {
            let offset = (y * 256 + x) * 4;
            self.frame_buffer[offset] = combined_color.0;
            self.frame_buffer[offset + 1] = combined_color.1;
            self.frame_buffer[offset + 2] = combined_color.2;
            self.frame_buffer[offset + 3] = 255;
        }
    }

    fn get_bg_color(&self) -> Option<(u8, u8, u8)> {
        todo!();
    }

    fn get_sp_color(&self) -> Option<(u8, u8, u8)> {
        todo!();
    }

    fn sp_rendering_allowed(&self) -> bool {
        todo!();
    }

    fn bg_rendering_allowed(&self) -> bool {
        todo!();
    }

    fn leftmost_sp_rendering_allowed(&self) -> bool {
        todo!();
    }

    fn leftmost_bg_rendering_allowed(&self) -> bool {
        todo!();
    }

    fn is_rendering_enabled(&self) -> bool {
        self.bg_rendering_allowed() && self.sp_rendering_allowed()
    }

    //// fetch background ///////////////////////
    pub fn fetch_bg(&mut self) {
        // before fetching:
        // shift the shift register by size of one pixel (4 bits)
        // to make room for next pixel value (4 bits)
        self.shift_register <<= 4;

        // fetching:
        match self.dot & 7 {
            // fetchs tile id
            1 => self.fetch_nt(),

            // at is used to determine which palette is used
            // (upper 2 bits of pixel value)
            3 => self.fetch_at(),

            // uses tile_id (fetched in fetch_nt) to fetch 2 bit-planes
            // combine 2 bit-planes (each bit plane is 1 bit)
            // (lower 2 bits of pixel value)
            7 => self.fetch_pt(),

            // after fetching:
            // combine fetched data into pixel value
            // upper 2 bits + lower 2 bits = 4bit pixel value
            // load combined value (4 bits) into shift register
            0 => self.load_fetched_data(),
            _ => {}
        }
    }
    fn fetch_nt(&mut self) {
        todo!();
    }
    fn fetch_at(&mut self) {
        todo!();
    }
    fn fetch_pt(&mut self) {
        todo!();
    }
    fn load_fetched_data(&mut self) {
        todo!();
    }

    //// fetch_sprites ///////////////////////
    pub fn fetch_sprites(&mut self) {
        todo!("Implement PPU::fetch_sprites")
    }
}

// read register
impl PPU {
    pub fn read_register(&mut self, addr: u16) -> u8 {
        match addr {
            0x2002 => self.read_status(),
            0x2004 => self.read_oam_data(),
            0x2007 => self.read_ppu_data(),
            _ => unreachable!("invalid PPU register address"),
        }
    }

    fn read_status(&self) -> u8 {
        todo!()
    }

    fn read_oam_data(&self) -> u8 {
        todo!()
    }

    fn read_ppu_data(&self) -> u8 {
        todo!()
    }
}

// write register
impl PPU {
    pub fn write_register(&mut self, addr: u16, data: u8) {
        self.open_bus = data;
        match addr {
            0x2000 => self.write_ctrl(data),
            0x2001 => self.write_mask(data),
            0x2003 => self.write_oam_addr(data),
            0x2004 => self.write_oam_data(data),
            0x2005 => self.write_scroll(data),
            0x2006 => self.write_ppu_addr(data),
            0x2007 => self.write_ppu_data(data),
            _ => unreachable!("invalid PPU register address"),
        }
    }

    fn write_ctrl(&self, data: u8) {
        todo!()
    }

    fn write_mask(&self, data: u8) {
        todo!()
    }

    fn write_oam_addr(&self, data: u8) {
        todo!()
    }

    fn write_oam_data(&self, data: u8) {
        todo!()
    }

    fn write_scroll(&self, data: u8) {
        todo!()
    }

    fn write_ppu_addr(&self, data: u8) {
        todo!()
    }

    fn write_ppu_data(&self, data: u8) {
        todo!()
    }
}
pub static SYSTEM_PALETTE: [(u8, u8, u8); 64] = [
    (0x80, 0x80, 0x80),
    (0x00, 0x3D, 0xA6),
    (0x00, 0x12, 0xB0),
    (0x44, 0x00, 0x96),
    (0xA1, 0x00, 0x5E),
    (0xC7, 0x00, 0x28),
    (0xBA, 0x06, 0x00),
    (0x8C, 0x17, 0x00),
    (0x5C, 0x2F, 0x00),
    (0x10, 0x45, 0x00),
    (0x05, 0x4A, 0x00),
    (0x00, 0x47, 0x2E),
    (0x00, 0x41, 0x66),
    (0x00, 0x00, 0x00),
    (0x05, 0x05, 0x05),
    (0x05, 0x05, 0x05),
    (0xC7, 0xC7, 0xC7),
    (0x00, 0x77, 0xFF),
    (0x21, 0x55, 0xFF),
    (0x82, 0x37, 0xFA),
    (0xEB, 0x2F, 0xB5),
    (0xFF, 0x29, 0x50),
    (0xFF, 0x22, 0x00),
    (0xD6, 0x32, 0x00),
    (0xC4, 0x62, 0x00),
    (0x35, 0x80, 0x00),
    (0x05, 0x8F, 0x00),
    (0x00, 0x8A, 0x55),
    (0x00, 0x99, 0xCC),
    (0x21, 0x21, 0x21),
    (0x09, 0x09, 0x09),
    (0x09, 0x09, 0x09),
    (0xFF, 0xFF, 0xFF),
    (0x0F, 0xD7, 0xFF),
    (0x69, 0xA2, 0xFF),
    (0xD4, 0x80, 0xFF),
    (0xFF, 0x45, 0xF3),
    (0xFF, 0x61, 0x8B),
    (0xFF, 0x88, 0x33),
    (0xFF, 0x9C, 0x12),
    (0xFA, 0xBC, 0x20),
    (0x9F, 0xE3, 0x0E),
    (0x2B, 0xF0, 0x35),
    (0x0C, 0xF0, 0xA4),
    (0x05, 0xFB, 0xFF),
    (0x5E, 0x5E, 0x5E),
    (0x0D, 0x0D, 0x0D),
    (0x0D, 0x0D, 0x0D),
    (0xFF, 0xFF, 0xFF),
    (0xA6, 0xFC, 0xFF),
    (0xB3, 0xEC, 0xFF),
    (0xDA, 0xAB, 0xEB),
    (0xFF, 0xA8, 0xF9),
    (0xFF, 0xAB, 0xB3),
    (0xFF, 0xD2, 0xB0),
    (0xFF, 0xEF, 0xA6),
    (0xFF, 0xF7, 0x9C),
    (0xD7, 0xE8, 0x95),
    (0xA6, 0xED, 0xAF),
    (0xA2, 0xF2, 0xDA),
    (0x99, 0xFF, 0xFC),
    (0xDD, 0xDD, 0xDD),
    (0x11, 0x11, 0x11),
    (0x11, 0x11, 0x11),
];
