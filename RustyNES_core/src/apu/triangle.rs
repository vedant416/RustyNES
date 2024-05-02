#[derive(Default)]
pub struct Triangle {}

// Step /////
impl Triangle {
    pub fn new() -> Self {
        Self {}
    }

    pub fn step(&mut self) {}

    fn step_quarter_frame(&mut self) {}

    fn step_half_frame(&mut self) {}

    pub fn output(&self) -> f32 {
        0.0
    }
}

// Read/Write /////
impl Triangle {
    fn read_status(&self) -> u8 {
        0
    }

    fn write_control(&self, val: u8) {}

    pub fn write0(&mut self, val: u8) {}

    pub fn write1(&mut self, val: u8) {}

    pub fn write2(&mut self, val: u8) {}

    pub fn write3(&mut self, val: u8) {}
}
