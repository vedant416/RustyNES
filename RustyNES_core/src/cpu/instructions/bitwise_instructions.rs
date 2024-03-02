use super::AddressingMode;

impl super::CPU {
    pub fn and(&mut self, addr: u16) {
        let val = self.read(addr);
        self.a &= val;
        self.update_zn_flags(self.a);
    }

    pub fn eor(&mut self, addr: u16) {
        let val = self.read(addr);
        self.a ^= val;
        self.update_zn_flags(self.a);
    }

    pub fn ora(&mut self, addr: u16) {
        let val = self.read(addr);
        self.a |= val;
        self.update_zn_flags(self.a);
    }

    pub fn asl(&mut self, addr: u16, addr_mode: &AddressingMode) {
        let val = match addr_mode {
            AddressingMode::Accumulator => self.a,
            _ => self.read(addr),
        };

        let result = val << 1;
        self.c = val & 0x80 != 0;
        self.update_zn_flags(result);

        match *addr_mode {
            AddressingMode::Accumulator => self.a = result,
            _ => self.write(addr, result),
        };
    }

    pub fn lsr(&mut self, addr: u16, addr_mode: &AddressingMode) {
        let val = match addr_mode {
            AddressingMode::Accumulator => self.a,
            _ => self.read(addr),
        };

        let result = val >> 1;
        self.c = val & 0x01 == 1;
        self.update_zn_flags(result);

        match *addr_mode {
            AddressingMode::Accumulator => self.a = result,
            _ => self.write(addr, result),
        };
    }

    pub fn rol(&mut self, addr: u16, addr_mode: &AddressingMode) {
        let val = match addr_mode {
            AddressingMode::Accumulator => self.a,
            _ => self.read(addr),
        };

        let result = (val << 1) | (self.c as u8);
        self.c = val & 0x80 != 0;
        self.update_zn_flags(result);

        match *addr_mode {
            AddressingMode::Accumulator => self.a = result,
            _ => self.write(addr, result),
        };
    }

    pub fn ror(&mut self, addr: u16, addr_mode: &AddressingMode) {
        let val = match addr_mode {
            AddressingMode::Accumulator => self.a,
            _ => self.read(addr),
        };

        let result = (val >> 1) | ((self.c as u8) << 7);
        self.c = val & 0x01 == 1;
        self.update_zn_flags(result);

        match *addr_mode {
            AddressingMode::Accumulator => self.a = result,
            _ => self.write(addr, result),
        };
    }
}
