use super::units::{LengthCounter, Timer};

const TRIANGLE: [u8; 32] = [
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 14, 15,
];

#[derive(Default)]
pub struct Triangle {
    enabled: bool,
    duty_cycle: u8,
    timer: Timer,
    length_counter: LengthCounter,
    // linear counter
    control_flag: bool,
    reload_flag: bool,
    counter: u8,
    reload_value: u8,
}

// Step /////
impl Triangle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self) {
        if self.timer.step() && self.counter != 0 && self.length_counter.counter != 0 {
            self.duty_cycle = (self.duty_cycle + 1) & 31;
        }
    }

    pub fn step_quarter_frame(&mut self) {
        self.step_linear_counter();
    }

    pub fn step_half_frame(&mut self) {
        self.step_linear_counter();
        self.length_counter.step();
    }

    fn step_linear_counter(&mut self) {
        if self.reload_flag {
            self.counter = self.reload_value;
        } else if self.counter > 0 {
            self.counter -= 1;
        }

        // if control_flag is clear, reload flag is cleared
        if !self.control_flag {
            self.reload_flag = false;
        }
    }

    pub fn output(&self) -> f32 {
        0.0
    }
}

// Read/Write /////
impl Triangle {
    pub fn read_status(&self) -> bool {
        false
    }

    pub fn write_control(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn write0(&mut self, val: u8) {}

    pub fn write1(&mut self, val: u8) {}

    pub fn write2(&mut self, val: u8) {}
}
