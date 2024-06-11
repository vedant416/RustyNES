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
pub mod units;

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
            square1: Square::new(0),
            square2: Square::new(1),
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
            // increment sample count after writing to buffer
            self.sample_count += 1;
        }
    }

    fn step_frame_counter(&mut self) {
        self.frame_counter += 1;
        match self.frame_counter {
            3729 => self.step_quarter_frame(),
            7457 => self.step_half_frame(),
            11186 => self.step_quarter_frame(),
            14915 if self.four_step_mode => {
                self.step_half_frame();
                self.frame_counter = 0;
                if !self.irq_disabled {
                    self.irq_triggered = true;
                }
            }
            18641 if !self.four_step_mode => {
                self.step_half_frame();
                self.frame_counter = 0;
            }
            _ => {}
        }
    }

    fn step_quarter_frame(&mut self) {
        self.square1.step_quarter_frame();
        self.square2.step_quarter_frame();
        self.triangle.step_quarter_frame();
        self.noise.step_quarter_frame();
    }

    fn step_half_frame(&mut self) {
        self.square1.step_half_frame();
        self.square2.step_half_frame();
        self.triangle.step_half_frame();
        self.noise.step_half_frame();
    }
}

// Read/Write /////
impl APU {
    pub fn read(&mut self, addr: u16) -> u8 {
        if addr == 0x4015 {
            return self.read_status();
        }
        return 0;
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
            0x4009 => {}
            0x400A => self.triangle.write1(val),
            0x400B => self.triangle.write2(val),

            // noise
            0x400C => self.noise.write0(val),
            0x400D => {}
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

    fn read_status(&mut self) -> u8 {
        // read channel status and irq status into single byte
        let mut status = 0;

        // set bit 1 (0000_0001)
        if self.square1.read_status() {
            status |= 0x1;
        }

        // set bit 2 (0000_0010)
        if self.square2.read_status() {
            status |= 0x2;
        }

        // set bit 3 (0000_0100)
        if self.triangle.read_status() {
            status |= 0x4;
        }

        // set bit 4 (0000_1000)
        if self.noise.read_status() {
            status |= 0x8;
        }

        // set bit 5 (0001_0000)
        if self.dmc.read_status() {
            status |= 0x10;
        }

        // set bit 7 (0100_0000)
        if self.irq_triggered {
            status |= 0x40;
        }

        // set bit 8 (1000_0000)
        if self.dmc.irq_triggered {
            status |= 0x80;
        }

        self.irq_triggered = false;

        return status;
    }

    fn write_control(&mut self, val: u8) {
        self.square1.write_control(val & 0x1 != 0);
        self.square2.write_control(val & 0x2 != 0);
        self.triangle.write_control(val & 0x4 != 0);
        self.noise.write_control(val & 0x8 != 0);
        self.dmc.write_control(val & 0x10 != 0);
    }

    fn write_frame_counter(&mut self, val: u8) {
        self.frame_counter = 0;
        // bit 7 (0100_0000)
        self.irq_disabled = val & 0x40 != 0;
        if self.irq_disabled {
            self.irq_triggered = false;
        }
        // bit 8 (1000_0000)
        // 0: 4-step mode, 1: 5-step mode
        self.four_step_mode = val & 0x80 == 0;
        if !self.four_step_mode {
            self.step_half_frame();
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
        output * 0.2
    }

    fn read_buffer(&mut self) -> f32 {
        let sample = self.buffer[self.buffer_start];
        self.buffer_start = (self.buffer_start + 1) % BUFFER_SIZE;
        sample
    }

    fn write_buffer(&mut self) {
        self.buffer[self.buffer_end] = self.output();
        self.buffer_end = (self.buffer_end + 1) % BUFFER_SIZE;
    }

    pub fn load_samples(&mut self, buffer: &mut [f32]) {
        let available_samples = {
            let start = self.buffer_start;
            let end = self.buffer_end;
            if start < end {
                end - start
            } else {
                BUFFER_SIZE - (start - end)
            }
        };
        for i in 0..buffer.len() {
            if i < available_samples {
                buffer[i] = self.read_buffer();
            } else {
                buffer[i] = 0.0;
            }
        }
    }
}
