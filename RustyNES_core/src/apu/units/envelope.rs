#[derive(Default)]
pub struct Envelope {
    pub counter: u8,
    pub period: u8,
    pub start: bool,
    pub loop_mode: bool,
    pub constant_mode: bool,
    pub constant_volume: u8,
    pub volume: u8,
}

impl Envelope {
    pub fn step(&mut self) {
        if self.start {
            self.start = false;
            self.volume = 15;
            self.counter = self.period;
            return;
        }
        if self.counter == 0 {
            self.update_volume();
            self.counter = self.period;
        } else {
            self.counter -= 1;
        }
    }

    fn update_volume(&mut self) {
        if self.volume == 0 {
            if self.loop_mode {
                self.volume = 15;
            }
        } else {
            self.volume -= 1;
        }
    }

    pub fn output(&self) -> u8 {
        if self.constant_mode {
            self.constant_volume
        } else {
            self.volume
        }
    }
}
