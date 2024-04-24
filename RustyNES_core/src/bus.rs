use crate::apu::APU;
use crate::buffer;

use super::controller::Controller;
use super::ppu::PPU;

// BUS connects CPU to PPU, APU, RAM, Cartridge, and Controller.
pub struct BUS {
    ram: [u8; 0x800], // 2KB of RAM
    pub ppu: PPU,
    pub controller: Controller,
    pub apu: APU,
}

impl Default for BUS {
    fn default() -> Self {
        Self {
            ram: [0; 0x800],
            ppu: PPU::default(),
            controller: Controller::default(),
            apu: APU::new(),
        }
    }
}

impl BUS {
    pub fn new_bus(ppu: PPU, controller: Controller) -> Self {
        Self {
            ram: [0; 0x800],
            ppu,
            controller,
            apu: APU::new(),
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // 2KB of RAM is mirrored 4 times over the address space 0x0000-0x1FFF
            0x0000..=0x1FFF => self.ram[(addr & 0x7FF) as usize],
            // PPU registers are mirrored every 8 bytes from 0x2008 to 0x3FFF
            // addr & 7 masks the address to 0-7
            0x2000..=0x3FFF => self.ppu.read_register(addr & 7),
            0x4016 => self.controller.read(),
            0x4000..=0x4017 => self.apu.read(addr),
            0x4018..=0x401F => 0, // unused
            0x4020..=0xFFFF => self.ppu.cartridge.read(addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            // 2KB of RAM is mirrored 4 times over the address space 0x0000-0x1FFF
            0x0000..=0x1FFF => self.ram[(addr & 0x7FF) as usize] = val,
            // PPU registers are mirrored every 8 bytes from 0x2008 to 0x3FFF
            // addr & 7 masks the address to 0-7
            0x2000..=0x3FFF => self.ppu.write_register(addr & 7, val),
            0x4014 => self.dma(val),
            0x4000..=0x4017 => self.apu.write(addr, val),
            0x4018..=0x401F => (), // unused
            0x4020..=0xFFFF => self.ppu.cartridge.write(addr, val),
        }
    }

    // DMA(Direct Memory Access) is used to transfer 256 bytes of data from CPU memory to OAM memory.
    // OAM memory is used to store the sprite attributes.
    // val is hi byte of addr to read 256 bytes from.
    fn dma(&mut self, val: u8) {
        let hi = (val as u16) << 8;
        for lo in 0..256 {
            let data = self.read(hi | lo);
            self.ppu.write_oam_data(data);
        }
        self.ppu.dma_triggered = true;
    }
}

impl BUS {
    pub fn encode(&self, buffer: &mut buffer::Buffer) {
        buffer.write_u8_arr(&self.ram);
    }

    pub fn decode(&mut self, buffer: &mut buffer::Buffer) {
        buffer.read_u8_arr(&mut self.ram);
    }
}
