use bus::{BusState, BUS};
use controller::{Controller, ControllerState};
use cpu::CpuState;
pub use cpu::CPU;
use mappers::nrom::NROM;
use ppu::{PpuState, PPU};
use rom::{Cartridge, ROM};

pub mod bus;
pub mod controller;
pub mod cpu;
pub mod mappers;
pub mod ppu;
pub mod rom;

#[derive(Clone)]
pub struct State {
    cartridge_type: CartridgeType,
    controller_state: ControllerState,
    ppu_state: PpuState,
    bus_state: BusState,
    cpu_state: CpuState,
}

#[derive(Clone)]
enum CartridgeType {
    NROM(NROM),
}

impl CartridgeType {}

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

    pub fn save(&mut self) -> State {
        let cpu = self;
        let mapper_id = cpu.bus.ppu.cartridge.get_data().mapper_id;
        let cartridge = match mapper_id {
            0 => CartridgeType::NROM(
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
            cartridge_type: cartridge,
            controller_state,
            ppu_state,
            bus_state,
            cpu_state,
        }
    }

    pub fn load(&mut self, state: &State) {
        let State {
            cartridge_type: carts,
            controller_state,
            ppu_state,
            bus_state,
            cpu_state,
        } = state.clone();

        let cart = match carts {
            CartridgeType::NROM(cart) => cart,
        };

        let cartridge: Cartridge = Box::new(cart);
        let ppu = PPU::new_from_state(cartridge, ppu_state);
        let controller = Controller::new_from_state(controller_state);
        let bus = BUS::new_from_state(ppu, controller, bus_state);
        *self = CPU::new_from_state(bus, cpu_state)
    }
}
