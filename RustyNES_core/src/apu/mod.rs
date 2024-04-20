#![allow(dead_code)]
#![allow(unused_variables)]
use dmc::Dmc;
use noise::Noise;
use square::Square;
use triangle::Triangle;

mod dmc;
mod noise;
mod square;
mod triangle;

pub struct APU {
    square1: Square,
    square2: Square,
    triangle: Triangle,
    noise: Noise,
    dmc: Dmc,
}

impl APU {
    fn new() -> Self {
        todo!();
    }

    fn step(&mut self) {
        todo!();
    }

    fn output(&self) -> u8 {
        todo!();
    }

    fn read(&self, addr: u16) -> u8 {
        todo!();
    }

    fn write(&mut self, addr: u16, data: u8) {
        todo!();
    }
}
