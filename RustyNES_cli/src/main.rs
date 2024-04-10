use rusty_nes_core::CPU;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;
use std::env::args;
use std::fs::read;
use std::process;
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!("Usage: rusty_nes_cli <path-to-rom-file>");
    }
    let rom_path = &args[1];
    let bytes = read(rom_path).expect("failed to read ROM file");

    // Create cpu from bytes
    let mut cpu = CPU::new_from_bytes(bytes);

    // Save initial state of cpu
    // todo

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
        // Record frame start time
        frame_start_time = Instant::now();

        // Handle input
        handle_input(&mut cpu, &mut event_pump);

        // Get rendering data
        let frame_buffer = cpu.frame_buffer();

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
pub fn handle_input(c: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => process::exit(0),

            Event::KeyDown {
                keycode: Some(key), ..
            } => match key {
                Keycode::L => c.update_button(0, true),
                Keycode::K => c.update_button(1, true),

                Keycode::Space => c.update_button(2, true),
                Keycode::Return => c.update_button(3, true),

                Keycode::W => c.update_button(4, true),
                Keycode::S => c.update_button(5, true),
                Keycode::A => c.update_button(6, true),
                Keycode::D => c.update_button(7, true),

                Keycode::Escape => process::exit(0),
                // Keycode::N => *state = c.save(),
                // Keycode::M => c.load(state),
                _ => (),
            },

            Event::KeyUp {
                keycode: Some(key), ..
            } => match key {
                Keycode::L => c.update_button(0, false),
                Keycode::K => c.update_button(1, false),

                Keycode::Space => c.update_button(2, false),
                Keycode::Return => c.update_button(3, false),

                Keycode::W => c.update_button(4, false),
                Keycode::S => c.update_button(5, false),
                Keycode::A => c.update_button(6, false),
                Keycode::D => c.update_button(7, false),
                _ => (),
            },
            _ => (),
        }
    }
}
