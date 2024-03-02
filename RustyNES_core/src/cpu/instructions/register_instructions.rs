impl super::CPU {
    pub fn tax(&mut self) {
        self.x = self.a;
        self.update_zn_flags(self.x);
    }

    pub fn tay(&mut self) {
        self.y = self.a;
        self.update_zn_flags(self.y);
    }

    pub fn txa(&mut self) {
        self.a = self.x;
        self.update_zn_flags(self.a);
    }

    pub fn tya(&mut self) {
        self.a = self.y;
        self.update_zn_flags(self.a);
    }

    pub fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.update_zn_flags(self.x);
    }

    pub fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.update_zn_flags(self.y);
    }

    pub fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.update_zn_flags(self.x);
    }

    pub fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.update_zn_flags(self.y);
    }

    pub fn tsx(&mut self) {
        self.x = self.sp;
        self.update_zn_flags(self.x);
    }

    pub fn txs(&mut self) {
        self.sp = self.x;
    }
}
