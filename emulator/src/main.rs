#![allow(dead_code)]
#![allow(unused_variables)]


use std::{env::args, fs::read};
use rom::new_cartridge;
use bus::BUS;
use controller::Controller;
use cpu::CPU;
use ppu::PPU;

pub mod bus;
pub mod controller;
pub mod cpu;
pub mod mappers;
pub mod ppu;
pub mod rom;
use sdl2::pixels::PixelFormatEnum;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!("Usage: emulator <path-to-rom-file>");
    }
    let rom_path = &args[1];
    let bytes = read(rom_path).expect("failed to read ROM file");

    // Create NES components
    let cartridge = new_cartridge(bytes);
    let ppu = PPU::new_ppu(cartridge);
    let controller = Controller::new_controller();
    let bus = BUS::new_bus(ppu, controller);
    let mut cpu = CPU::new_cpu(bus);


    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("nes", (256 * 3) as u32, (240 * 3) as u32)
        .build()
        .expect("could not create window");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("could not create canvas");

    // 3x scale canvas
    canvas.set_scale(3 as f32, 3 as f32).unwrap();

    // create texture
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(PixelFormatEnum::ABGR8888, 256 as u32, 240 as u32)
        .unwrap();


    loop {
        // Update emulator
        let frame_buffer = {
            while !cpu.bus.ppu.frame_complete {
                let cpu_cycles = cpu.step();
                let ppu_cycles = cpu_cycles * 3;

                for _ in 0..ppu_cycles {
                    cpu.bus.ppu.step();
                }
            }
            cpu.bus.ppu.frame_complete  = false;
            &cpu.bus.ppu.frame_buffer
        };

        // Update texture
        texture.update(None, frame_buffer, 256 * 4).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    }
}
