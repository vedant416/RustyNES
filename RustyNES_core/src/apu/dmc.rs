#[derive(Default)]
pub struct Dmc {
    enabled: bool,
    pub irq_triggered: bool,
}

// Step /////
impl Dmc {
    pub fn new() -> Self {
        Dmc::default()
    }

    pub fn step(&mut self) {}

    pub fn output(&self) -> f32 {
        0.0
    }
}

// Read/Write /////
impl Dmc {
    pub fn read_status(&self) -> bool {
        false
    }

    pub fn write_control(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.irq_triggered = false;
    }

    pub fn write0(&mut self, val: u8) {}

    pub fn write1(&mut self, val: u8) {}

    pub fn write2(&mut self, val: u8) {}

    pub fn write3(&mut self, val: u8) {}
}
