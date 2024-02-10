impl super::CPU {
    pub fn adc(&mut self, addr: u16) {
        let val = self.read(addr);
        let sum_u16 = (self.a as u16) + (val as u16) + (self.c as u16);
        let sum_u8 = (sum_u16 & 0x00FF) as u8;

        self.v = ((self.a ^ val) & 0x80 == 0) && ((self.a ^ sum_u8 as u8) & 0x80 != 0);
        self.c = sum_u16 > 0xFF;
        self.a = sum_u8;
        self.update_zn_flags(sum_u8);
    }

    pub fn sbc(&mut self, addr: u16) {
        let val = !self.read(addr);
        let sum_u16 = (self.a as u16) + (val as u16) + (self.c as u16);
        let sum_u8 = (sum_u16 & 0x00FF) as u8;

        self.v = ((self.a ^ val) & 0x80 == 0) && ((self.a ^ sum_u8 as u8) & 0x80 != 0);
        self.c = sum_u16 > 0xFF;
        self.a = sum_u8;
        self.update_zn_flags(sum_u8);
    }
}
