impl super::CPU {
    pub fn lda(&mut self, addr: u16) {
        self.a = self.read(addr);
        self.update_zn_flags(self.a);
    }

    pub fn ldx(&mut self, addr: u16) {
        self.x = self.read(addr);
        self.update_zn_flags(self.x);
    }

    pub fn ldy(&mut self, addr: u16) {
        self.y = self.read(addr);
        self.update_zn_flags(self.y);
    }

    pub fn sta(&mut self, addr: u16) {
        self.write(addr, self.a);
    }

    pub fn stx(&mut self, addr: u16) {
        self.write(addr, self.x);
    }

    pub fn sty(&mut self, addr: u16) {
        self.write(addr, self.y);
    }

    pub fn inc(&mut self, addr: u16) {
        let val = self.read(addr).wrapping_add(1);
        self.write(addr, val);
        self.update_zn_flags(val);
    }

    pub fn dec(&mut self, addr: u16) {
        let val = self.read(addr).wrapping_sub(1);
        self.write(addr, val);
        self.update_zn_flags(val);
    }
}
