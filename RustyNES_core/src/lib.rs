#![allow(clippy::upper_case_acronyms)]
pub mod apu;
pub mod buffer;
pub mod bus;
pub mod controller;
pub mod cpu;
pub mod mappers;
pub mod ppu;
pub mod rom;

pub use apu::BUFFER_SIZE;
pub use apu::SAMPLE_RATE;
use bus::BUS;
use controller::Controller;
pub use cpu::CPU;
use ppu::PPU;
use rom::ROM;

impl CPU {
    pub fn new_from_rom_bytes(bytes: Vec<u8>) -> CPU {
        let cartridge = ROM::new_cartridge(bytes);
        let ppu = PPU::new_ppu(cartridge);
        let controller = Controller::new_controller();
        let bus = BUS::new_bus(ppu, controller);
        CPU::new_cpu(bus)
    }

    pub fn new_nes_from_save_bytes(bytes: Vec<u8>) -> CPU {
        let mut cpu = CPU::default();
        let buffer = &mut buffer::Buffer::new_buffer();
        buffer.data = bytes;
        cpu.decode(buffer);
        cpu
    }

    pub fn update_button(&mut self, index: u8, pressed: bool) {
        self.bus.controller.update_button(index, pressed)
    }

    pub fn step_till_next_frame(&mut self) {
        while !self.bus.ppu.frame_complete() {
            let cpu_cycles = self.step();
            let ppu_cycles = cpu_cycles * 3; // 1 CPU cycle = 3 PPU cycles
            for _ in 0..ppu_cycles {
                self.bus.ppu.step();
            }
            for _ in 0..cpu_cycles {
                // todo: only read data when needed
                let addr = self.bus.apu.dmc.current_address;
                let dmc_data = self.bus.read(addr);
                self.bus.apu.step(dmc_data);
            }
        }
    }

    pub fn frame_buffer_ref(&self) -> &[u8] {
        self.bus.ppu.frame_buffer.as_ref()
    }

    pub fn load_samples(&mut self, buffer: &mut [f32]) {
        self.bus.apu.load_samples(buffer)
    }
}
