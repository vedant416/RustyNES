const LENGTH: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];

#[derive(Default)]
pub struct LengthCounter {
    pub enabled: bool,
    pub counter: u8,
}

impl LengthCounter {
    pub fn step(&mut self) {
        if self.enabled && self.counter > 0 {
            self.counter -= 1;
        }
    }

    pub fn set(&mut self, index: u8) {
        self.counter = LENGTH[index as usize];
    }
}
