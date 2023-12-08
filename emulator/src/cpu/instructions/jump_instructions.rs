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

    pub fn brk(&mut self) {
        let pc = self.pc + 1;
        self.push_16(pc);
        
        self.b = true;
        self.push_8(self.get_flags() | 0x10);
        self.i = true;
        
        let lo = self.read(0xFFFE) as u16;
        let hi = self.read(0xFFFF) as u16;
        self.pc = (hi << 8) | lo;
    }

    pub fn rti(&mut self) {
        let flags = self.pull_8() & 0xEF | 0x20;
        self.set_flags(flags);
        let pc = self.pull_16();
        self.pc = pc;
    }
}