pub mod bus;
pub mod controller;
pub mod cpu;
pub mod mappers;
pub mod ppu;
pub mod rom;

use self::{bus::BUS, controller::Controller, cpu::CPU, ppu::PPU, rom::new_cartridge};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, EventPump};
use std::{
    env::args,
    fs::read,
    process, thread,
    time::{Duration, Instant},
};

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

    // Initialize SDL
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("nes", (256 * 3) as u32, (240 * 3) as u32)
        .position_centered()
        .build()
        .expect("could not create window");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .expect("could not create canvas");

    // 3x scale canvas
    canvas.set_scale(3_f32, 3_f32).unwrap();

    // Create texture
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(PixelFormatEnum::ABGR8888, 256_u32, 240_u32)
        .unwrap();

    // Event handling
    let mut event_pump = sdl.event_pump().unwrap();

    // For 60 fps emulation
    let target_fps = 62_f64;
    let target_duration = Duration::from_secs_f64(1.0 / target_fps);
    let mut frame_start_time;

    // Game loop
    loop {
        // Reset start time
        frame_start_time = Instant::now();

        // Handle input
        handle_input(&mut event_pump, &mut cpu.bus.controller);

        // Get rendering data
        let frame_buffer = {
            // Run emulator until frame is complete
            while !cpu.bus.ppu.frame_complete() {
                let cpu_cycles = cpu.step();
                let ppu_cycles = cpu_cycles * 3; // 1 CPU cycle = 3 PPU cycles
                for _ in 0..ppu_cycles {
                    cpu.bus.ppu.step();
                }
            }
            &cpu.bus.ppu.frame_buffer
        };

        // Update texture
        texture.update(None, frame_buffer, 256 * 4).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        // Time taken to emulate frame
        let elapsed_time = frame_start_time.elapsed();
        // If frame was emulated faster than target duration, sleep for remaining time
        let remaining_time = target_duration.checked_sub(elapsed_time);
        if let Some(remaining_time) = remaining_time {
            thread::sleep(remaining_time);
        }
    }
}

/*
button 0: A
button 1: B
button 2: Select
button 3: Start
button 4: Up
button 5: Down
button 6: Left
button 7: Right
 */
pub fn handle_input(event_pump: &mut EventPump, controller: &mut Controller) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => process::exit(0),

            Event::KeyDown {
                keycode: Some(key), ..
            } => match key {
                Keycode::L => controller.update_button(0, true),
                Keycode::K => controller.update_button(1, true),
                Keycode::Space => controller.update_button(2, true),
                Keycode::Return => controller.update_button(3, true),
                Keycode::W => controller.update_button(4, true),
                Keycode::S => controller.update_button(5, true),
                Keycode::A => controller.update_button(6, true),
                Keycode::D => controller.update_button(7, true),
                Keycode::Escape => process::exit(0),
                _ => (),
            },

            Event::KeyUp {
                keycode: Some(key), ..
            } => match key {
                Keycode::L => controller.update_button(0, false),
                Keycode::K => controller.update_button(1, false),
                Keycode::Space => controller.update_button(2, false),
                Keycode::Return => controller.update_button(3, false),
                Keycode::W => controller.update_button(4, false),
                Keycode::S => controller.update_button(5, false),
                Keycode::A => controller.update_button(6, false),
                Keycode::D => controller.update_button(7, false),
                _ => (),
            },
            _ => (),
        }
    }
}
