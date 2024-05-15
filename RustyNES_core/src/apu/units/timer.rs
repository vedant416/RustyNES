#[derive(Default)]
pub struct Timer {
    pub counter: u16,
    pub period: u16,
}

impl Timer {
    pub fn step(&mut self) -> bool {
        if self.counter == 0 {
            self.counter = self.period;
            return true;
        } else {
            self.counter -= 1;
            return false;
        }
    }
}
