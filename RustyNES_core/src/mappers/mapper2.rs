use crate::{buffer::Buffer, rom::ROM};

use super::Mapper;

#[derive(Clone, Debug)]
pub struct Mapper2 {
    prg_ram: [u8; 2048],
    chr_ram: [u8; 0x2000],
    bank: u8,
    rom: ROM,
}

impl Mapper2 {
    pub fn new(rom: ROM) -> Self {
        Mapper2 {
            prg_ram: [0; 0x800],
            chr_ram: [0; 0x2000],
            bank: 0,
            rom,
        }
    }
}

impl Mapper for Mapper2 {
    fn read(&mut self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            // CHR ROM/RAM
            0x0000..=0x1FFF => {
                if self.rom.chr_rom_banks == 0 {
                    self.chr_ram[addr]
                } else {
                    let addr = self.rom.chr_rom_start + addr;
                    self.rom.bytes[addr]
                }
            }

            // PRG RAM
            0x6000..=0x7FFF => self.prg_ram[(addr - 0x6000) & 0x7FF],

            // PRG ROM
            0x8000..=0xBFFF => {
                let bank = self.bank as usize * 0x4000;
                let addr = self.rom.prg_rom_start + bank + (addr & 0x3FFF);
                self.rom.bytes[addr]
            }

            // PRG ROM (last bank)
            0xC000..=0xFFFF => {
                let bank = (self.rom.prg_rom_banks - 1) as usize * 0x4000;
                let addr = self.rom.prg_rom_start + bank + (addr & 0x3FFF);
                self.rom.bytes[addr]
            }
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        let addr = addr as usize;
        match addr {
            // CHR RAM
            0x0000..=0x1FFF => {
                if self.rom.chr_rom_banks == 0 {
                    self.chr_ram[addr] = val;
                }
            }

            // PRG RAM
            0x6000..=0x7FFF => self.prg_ram[addr - 0x6000] = val,

            // Bank select
            0x8000..=0xFFFF => self.bank = val & 0b1111,

            _ => {}
        }
    }

    fn data(&self) -> &ROM {
        &self.rom
    }

    fn encode(&self, buffer: &mut Buffer) {
        buffer.write_u8_arr(&self.prg_ram);
        buffer.write_u8_arr(&self.chr_ram);
        buffer.write_u8(self.bank);
    }

    fn decode(&mut self, buffer: &mut Buffer) {
        buffer.read_u8_arr(&mut self.prg_ram);
        buffer.read_u8_arr(&mut self.chr_ram);
        self.bank = buffer.read_u8();
    }
}
