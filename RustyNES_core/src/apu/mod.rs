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

#[derive(Default)]
pub struct APU {
    square1: Square,
    square2: Square,
    triangle: Triangle,
    noise: Noise,
    dmc: Dmc,
}

impl APU {
    pub fn new() -> Self {
        Self {
            square1: Square::new(),
            square2: Square::new(),
            triangle: Triangle::new(),
            noise: Noise::new(),
            dmc: Dmc::new(),
        }
    }

    fn step(&mut self) {}

    fn output(&self) -> u8 {
        0
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x4015 => self.read_status(),
            _ => 0,
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            // square1
            0x4000 => self.square1.write0(val),
            0x4001 => self.square1.write1(val),
            0x4002 => self.square1.write2(val),
            0x4003 => self.square1.write3(val),

            // square2
            0x4004 => self.square2.write0(val),
            0x4005 => self.square2.write1(val),
            0x4006 => self.square2.write2(val),
            0x4007 => self.square2.write3(val),

            // triangle
            0x4008 => self.triangle.write0(val),
            0x4009 => self.triangle.write1(val),
            0x400A => self.triangle.write2(val),
            0x400B => self.triangle.write3(val),

            // noise
            0x400C => self.noise.write0(val),
            // 0x400D, // unused
            0x400E => self.noise.write1(val),
            0x400F => self.noise.write2(val),

            // DMC
            0x4010 => self.dmc.write0(val),
            0x4011 => self.dmc.write1(val),
            0x4012 => self.dmc.write2(val),
            0x4013 => self.dmc.write3(val),

            // control
            0x4015 => self.write_control(val),

            // frame counter
            0x4017 => self.write_frame_counter(val),

            _ => {}
        }
    }

    fn read_status(&self) -> u8 {
        0
    }

    fn write_control(&self, val: u8) {}

    fn write_frame_counter(&self, val: u8) {}
}
