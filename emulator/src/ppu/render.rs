struct SpriteRenderData {
    color: (u8, u8, u8),
    index: u8,
    show_bg: bool,
}

impl super::PPU {
    pub fn render(&mut self) {
        /*
        pixel is not generated on dot 1
        which makes x-coord 1-indexed
        subtract by 1 to make x-coord 0-indexed
        */
        let x = (self.dot - 1) as usize;
        let y = self.line as usize;

        ///// handle clipping //////
        let mut render_bg = self.bg_rendering_allowed();
        let mut render_sp = self.sp_rendering_allowed();
        if x < 8 {
            if !self.leftmost_bg_rendering_allowed() {
                render_bg = false;
            }
            if !self.leftmost_sp_rendering_allowed() {
                render_sp = false;
            }
        }

        ///// get final color //////
        let bg = if render_bg { self.get_bg_color() } else { None };
        let sp = if render_sp { self.get_sp_color() } else { None };
        let color = match (bg, &sp) {
            (None, None) => SYSTEM_PALETTE[self.frame_palette[0] as usize],
            (None, Some(sp)) => sp.color,
            (Some(bg), None) => bg,
            (Some(bg), Some(sp)) => {
                if sp.show_bg {
                    bg
                } else {
                    sp.color
                }
            }
        };

        ///// handle sprite 0 hit /////
        if let Some(sp) = &sp {
            if sp.index == 0 && x < 255 && bg.is_some() && !self.sprite_0_hit() {
                self.status = self.status | 0x40;
            }
        }

        ///// put final color in frame buffer /////
        let (r, g, b) = color;
        if x < 256 && y < 240 {
            let offset = (y * 256 + x) * 4;
            self.frame_buffer[offset + 0] = r;
            self.frame_buffer[offset + 1] = g;
            self.frame_buffer[offset + 2] = b;
            self.frame_buffer[offset + 3] = 255;
        }
    }

    fn get_bg_color(&mut self) -> Option<(u8, u8, u8)> {
        // upper 32 bits of shift register store the current tile row
        let tile_row = self.shift_register >> 32;
        /*
        Pixel data is stored in below format:
        Example => px0, px1, px2, px3, px4, px5, px6, px7
        Bits  =>   7    6    5    4    3    2    1    0
        To get px0, we need to get the bit7 of tile_row
        so we need to shift by 7 - x => 7 - 0 = 7
        we also need to multiply by 4 because each pixel is 4 bits
        */
        let shift = (7 - self.x) * 4;
        let index = (((tile_row) >> shift) & 0xF) as usize;
        // skip transparent pixels
        if index & 3 == 0 {
            return None;
        }
        // calculate color
        let index = self.frame_palette[index] as usize;
        let color = SYSTEM_PALETTE[index];
        return Some(color);
    }

    fn get_sp_color(&mut self) -> Option<SpriteRenderData> {
        let x = self.dot - 1;
        // loop through all sprites in secondary OAM
        // return color if non-transparent px is found
        for i in 0..(self.sprites_count as usize) {
            let sprite = self.sprites[i];
            // skip current sprite if x is not 8px range of sprite.x
            if !(sprite.x..=sprite.x + 7).contains(&x) {
                continue;
            }
            let index = (x - sprite.x) as usize;
            let color_index = sprite.tile_row[index];
            // skip transparent pixels
            if color_index & 3 == 0 {
                continue;
            }
            // calculate color
            // sprite palette address is offset by 0x10
            let index = self.frame_palette[(0x10 + color_index) as usize] as usize;
            let color = SYSTEM_PALETTE[index];
            return Some(SpriteRenderData {
                color,
                show_bg: sprite.show_bg,
                index: sprite.index,
            });
        }
        return None;
    }
}

// SYSTEM_PALETTE does not change
// It is static memory within the PPU
// frame_palette stores indexes in SYSTEM_PALETTE
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
