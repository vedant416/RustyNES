use super::Mapper;
use crate::rom::{Mirroring, ROM};

#[derive(Clone, Debug)]
pub struct Mapper4 {
    registers: [u8; 8],
    reg_index: u8,
    prg_mode: u8,
    chr_mode: u8,
    prg_ram: [u8; 0x2000],
    prg_offsets: [u32; 4],
    chr_offsets: [u32; 8],
    irq_enabled: bool,
    irq_reload: u8,
    irq_counter: u8,
    irq_triggered: bool,
    rom: ROM,
}

impl Mapper4 {
    pub fn new(rom: ROM) -> Mapper4 {
        let bank_count = rom.prg_rom_banks as u32 * 2;
        let offset0 = 0; // bank1
        let offset1 = 0x2000; // bank2
        let offset2 = (bank_count - 2) * 0x2000; // 2nd last bank
        let offset3 = (bank_count - 1) * 0x2000; // last bank
        Mapper4 {
            reg_index: 0,
            registers: [0; 8],
            chr_mode: 0,
            prg_mode: 0,
            prg_ram: [0; 0x2000],
            prg_offsets: [offset0, offset1, offset2, offset3],
            chr_offsets: [0; 8],
            irq_enabled: false,
            irq_reload: 0,
            irq_counter: 0,
            irq_triggered: false,
            rom,
        }
    }

    fn write_mirror(&mut self, addr: u16, val: u8) {
        if addr & 1 == 0 {
            if val & 1 == 0 {
                self.rom.mirroring = Mirroring::Vertical
            } else {
                self.rom.mirroring = Mirroring::Horizontal
            }
        }
    }

    fn write_reload(&mut self, addr: u16, val: u8) {
        if addr & 1 == 0 {
            self.irq_reload = val;
        } else {
            self.irq_counter = 0;
        }
    }

    fn write_irq(&mut self, addr: u16) {
        self.irq_enabled = addr & 1 == 1;
        if !self.irq_enabled {
            self.irq_triggered = false;
        }
    }

    fn write_mode(&mut self, val: u8) {
        self.chr_mode = (val >> 7) & 1;
        self.prg_mode = (val >> 6) & 1;
        self.reg_index = val & 0b111;
    }

    fn write_offsets(&mut self, val: u8) {
        self.registers[self.reg_index as usize] = val;
        let _1kb = 0x400;
        if self.chr_mode == 0 {
            self.chr_offsets[0] = (self.registers[0]) as u32 * _1kb;
            self.chr_offsets[1] = (self.registers[0] + 1) as u32 * _1kb;
            self.chr_offsets[2] = (self.registers[1]) as u32 * _1kb;
            self.chr_offsets[3] = (self.registers[1] + 1) as u32 * _1kb;

            self.chr_offsets[4] = (self.registers[2]) as u32 * _1kb;
            self.chr_offsets[5] = (self.registers[3]) as u32 * _1kb;
            self.chr_offsets[6] = (self.registers[4]) as u32 * _1kb;
            self.chr_offsets[7] = (self.registers[5]) as u32 * _1kb;
        }
        // swap banks
        else {
            self.chr_offsets[4] = (self.registers[0]) as u32 * _1kb;
            self.chr_offsets[5] = (self.registers[0] + 1) as u32 * _1kb;
            self.chr_offsets[6] = (self.registers[1]) as u32 * _1kb;
            self.chr_offsets[7] = (self.registers[1] + 1) as u32 * _1kb;

            self.chr_offsets[0] = (self.registers[2]) as u32 * _1kb;
            self.chr_offsets[1] = (self.registers[3]) as u32 * _1kb;
            self.chr_offsets[2] = (self.registers[4]) as u32 * _1kb;
            self.chr_offsets[3] = (self.registers[5]) as u32 * _1kb;
        }

        // prg_rom_banks is in 16kb units
        // but we need to convert it to 8kb units
        let bank_count = self.rom.prg_rom_banks * 2;
        let _8kb = 0x2000;
        if self.prg_mode == 0 {
            self.prg_offsets[0] = self.registers[6] as u32 * _8kb; // fixed bank
            self.prg_offsets[1] = self.registers[7] as u32 * _8kb; // fixed bank
            self.prg_offsets[2] = (bank_count - 2) as u32 * _8kb; // 2nd last bank
            self.prg_offsets[3] = (bank_count - 1) as u32 * _8kb; // last bank
        }
        // swap banks
        else {
            self.prg_offsets[2] = self.registers[6] as u32 * _8kb;
            self.prg_offsets[1] = self.registers[7] as u32 * _8kb;
            self.prg_offsets[0] = (bank_count - 2) as u32 * _8kb;
            self.prg_offsets[3] = (bank_count - 1) as u32 * _8kb;
        }
    }
}

impl Mapper for Mapper4 {
    fn step(&mut self) {
        if self.irq_counter == 0 {
            self.irq_counter = self.irq_reload;
        } else {
            self.irq_counter -= 1;
        }

        if self.irq_counter == 0 && self.irq_enabled {
            self.irq_triggered = true;
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // CHR ROM
            0x0000..=0x1FFF => {
                let index = (addr / 0x400) as usize;
                let offset = self.chr_offsets[index] as usize + (addr & 0x3FF) as usize;
                self.rom.bytes[self.rom.chr_rom_start + offset]
            }

            // PRG RAM
            0x6000..=0x7FFF => self.prg_ram[(addr - 0x6000) as usize],

            // PRG ROM
            0x8000..=0xFFFF => {
                let index = ((addr - 0x8000) / 0x2000) as usize;
                let offset = self.prg_offsets[index] as usize + (addr & 0x1FFF) as usize;
                self.rom.bytes[self.rom.prg_rom_start + offset]
            }
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x6000..=0x7FFF => self.prg_ram[(addr - 0x6000) as usize] = val,

            0x8000..=0x9FFF => {
                if addr & 1 == 0 {
                    self.write_mode(val);
                } else {
                    self.write_offsets(val);
                }
            }

            0xA000..=0xBFFF => self.write_mirror(addr, val),

            0xC000..=0xDFFF => self.write_reload(addr, val),

            0xE000..=0xFFFF => self.write_irq(addr),

            _ => {}
        }
    }

    fn irq_triggered(&mut self) -> bool {
        let triggered = self.irq_triggered;
        self.irq_triggered = false;
        triggered
    }

    fn data(&self) -> &ROM {
        &self.rom
    }

    fn encode(&self, buffer: &mut crate::buffer::Buffer) {
        buffer.write_u8_arr(&self.registers);
        buffer.write_u8(self.reg_index);
        buffer.write_u8(self.prg_mode);
        buffer.write_u8(self.chr_mode);
        buffer.write_u8_arr(&self.prg_ram);
        buffer.write_u32_arr(&self.prg_offsets);
        buffer.write_u32_arr(&self.chr_offsets);
        buffer.write_bool(self.irq_enabled);
        buffer.write_u8(self.irq_reload);
        buffer.write_u8(self.irq_counter);
        buffer.write_bool(self.irq_triggered);
    }

    fn decode(&mut self, buffer: &mut crate::buffer::Buffer) {
        buffer.read_u8_arr(&mut self.registers);
        self.reg_index = buffer.read_u8();
        self.prg_mode = buffer.read_u8();
        self.chr_mode = buffer.read_u8();
        buffer.read_u8_arr(&mut self.prg_ram);
        buffer.read_u32_arr(&mut self.prg_offsets);
        buffer.read_u32_arr(&mut self.chr_offsets);
        self.irq_enabled = buffer.read_bool();
        self.irq_reload = buffer.read_u8();
        self.irq_counter = buffer.read_u8();
        self.irq_triggered = buffer.read_bool();
    }
}
