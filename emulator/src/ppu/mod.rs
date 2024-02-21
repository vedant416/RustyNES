use crate::rom::Cartridge;
mod fetch;
mod io;
mod render;
pub struct PPU {
    dot: u16,  // 0-340
    line: u16, // 0-261, 0-239=visible, 240=post, 241-260=vblank, 261=pre

    // PPU Registers
    pub ctrl: u8,
    pub mask: u8,
    pub status: u8,
    pub oam_addr: u8,

    // Loopy Registers
    v: u16,  // Current VRAM address (15 bits)
    t: u16,  // Temporary VRAM address (15 bits)
    x: u8,   // Fine X scroll (3 bits)
    w: bool, // First or second write toggle (1 bit)

    // shift register stuff
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
    // latches to store fetched data before loading into shift register
    nametable_latch: u8,
    attribute_table_latch: u8,
    pattern_table_low_latch: u8,
    pattern_table_high_latch: u8,

    // ppu memory
    vram: [u8; 2048],
    frame_palette: [u8; 32],
    oam: [u8; 256],
    sprites: [Sprite; 8],
    sprites_count: u8,

    // frame management
    odd: bool, // odd frame flag
    frame_counter: u64,
    frame_buffer: Box<[u8; 256 * 240 * 4]>,
    pub frame_complete: bool,

    // nmi
    nmi_previous_state: bool,
    nmi_triggering_allowed: bool,
    nmi_triggered: bool,


    open_bus: u8,
    data_latch: u8,
    pub dma_triggered: bool,
    pub cartridge: Cartridge,
}

#[derive(Clone, Copy)]
struct Sprite {
    x: u16,
    index: u8,
    show_bg: bool,
    tile_row: [u8; 8],
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

        // which time?
        let render_time = visible_line && visible_dot;
        let fetch_time = fetch_line && fetch_dot;
        let sp_fetch_time = self.dot == 256 && visible_line;

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

            ////// fetch sprites //////
            if sp_fetch_time {
                self.fetch_sprites();
            }

            //// increment course x and fine y and copy x and y bits from t to v ////
            self.increment_and_copy(fetch_line, fetch_dot, preline);
        }

        ////// enter vblank //////
        if self.line == 241 && self.dot == 1 {
            self.set_vblank_started();
            self.update_nmi_state();
        }

        ////// exit vblank  //////
        if self.line == 261 && self.dot == 1 {
            self.clear_vblank_started();
            self.clear_sprite_0_hit();
            self.clear_sprite_overflow();
            self.update_nmi_state();
        }

        ////// nmi handling //////
        if self.nmi_triggering_allowed && self.current_nmi_state() {
            self.nmi_triggered = true;
            self.nmi_triggering_allowed = false;
        }

        ////// dot, line and frame counters (increment) and (reset) and special case of (skipping) //////
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

    //// nmi handling /////////////////////////////
    fn update_nmi_state(&mut self) {
        let nmi_current_state = self.current_nmi_state();

        if !self.nmi_previous_state && nmi_current_state {
            self.nmi_triggering_allowed = true;
        }

        self.nmi_previous_state = nmi_current_state;
    }

    fn current_nmi_state(&self) -> bool {
        self.genrate_nmi() && self.vblank_started()
    }

    //// increment and copy //////////////////////
    fn increment_and_copy(&mut self, fetch_line: bool, fetch_dot: bool, preline: bool) {
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

            ///// copy x bits from t to v //////
            if self.dot == 257 {
                self.v = (self.v & 0xFBE0) | (self.t & 0x041F);
            }
        }

        ///// copy y bits from t to v ///////
        if preline && self.dot >= 280 && self.dot <= 304 {
            self.v = (self.v & 0x841F) | (self.t & 0x7BE0);
        }
    }
}

impl PPU {
    pub fn dma_triggered(&mut self) -> bool {
        let triggered = self.dma_triggered;
        self.dma_triggered = false;
        triggered
    }

    pub fn nmi_triggered(&mut self) -> bool {
        let triggered = self.nmi_triggered;
        self.nmi_triggered = false;
        triggered
    }
}
