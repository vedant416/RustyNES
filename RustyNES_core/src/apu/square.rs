#[derive(Default)]
pub struct Square {
    enabled: bool,
}

// Step /////
impl Square {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self) {}

    pub fn step_quarter_frame(&mut self) {}

    pub fn step_half_frame(&mut self) {}

    pub fn output(&self) -> f32 {
        0.0
    }
}

// Read/Write /////
impl Square {
    pub fn read_status(&self) -> bool {
        false
    }

    pub fn write_control(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn write0(&mut self, val: u8) {}

    pub fn write1(&mut self, val: u8) {}

    pub fn write2(&mut self, val: u8) {}

    pub fn write3(&mut self, val: u8) {}
}
