impl super::PPU {
    //// fetch background //////////////////////////
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

    
    //// fetch_sprites ////////////////////////////
    pub fn fetch_sprites(&mut self) {
        todo!("Implement PPU::fetch_sprites")
    }
}