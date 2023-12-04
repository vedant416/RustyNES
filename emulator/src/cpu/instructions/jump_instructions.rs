impl super::CPU {
    pub fn jmp(&mut self, addr: u16) {
        self.pc = addr;
    }

    pub fn jsr(&mut self, addr: u16) {
        self.push_16(self.pc - 1);
        self.pc = addr;
    }

    pub fn rts(&mut self) {
        let pc = self.pull_16();
        self.pc = pc + 1;
    }

    pub fn rti(&mut self) {
        let flags = self.pull_8() & 0xEF | 0x20;
        self.set_flags(flags);
        let pc = self.pull_16();
        self.pc = pc;
    }
}