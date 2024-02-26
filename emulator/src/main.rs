use emulator::{
    bus::{BusState, BUS},
    controller::{Controller, ControllerState},
    cpu::{CpuState, CPU},
    mappers::nrom::NROM,
    ppu::{PpuState, PPU},
    rom::{new_cartridge, Cartridge},
};
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
    // Save initial state of Emulator
    let mut state = get_state(&mut cpu);

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
        handle_input(&mut cpu, &mut state, &mut event_pump);

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
pub fn handle_input(cpu: &mut CPU, state: &mut State, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        let controller = &mut cpu.bus.controller;
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
                Keycode::N => *state = get_state(cpu),
                Keycode::M => load_state(cpu, state),
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

#[derive(Clone)]
enum Carts {
    NROM(NROM),
}

#[derive(Clone)]
pub struct State {
    carts: Carts,
    controller_state: ControllerState,
    ppu_state: PpuState,
    bus_state: BusState,
    cpu_state: CpuState,
}

fn get_state(cpu: &mut CPU) -> State {
    let mapper_id = cpu.bus.ppu.cartridge.get_data().mapper_id;
    let cartridge = match mapper_id {
        0 => Carts::NROM(
            cpu.bus
                .ppu
                .cartridge
                .as_any()
                .downcast_ref::<NROM>()
                .unwrap()
                .to_owned(),
        ),
        _ => panic!("Mapper not supported"),
    };

    let controller_state = Controller::get_state(&cpu.bus.controller);
    let ppu_state = PPU::get_state(&cpu.bus.ppu);
    let bus_state = BUS::get_state(&cpu.bus);
    let cpu_state = CPU::get_state(cpu);
    State {
        carts: cartridge,
        controller_state,
        ppu_state,
        bus_state,
        cpu_state,
    }
}

fn load_state(cpu: &mut CPU, state: &State) {
    let State {
        carts,
        controller_state,
        ppu_state,
        bus_state,
        cpu_state,
    } = state.clone();

    let cart = match carts {
        Carts::NROM(cart) => cart,
    };

    let cartridge: Cartridge = Box::new(cart);
    let ppu = PPU::new_from_state(cartridge, ppu_state);
    let controller = Controller::new_from_state(controller_state);
    let bus = BUS::new_from_state(ppu, controller, bus_state);
    *cpu = CPU::set_state(bus, cpu_state)
}
