use rusty_nes_core::buffer::Buffer;
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
        set_panic_hook();
        let cpu = CPU::new_from_rom_bytes(bytes);
        add(1, 2);
        // throw_js_error();
        NES { cpu }
    }

    pub fn new_from_save_bytes(&mut self, bytes: Vec<u8>) -> NES {
        set_panic_hook();
        let cpu = CPU::new_nes_from_save_bytes(bytes);
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

    pub fn change_rom(&mut self, bytes: Vec<u8>) {
        self.cpu = CPU::new_from_rom_bytes(bytes);
    }

    pub fn get_state(&mut self) -> Vec<u8> {
        let buffer = &mut Buffer::new_buffer();
        self.cpu.encode(buffer);
        buffer.data.clone()
    }

    pub fn set_state(&mut self, bytes: Vec<u8>) {
        self.cpu = CPU::default();
        let buffer = &mut Buffer::new_from_bytes(bytes);
        self.cpu.decode(buffer);
    }

    pub fn throw_rust_error(&self) {
        panic!("Rust error");
    }
}

#[wasm_bindgen(module = "/foo.js")]
extern "C" {
    fn add(a: u32, b: u32) -> u32;
    fn throw_js_error();
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn onError(error: &str);
}

use std::panic;

pub fn set_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let error = info.to_string();
        onError(&error);
    }));
}