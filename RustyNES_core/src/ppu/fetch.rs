use super::PPU;

impl PPU {
    // fetch background //////////////////////////
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

        // pattern table is stored in chr_rom located in cartridge
        self.pattern_table_low_latch = self.read_chr(addr);
        self.pattern_table_high_latch = self.read_chr(addr + 8);
    }

    fn load_fetched_data(&mut self) {
        let mut new_data: u32 = 0;
        let attr = self.attribute_table_latch << 2;

        for _ in 0..8 {
            let p1 = (self.pattern_table_low_latch & (1 << 7)) >> 7;
            let p2 = (self.pattern_table_high_latch & (1 << 7)) >> 6;
            let pattern = p2 | p1;

            self.pattern_table_low_latch <<= 1;
            self.pattern_table_high_latch <<= 1;
            new_data <<= 4;

            new_data |= (attr | pattern) as u32;
        }

        self.shift_register |= new_data as u64;
    }

    // fetch_sprites ////////////////////////////
    pub fn fetch_sprites(&mut self) {
        let mut count = 0;
        let height = self.sprite_size() as u16;
        // iterate over all 64 sprites in OAM
        for i in 0..64 {
            let offset = i * 4;
            let y = self.oam[offset] as u16;

            // check if the sprite lies on the current line
            if !(y..=y + height).contains(&self.line) {
                continue;
            }
            let attr = self.oam[offset + 2];
            let palette_idx = attr & 0b0000_0011;
            let show_bg = attr & 0b0010_0000 != 0;
            let flip_horizontally = attr & 0b0100_0000 != 0;
            let flip_vertically = attr & 0b1000_0000 != 0;
            let x = self.oam[offset + 3];

            // calculate which row of the sprite is being rendered
            let row = self.line - y;
            // flip the row if the sprite is flipped vertically
            let mut row = if flip_vertically { height - row } else { row };
            let mut tile_idx: u16 = self.oam[offset + 1] as u16;
            let mut chr_bank = self.sprite_pt_addr();

            // if the sprite is 8x16, the chr bank is determined by the least significant bit of the tile index
            if height == 15 {
                chr_bank = (tile_idx & 1) * 0x1000;
                tile_idx &= 0xFE;
                if row > 7 {
                    // add 1 to use the second tile of the 8x16 sprite
                    tile_idx += 1;
                    // adjust the row within the second tile (0-7)
                    row -= 8;
                }
            };

            // get the data for the row
            let tile_offset = chr_bank + tile_idx * 16 + row;
            let chr_low = self.read_chr(tile_offset);
            let chr_high = self.read_chr(tile_offset + 8);
            // iterate over each pixel in the row
            // and combine the two bit planes into a single byte
            let mut tile_row = [0u8; 8];
            for i in 0..8 {
                let pixel_index = 1 << if flip_horizontally { i } else { 7 - i };
                let p1 = (chr_low & pixel_index != 0) as u8;
                let p2 = (chr_high & pixel_index != 0) as u8;
                let pattern = (palette_idx << 2) | (p2 << 1) | p1;
                tile_row[i] = pattern;
            }

            // store x coordinate, tile index, palette index, behind background, and the chr
            self.sprites[count] = super::Sprite {
                x: x as u16,
                index: i as u8,
                show_bg,
                tile_row,
            };

            count += 1;

            if count == 8 {
                self.set_sprite_overflow();
                break;
            }
        }

        self.sprites_count = count as u8;
    }
}
