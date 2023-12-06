impl super::CPU {
    pub fn pha(&mut self) {
        self.push_8(self.a);
    }

    pub fn pla(&mut self) {
        self.a = self.pull_8();
        self.update_zn_flags(self.a);
    }

    pub fn php(&mut self) {
        let flags = self.get_flags() | 0x10;
        self.push_8(flags);
    }

    pub fn plp(&mut self) {
        let flags = self.pull_8() & 0xef | 0x20;
        self.set_flags(flags);
    }
}