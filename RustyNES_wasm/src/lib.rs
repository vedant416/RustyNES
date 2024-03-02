mod utils;

use rusty_nes_core::cpu::CPU;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct NES {
    cpu: CPU,
}

#[wasm_bindgen]
impl NES {
    pub fn new_nes(bytes: Vec<u8>) -> NES {
        utils::set_panic_hook();
        let cpu = CPU::new_from_bytes(bytes);
        NES { cpu }
    }

    pub fn step(&mut self) {
        while !self.cpu.bus.ppu.frame_complete() {
            let cpu_cycles = self.cpu.step();
            let ppu_cycles = cpu_cycles * 3;

            for _ in 0..ppu_cycles {
                self.cpu.bus.ppu.step();
            }
        }
    }

    pub fn frame_buffer_pointer(&self) -> *const u8 {
        self.cpu.bus.ppu.frame_buffer.as_ptr()
    }

    pub fn update_button(&mut self, index: u8, pressed: bool) {
        self.cpu.update_button(index, pressed);
    }
}
