mod utils;

use rusty_nes_core::cpu::CPU;
use rusty_nes_core::SAMPLE_RATE;
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
        self.cpu.step_till_next_frame();
    }

    pub fn frame_buffer_pointer(&self) -> *const u8 {
        self.cpu.bus.ppu.frame_buffer.as_ptr()
    }

    pub fn update_button(&mut self, index: u8, pressed: bool) {
        self.cpu.update_button(index, pressed);
    }

    pub fn load_audio_buffer(&mut self, buffer: &mut [f32]) {
        self.cpu.load_samples(buffer);
    }

    pub fn sample_rate(&self) -> f32 {
        SAMPLE_RATE
    }
}
