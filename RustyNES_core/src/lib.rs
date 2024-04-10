pub mod bus;
pub mod controller;
pub mod cpu;
pub mod mappers;
pub mod ppu;
pub mod rom;

use bus::BUS;
use controller::Controller;
pub use cpu::CPU;
use ppu::PPU;
use rom::ROM;

impl CPU {
    pub fn new_from_bytes(bytes: Vec<u8>) -> CPU {
        let cartridge = ROM::new_cartridge(bytes);
        let ppu = PPU::new_ppu(cartridge);
        let controller = Controller::new_controller();
        let bus = BUS::new_bus(ppu, controller);
        CPU::new_cpu(bus)
    }

    pub fn update_button(&mut self, index: u8, pressed: bool) {
        self.bus.controller.update_button(index, pressed)
    }

    pub fn frame_buffer(&mut self) -> &[u8] {
        // Run emulator until frame is complete
        // todo: refactor ppu to use while loop till we new frame is complete
        while !self.bus.ppu.frame_complete() {
            let cpu_cycles = self.step();
            let ppu_cycles = cpu_cycles * 3; // 1 CPU cycle = 3 PPU cycles
            for _ in 0..ppu_cycles {
                self.bus.ppu.step();
            }
        }
        self.bus.ppu.frame_buffer.as_ref()
    }
}
