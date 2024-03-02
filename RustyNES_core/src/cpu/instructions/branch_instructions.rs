impl super::CPU {
    #[inline]
    pub fn branch(&mut self, new_addr: u16) {
        // record old address
        let old_addr = self.pc;

        // branch to new address
        self.pc = new_addr;

        // update cycles
        self.cycles += 1;
        // check if branch crosses page boundary
        // between old and new address
        // if it does, add 1 cycle
        if (old_addr & 0xFF00) != (new_addr & 0xFF00) {
            self.cycles += 1;
        }
    }

    pub fn bpl(&mut self, new_addr: u16) {
        if !self.n {
            self.branch(new_addr);
        }
    }

    pub fn bmi(&mut self, new_addr: u16) {
        if self.n {
            self.branch(new_addr);
        }
    }

    pub fn bvc(&mut self, new_addr: u16) {
        if !self.v {
            self.branch(new_addr);
        }
    }

    pub fn bvs(&mut self, new_addr: u16) {
        if self.v {
            self.branch(new_addr);
        }
    }

    pub fn bcc(&mut self, new_addr: u16) {
        if !self.c {
            self.branch(new_addr);
        }
    }

    pub fn bcs(&mut self, new_addr: u16) {
        if self.c {
            self.branch(new_addr);
        }
    }

    pub fn bne(&mut self, new_addr: u16) {
        if !self.z {
            self.branch(new_addr);
        }
    }

    pub fn beq(&mut self, new_addr: u16) {
        if self.z {
            self.branch(new_addr);
        }
    }
}
