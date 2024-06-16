use super::units::{Envelope, LengthCounter, Timer};

const SQUARE: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];

#[derive(Default)]
pub struct Square {
    id: u8,
    enabled: bool,
    // duty
    duty_mode: u8,  // 4 modes
    duty_cycle: u8, // 8 steps

    // sweep unit
    sweep_enabled: bool,
    sweep_period: u8,
    sweep_counter: u8,
    sweep_reload: bool,
    sweep_negate: bool,
    sweep_shift: u8,
    sweep_mute: bool,

    // other units
    timer: Timer,
    envelope: Envelope,
    length_counter: LengthCounter,
}

// Step /////
impl Square {
    pub fn new(id: u8) -> Self {
        let mut square = Self::default();
        square.id = id;
        square
    }

    pub fn step(&mut self) {
        if self.timer.step() {
            self.duty_cycle = (self.duty_cycle + 1) & 7;
        }
    }

    pub fn step_quarter_frame(&mut self) {
        self.envelope.step();
    }

    pub fn step_half_frame(&mut self) {
        self.step_sweep();
        self.envelope.step();
        self.length_counter.step();
    }

    fn calculate_period(&self) -> u16 {
        let change_amount = self.timer.period >> self.sweep_shift;
        if self.sweep_negate {
            if self.id == 0 {
                self.timer.period.saturating_sub(change_amount + 1)
            } else {
                self.timer.period.saturating_sub(change_amount)
            }
        } else {
            self.timer.period + change_amount
        }
    }

    fn step_sweep(&mut self) {
        let period = self.calculate_period();
        self.sweep_mute = period > 0x7FF || self.timer.period < 8;
        if self.sweep_enabled && self.sweep_counter == 0 && !self.sweep_mute {
            self.timer.period = period;
        }
        if self.sweep_counter == 0 || self.sweep_reload {
            self.sweep_counter = self.sweep_period;
            self.sweep_reload = false;
        } else {
            self.sweep_counter -= 1;
        }
    }

    pub fn output(&self) -> f32 {
        if !self.enabled || self.sweep_mute || self.length_counter.counter == 0 {
            return 0.0;
        }
        let duty = SQUARE[self.duty_mode as usize][self.duty_cycle as usize];
        let output = self.envelope.output();
        (duty * output) as f32
    }
}

// Read/Write /////
impl Square {
    pub fn read_status(&self) -> bool {
        self.length_counter.counter != 0
    }

    pub fn write_control(&mut self, enabled: bool) {
        if !enabled {
            self.length_counter.counter = 0;
        }
        self.enabled = enabled;
    }

    pub fn write0(&mut self, val: u8) {
        self.envelope.start = true;
        self.duty_mode = (val >> 6) & 0b11;

        let halt = val & 0x20 != 0;
        self.length_counter.enabled = !halt;
        self.envelope.loop_mode = halt;

        let period = val & 0b1111;
        self.envelope.period = period;
        self.envelope.constant_volume = period;
        self.envelope.constant_mode = val & 0x10 != 0;
    }

    pub fn write1(&mut self, val: u8) {
        self.sweep_reload = true;
        self.sweep_enabled = val & 0x80 != 0;
        self.sweep_period = ((val >> 4) & 0b111) + 1;
        self.sweep_negate = val & 0x8 != 0;
        self.sweep_shift = val & 0b111;
    }

    pub fn write2(&mut self, val: u8) {
        self.timer.period = (self.timer.period & 0xFF00) | val as u16;
    }

    pub fn write3(&mut self, val: u8) {
        self.duty_cycle = 0;
        let period = (val & 0b111) as u16;
        let period = period << 8;
        self.timer.period = (self.timer.period & 0x00FF) | period;
        self.length_counter.set(val >> 3);
    }
}
