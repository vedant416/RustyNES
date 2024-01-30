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
        let nt_addr = 0x2000 | (self.v & 0x0FFF);
        self.nametable_latch = self.read_nametable(nt_addr);
    }
    
    fn fetch_at(&mut self) {
        let v = self.v;
        let attr_addr = 0x23C0 | (v & 0x0C00) | ((v >> 4) & 0x38) | ((v >> 2) & 0b111);
        let shift = ((v >> 4) & 4) | (v & 2);
        self.attribute_table_latch = (self.read_nametable(attr_addr) >> shift) & 0b11;
    }
    
    fn fetch_pt(&mut self) {
        let table = self.background_pt_addr();
        let tile_num = self.nametable_latch as u16;
        let fine_y = ((self.v >> 12) & 0b111) as u8;
        let addr = table + tile_num * 16 + fine_y as u16;

        self.pattern_table_low_latch = self.read_chr(addr);
        self.pattern_table_high_latch = self.read_chr(addr + 8);
    }
    
    fn load_fetched_data(&mut self) {
        todo!();
    }

    
    //// fetch_sprites ////////////////////////////
    pub fn fetch_sprites(&mut self) {
        todo!("Implement PPU::fetch_sprites")
    }
}