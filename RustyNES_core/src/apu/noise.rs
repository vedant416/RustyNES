use super::units::{Envelope, LengthCounter, Timer};

const NOISE: [u16; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];

#[derive(Default)]
pub struct Noise {
    enabled: bool,
    mode: bool,
    shift_register: u16,
    timer: Timer,
    envelope: Envelope,
    length_counter: LengthCounter,
}

// Step /////
impl Noise {
    pub fn new() -> Self {
        let mut noise = Self::default();
        noise.shift_register = 1;
        noise
    }

    pub fn step(&mut self) {
        if self.timer.step() {
            let shift = if self.mode { 6 } else { 1 };
            let bit0 = self.shift_register & 1;
            let bit1 = (self.shift_register >> shift) & 1;
            let feedback = bit0 ^ bit1;
            // shifted right by one bit
            self.shift_register >>= 1;
            // set bit 14 to feedback
            self.shift_register |= feedback << 14;
        }
    }

    pub fn step_quarter_frame(&mut self) {
        self.envelope.step();
    }

    pub fn step_half_frame(&mut self) {
        self.envelope.step();
        self.length_counter.step();
    }

    pub fn output(&self) -> f32 {
        if self.shift_register & 1 == 1 || self.length_counter.counter == 0 {
            0.0
        } else {
            self.envelope.output() as f32
        }
    }
}

// Read/Write /////
impl Noise {
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
