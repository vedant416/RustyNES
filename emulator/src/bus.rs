use super::controller::Controller;
use super::ppu::PPU;

pub struct BUS {
    ram: [u8; 2 * 1024], // 2KB of RAM
    ppu: PPU,
    controller: Controller,
}

impl BUS {
    fn new(ppu: PPU, controller: Controller) -> Self {
        Self {
            ram: [0; 0x800],
            ppu,
            controller,
        }
    }

    pub fn read(&self, _address: u16) -> u8 {
        todo!()
    }

    pub fn write(&mut self, _address: u16, _data: u8) {
        todo!()
    }
}
