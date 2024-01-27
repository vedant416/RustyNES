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
}