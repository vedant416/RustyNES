impl super::CPU {
    pub fn clc(&mut self) {
        self.c = false;
    }

    pub fn cld(&mut self) {
        self.d = false;
    }

    pub fn cli(&mut self) {
        self.i = false;
    }

    pub fn clv(&mut self) {
        self.v = false;
    }

    pub fn sec(&mut self) {
        self.c = true;
    }

    pub fn sed(&mut self) {
        self.d = true;
    }

    pub fn sei(&mut self) {
        self.i = true;
    }
}
