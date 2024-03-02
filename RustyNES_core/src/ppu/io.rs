impl super::PPU {
    // read register
    pub fn read_register(&mut self, addr: u16) -> u8 {
        match addr {
            2 => self.read_status(),
            4 => self.read_oam_data(),
            7 => self.read_ppu_data(),
            _ => unreachable!(),
        }
    }

    pub fn read_status(&mut self) -> u8 {
        let res = (self.status & 0b1110_0000) | (self.open_bus & 0b0001_1111);
        self.w = false;
        self.status = self.status & !0x80;
        self.update_nmi_state();
        res
    }

    pub fn read_oam_data(&mut self) -> u8 {
        self.oam[self.oam_addr as usize]
    }

    // write register
    pub fn write_register(&mut self, addr: u16, data: u8) {
        self.open_bus = data;
        match addr {
            0 => self.write_ctrl(data),
            1 => self.write_mask(data),
            2 => unreachable!(),
            3 => self.write_oam_addr(data),
            4 => self.write_oam_data(data),
            5 => self.write_scroll(data),
            6 => self.write_ppu_addr(data),
            7 => self.write_ppu_data(data),
            _ => unreachable!(),
        }
    }

    pub fn write_ctrl(&mut self, data: u8) {
        self.ctrl = data;
        self.t = (self.t & 0xF3FF) | (((data as u16) & 0b11) << 10);
        self.update_nmi_state();
    }

    pub fn write_mask(&mut self, data: u8) {
        self.mask = data;
    }

    pub fn write_oam_addr(&mut self, val: u8) {
        self.oam_addr = val;
    }

    pub fn read_ppu_data(&mut self) -> u8 {
        let addr = self.v;

        let res = match addr {
            0x0000..=0x1fff => self.read_chr_delayed(addr),
            0x2000..=0x3eff => self.read_nametable_delayed(addr),
            0x3f00..=0x3fff => self.read_palette(addr),
            _ => unreachable!(),
        };

        self.v = self.v.wrapping_add(self.vram_addr_increment()) & 0x3fff;
        res
    }

    pub fn write_oam_data(&mut self, data: u8) {
        self.oam[self.oam_addr as usize] = data;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub fn write_scroll(&mut self, data: u8) {
        if !self.w {
            self.t = (self.t & 0xFFE0) | ((data as u16) >> 3);
            self.x = data & 0b111;
            self.w = true;
        } else {
            self.t = (self.t & 0x8FFF) | (((data as u16) & 0b111) << 12);
            self.t = (self.t & 0xFC1F) | (((data as u16) & 0b11111000) << 2);
            self.w = false;
        }
    }

    pub fn write_ppu_addr(&mut self, data: u8) {
        if !self.w {
            self.t = (self.t & 0x80FF) | (((data as u16) & 0b111111) << 8);
            self.t &= 0xBFFF;
            self.w = true;
        } else {
            self.t = (self.t & 0xFF00) | (data as u16);
            self.v = self.t;
            self.w = false;
        }
    }

    pub fn write_ppu_data(&mut self, data: u8) {
        let addr = self.v;

        match addr {
            0x0000..=0x1fff => self.write_chr(addr, data),
            0x2000..=0x3eff => self.write_nametable(addr, data),
            0x3f00..=0x3fff => self.write_palette(addr, data),
            _ => unreachable!(),
        }

        self.v = self.v.wrapping_add(self.vram_addr_increment()) & 0x3fff;
    }

    // utils to extract info from ppu registers
    // ctrl bits
    pub fn genrate_nmi(&self) -> bool {
        self.ctrl & 0x80 != 0
    }

    pub fn sprite_size(&self) -> u8 {
        if (self.ctrl & 0x20) == 0 {
            7
        } else {
            15
        }
    }

    pub fn background_pt_addr(&self) -> u16 {
        if self.ctrl & 0x10 == 0 {
            0x0000
        } else {
            0x1000
        }
    }

    pub fn sprite_pt_addr(&self) -> u16 {
        if self.ctrl & 0x08 == 0 {
            0x0000
        } else {
            0x1000
        }
    }

    pub fn vram_addr_increment(&mut self) -> u16 {
        if (self.ctrl & 0x04) == 0 {
            1
        } else {
            32
        }
    }

    // mask bits
    pub fn sp_rendering_allowed(&self) -> bool {
        self.mask & 0x10 != 0
    }

    pub fn bg_rendering_allowed(&self) -> bool {
        self.mask & 0x08 != 0
    }

    pub fn leftmost_sp_rendering_allowed(&self) -> bool {
        self.mask & 0x04 != 0
    }

    pub fn leftmost_bg_rendering_allowed(&self) -> bool {
        self.mask & 0x02 != 0
    }

    pub fn is_rendering_enabled(&self) -> bool {
        self.bg_rendering_allowed() || self.sp_rendering_allowed()
    }

    // status bits
    // vblank flag
    pub fn vblank_started(&self) -> bool {
        self.status & 0x80 != 0
    }

    pub fn set_vblank_started(&mut self) {
        self.status |= 0x80;
    }

    pub fn clear_vblank_started(&mut self) {
        self.status &= !0x80;
    }

    // sprite 0 hit
    pub fn sprite_0_hit(&self) -> bool {
        self.status & 0x40 != 0
    }

    pub fn set_sprite_0_hit(&mut self) {
        self.status |= 0x40;
    }

    pub fn clear_sprite_0_hit(&mut self) {
        self.status &= !0x40;
    }

    // sprite overflow
    pub fn sprite_overflow(&self) -> bool {
        self.status & 0x20 != 0
    }

    pub fn set_sprite_overflow(&mut self) {
        self.status |= 0x20;
    }

    pub fn clear_sprite_overflow(&mut self) {
        self.status &= !0x20;
    }

    // read/write ppu address space
    // CHR ROM (Cartridge)
    pub fn read_chr(&mut self, addr: u16) -> u8 {
        self.cartridge.read(addr)
    }

    pub fn write_chr(&mut self, addr: u16, data: u8) {
        self.cartridge.write(addr, data);
    }

    pub fn read_chr_delayed(&mut self, addr: u16) -> u8 {
        let res = self.data_latch;
        self.data_latch = self.read_chr(addr);
        res
    }

    // NAMETABLE (VRAM)
    pub fn read_nametable(&self, addr: u16) -> u8 {
        let addr = self.map_vram_addr(addr);
        self.vram[addr as usize]
    }

    pub fn write_nametable(&mut self, addr: u16, data: u8) {
        let addr = self.map_vram_addr(addr);
        self.vram[addr as usize] = data;
    }

    pub fn read_nametable_delayed(&mut self, addr: u16) -> u8 {
        let res = self.data_latch;
        self.data_latch = self.read_nametable(addr);
        res
    }

    // PALETTE
    pub fn write_palette(&mut self, addr: u16, data: u8) {
        let addr = self.map_palette_addr(addr) as usize;
        self.frame_palette[addr] = data;
    }

    pub fn read_palette(&mut self, addr: u16) -> u8 {
        let addr = self.map_palette_addr(addr) as usize;
        self.frame_palette[addr]
    }

    // Mirrorings
    pub fn map_vram_addr(&self, addr: u16) -> u16 {
        let mirror_mode = &self.cartridge.get_data().mirroring;
        mirror_mode.get_address(addr)
    }

    pub fn map_palette_addr(&self, addr: u16) -> u16 {
        let addr = (addr - 0x3F00) & 31;
        if addr >= 16 && addr & 3 == 0 {
            addr - 16
        } else {
            addr
        }
    }
}
