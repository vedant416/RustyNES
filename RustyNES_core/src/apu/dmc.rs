use super::units::Timer;

const DMC: [u16; 16] = [
    428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
];

#[derive(Default)]
pub struct Dmc {
    enabled: bool,

    start_address: u16, // start address of the sample
    total_length: u16,  // length of the sample

    pub current_address: u16,
    current_lenght: u16,

    shift_index: u8,
    shift_register: u8, // can only shift 8 times

    pub irq_triggered: bool,
    irq_enabled: bool,
    loop_mode: bool,
    timer: Timer,
    output: u8,
}

// Step /////
impl Dmc {
    pub fn new() -> Self {
        Dmc::default()
    }

    pub fn step(&mut self) {}

    pub fn output(&self) -> f32 {
        self.output as f32
    }
}

// Read/Write /////
impl Dmc {
    pub fn read_status(&self) -> bool {
        self.current_lenght != 0
    }

    pub fn write_control(&mut self, enabled: bool) {
        if !enabled {
            self.current_lenght = 0
        }
        // if enabled and length is 0, restart sample
        else if self.current_lenght == 0 {
            self.current_address = self.start_address;
            self.current_lenght = self.total_length;
        }
        self.enabled = enabled;
        self.irq_triggered = false;
    }

    pub fn write0(&mut self, val: u8) {
        self.irq_enabled = val & 0x80 != 0;
        self.loop_mode = val & 0x40 != 0;
        self.timer.period = DMC[val as usize & 0b1111];
        if !self.irq_enabled {
            self.irq_triggered = false;
        }
    }

    pub fn write1(&mut self, val: u8) {
        self.output = val & 0x7F;
    }

    pub fn write2(&mut self, val: u8) {
        self.start_address = 0xC000 | ((val as u16) << 6);
    }

    pub fn write3(&mut self, val: u8) {
        self.total_length = ((val as u16) << 4) | 1;
    }
}
