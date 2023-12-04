impl super::CPU {
    pub fn cmp(&mut self, addr: u16) {
        let val = self.read(addr);
        let subtraction = self.a.wrapping_sub(val);
        self.c = self.a >= val;
        self.update_zn_flags(subtraction);
    }

    pub fn cpx(&mut self, addr: u16) {
        let val = self.read(addr);
        let subtraction = self.x.wrapping_sub(val);
        self.c = self.x >= val;
        self.update_zn_flags(subtraction);
    }

    pub fn cpy(&mut self, addr: u16) {
        let val = self.read(addr);
        let subtraction = self.y.wrapping_sub(val);
        self.c = self.y >= val;
        self.update_zn_flags(subtraction);
    }

    pub fn bit(&mut self, addr: u16) {
        let val = self.read(addr);
        self.z = (self.a & val) == 0;
        self.v = (val >> 0x06 & 0x01) == 1;
        self.n = val & 0x80 != 0;
    }
}
