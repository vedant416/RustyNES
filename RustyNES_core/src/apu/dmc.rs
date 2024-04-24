#[derive(Default)]
pub struct Dmc {}

impl Dmc {
    pub fn new() -> Self {
        Self {}
    }

    fn step(&mut self) {}

    fn output(&self) -> f32 {
        0.0
    }

    pub fn write0(&mut self, val: u8) {}

    pub fn write1(&mut self, val: u8) {}

    pub fn write2(&mut self, val: u8) {}

    pub fn write3(&mut self, val: u8) {}
}
