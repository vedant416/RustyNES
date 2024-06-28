use super::Mapper;
use crate::{buffer, rom::ROM};

#[derive(Clone, Debug)]
pub struct Mapper0 {
    prg_ram: [u8; 0x800],
    rom: ROM,
}

impl Default for Mapper0 {
    fn default() -> Self {
        Self {
            prg_ram: [0; 0x800],
            rom: ROM::default(),
        }
    }
}

impl Mapper0 {
    pub fn new(rom: ROM) -> Self {
        Self {
            prg_ram: [0; 0x800],
            rom,
        }
    }
}

impl Mapper for Mapper0 {
    fn read(&mut self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            // CHR ROM
            0x0000..=0x1FFF => {
                let offset = self.rom.chr_rom_start + addr;
                self.rom.bytes[offset]
            }

            // PRG RAM
            0x6000..=0x7FFF => self.prg_ram[(addr - 0x6000) & 0x7FF],

            // PRG ROM
            0x8000..=0xFFFF => {
                let offset = {
                    let mut addr = addr - 0x8000;
                    if self.rom.prg_rom_banks == 1 {
                        addr &= 0x3FFF;
                    }
                    self.rom.prg_rom_start + addr
                };
                self.rom.bytes[offset]
            }

            _ => unreachable!("Invalid read address for NROM: {:#X}", addr),
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        let addr = addr as usize;
        match addr {
            // PRG RAM
            0x6000..=0x7FFF => self.prg_ram[(addr - 0x6000) & 0x7FF] = val,
            _ => (),
        }
    }

    fn get_rom(&self) -> &ROM {
        &self.rom
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn encode(&self, buffer: &mut buffer::Buffer) {
        buffer.write_u8_arr(&self.prg_ram);
    }

    fn decode(&mut self, buffer: &mut buffer::Buffer) {
        buffer.read_u8_arr(&mut self.prg_ram);
    }
}
