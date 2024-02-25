use super::Mapper;
use crate::rom::ROM;

pub struct NROM {
    prg_ram: [u8; 0x800],
    rom: ROM,
}

impl NROM {
    pub fn new(rom: ROM) -> Self {
        Self {
            prg_ram: [0; 0x800],
            rom,
        }
    }
}

impl Mapper for NROM {
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

    fn get_data(&self) -> &ROM {
        &self.rom
    }
}
