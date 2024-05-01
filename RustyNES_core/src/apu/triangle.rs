#[derive(Default)]
pub struct Triangle {}

impl Triangle {
    pub fn new() -> Self {
        Self {}
    }

    pub fn step(&mut self) {}

    pub fn output(&self) -> f32 {
        0.0
    }

    pub fn write0(&mut self, val: u8) {}

    pub fn write1(&mut self, val: u8) {}

    pub fn write2(&mut self, val: u8) {}

    pub fn write3(&mut self, val: u8) {}
}
