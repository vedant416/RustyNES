
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

//// rendering ////////////////////////////////
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
        let bg = if render_bg { self.get_bg_color() } else { None };
        let sp = if render_sp { self.get_sp_color() } else { None };
        let combined_color = match (bg, sp) {
            (None, None) => SYSTEM_PALETTE[self.frame_palette[0] as usize],
            (None, Some((sp, _, _))) => sp,
            (Some(bg), None) => bg,
            (Some(bg), Some((sp, behind_bg, _))) => {
                if behind_bg {
                    bg
                } else {
                    sp
                }
            }
        };

        ///// handle sprite 0 hit //////
        if let Some((_, _, idx)) = sp {
            if idx == 0 && x < 255 && bg.is_some() && !self.sprite_0_hit() {
                self.set_sprite_0_hit();
            }
        }
        
        ///// put final color in frame buffer //////
        if x < 256 && y < 240 {
            let offset = (y * 256 + x) * 4;
            self.frame_buffer[offset] = combined_color.0;
            self.frame_buffer[offset + 1] = combined_color.1;
            self.frame_buffer[offset + 2] = combined_color.2;
            self.frame_buffer[offset + 3] = 255;
        }
    }

    fn get_bg_color(&mut self) -> Option<(u8, u8, u8)> {
        // upper 32 bits of shift register store the current tile row
        let tile_row = self.shift_register >> 32;
        // we store pixel data in reverse order
        // so we need to shift by 7 - x
        // we also need to multiply by 4 because each pixel is 4 bits
        let shift = (7 - self.x) * 4;
        let color_idx = (((tile_row) >> shift) & 0xF) as usize;
        if color_idx & 3 != 0 {
            return Some(SYSTEM_PALETTE[self.frame_palette[color_idx] as usize]);
        }
        return None;
    }

    fn get_sp_color(&mut self) -> Option<((u8, u8, u8), bool, u8)> {
        let x = self.dot - 1;
        for i in 0..(self.sprites_count as usize) {
            let sprite = self.sprites[i];
            if x >= sprite.x && x < sprite.x + 8 {
                let color_idx = sprite.tile_row[(x - sprite.x) as usize];
                if color_idx != 0 {
                    let index = 0x10 + (sprite.palette_index as usize) * 4 + (color_idx as usize);
                    let index = self.frame_palette[index] as usize;
                    let color = SYSTEM_PALETTE[index];
                    return Some((color, sprite.show_bg, sprite.index));
                }
            }
        }
        return None;
    }

}
