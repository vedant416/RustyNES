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
mod units;

const CPU_FREQ: f32 = 1789773.0;
const SAMPLE_RATE: f32 = 48000.0;
const BUFFER_SIZE: usize = 0x2000;

pub struct APU {
    // channels
    square1: Square,
    square2: Square,
    triangle: Triangle,
    noise: Noise,
    dmc: Dmc,

    // timing
    cycle: u32,
    cycles_per_sample: f32, // cycles required to generate one sample
    sample_count: u32,      // total samples generated so far

    // frame counter
    frame_counter: u32,
    four_step_mode: bool, // frame step mode: 4-step or 5-step
    irq_triggered: bool,
    irq_disabled: bool,

    // buffer
    buffer_start: usize,             // start of buffer
    buffer_end: usize,               // end of buffer
    buffer: Box<[f32; BUFFER_SIZE]>, // circular buffer
}

// Step /////
impl APU {
    pub fn new() -> Self {
        Self {
            square1: Square::new(),
            square2: Square::new(),
            triangle: Triangle::new(),
            noise: Noise::new(),
            dmc: Dmc::new(),

            cycle: 0,
            cycles_per_sample: CPU_FREQ / SAMPLE_RATE,
            sample_count: 0,

            frame_counter: 0,
            four_step_mode: true,
            irq_triggered: false,
            irq_disabled: false,

            buffer_start: 0,
            buffer_end: 0,
            buffer: Box::new([0.0; BUFFER_SIZE]),
        }
    }

    pub fn step(&mut self) {
        self.cycle += 1;
        self.triangle.step();
        if self.cycle % 2 == 0 {
            self.square1.step();
            self.square2.step();
            self.noise.step();
            self.dmc.step();
            self.step_frame_counter();
        }

        // apu runs very fast compared to device sampling rate
        // so we need to downsample the output
        // we downsample output by writing to buffer at specific intervals i.e. cycles_per_sample
        let required_sample_count = (self.cycle as f32 / self.cycles_per_sample) as u32;
        if self.sample_count < required_sample_count {
            self.write_buffer(); 
        }
    }

    fn step_frame_counter(&mut self) {
        self.frame_counter += 1;
        match self.frame_counter {
            _ => {}
        }
    }
}

// Sound output /////
impl APU {
    fn output(&self) -> f32 {
        let s1 = self.square1.output();
        let s2 = self.square2.output();
        let t = self.triangle.output();
        let n = self.noise.output();
        let d = self.dmc.output();
        let out1 = 0.00752 * (s1 + s2);
        let out2 = (0.00851 * t) + (0.00494 * n) + (0.00335 * d);
        let output = out1 + out2;
        output
    }

    fn read_buffer(&self) -> f32 {
        0.0
    }
    fn write_buffer(&self) {}

    pub fn load_samples(&self, buffer: &mut [f32]) {
        for i in 0..buffer.len() {
            buffer[i] = self.read_buffer();
        }
    }
}

// Read/Write /////
impl APU {
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
