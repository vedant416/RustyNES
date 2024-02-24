use crate::mappers::{nrom::NROM, Mapper};

// mapper is a chip on the cartridge that controls
// how the program code and graphics data are read from the PRG ROM and CHR ROM
// read and write to Cartridge data (ROM file) is done through the mapper
pub type Cartridge = Box<dyn Mapper>;

#[derive(Debug)]
pub struct ROM {
    // contents of the ROM file (.nes file)
    pub bytes: Vec<u8>,

    //// Parsed metadata from the header of the ROM file ////
    // PRG ROM stores the game's program code
    // CHR ROM stores the game's graphics data

    // number of 16KB PRG ROM banks
    pub prg_rom_banks: u8,

    // number of 8KB CHR ROM banks
    pub chr_rom_banks: u8,

    // PRG ROM's start offset in rom file (vec bytes)
    pub prg_rom_start: usize,

    // CHR ROM's start offset in rom file (vec bytes)
    pub chr_rom_start: usize,

    // mapper determines from which bank to read the program code and graphics data
    pub mapper_id: u8,

    // mirroring mode determines how the nametables are mirrored
    pub mirroring: Mirroring,

    // trainer is a 512-byte data, it is not required for emulation of NES
    // trainer flag is used to determine if the trainer is present in the rom file
    pub trainer: bool,
}


/*
ines file format
header (16 bytes)
trainer (0 or 512 bytes)
prg rom (16KB * number of prg rom banks)
chr rom (8KB * number of chr rom banks)
*/
pub fn new_cartridge(bytes: Vec<u8>) -> Cartridge {
    // Check file signature
    let signature = &bytes[0..4];
    if signature != [78, 69, 83, 26] {
        panic!("Invalid file signature: {:?}", signature);
    }

    // Check ines version
    let ines_version = (bytes[7] & 0b1100_0000) >> 2;
    if ines_version != 0 {
        panic!("Invalid ines version: {}", ines_version);
    }

    // number of PRG ROM and CHR ROM banks
    let prg_rom_banks = bytes[4];
    let chr_rom_banks = bytes[5];

    // Check if trainer is present
    let trainer = (bytes[6] & 0b0000_0100) != 0;

    // Skip header bytes (16 bytes) and trainer bytes(0 or 512 bytes)
    // PRG ROM starts after the header and trainer
    let prg_rom_start = 16 + if trainer { 512 } else { 0 };
    let prg_rom_end = prg_rom_start + (prg_rom_banks as usize) * 0x4000;

    // CHR ROM starts after the PRG ROM
    let chr_rom_start = prg_rom_end;

    // Mirroring mode
    let mirroring = if (bytes[6] & 0b0000_1000) != 0 {
        Mirroring::FourScreen
    } else if (bytes[6] & 0b1) != 0 {
        Mirroring::Vertical
    } else {
        Mirroring::Horizontal
    };

    // Construct the mapper id from the header
    let mapper_id = (bytes[7] & 0b1111_0000) | (bytes[6] >> 4);

    println!("byte 0 {}", &bytes[0]);
    println!("byte 1 {}", &bytes[1]);
    println!("byte 2 {}", &bytes[2]);
    println!("byte 3 {}", &bytes[3]);
    println!("prg_rom_banks {}", &prg_rom_banks);
    println!("chr_rom_banks {}", &chr_rom_banks);
    println!("prg_rom_start {}", &prg_rom_start);
    println!("chr_rom_start {}", &chr_rom_start);
    println!("mirroring {:?}", &mirroring);
    println!("mapper_id {}", &mapper_id);
    println!("trainer {}", &trainer);

    // Create ROM
    let rom = ROM {
        bytes,
        prg_rom_banks,
        chr_rom_banks,
        prg_rom_start,
        chr_rom_start,
        mapper_id,
        mirroring,
        trainer,
    };

    let cartridge: Cartridge = match mapper_id {
        0 => Box::new(NROM::new(rom)),
        _ => panic!("Mapper not implemented: {}", rom.mapper_id),
    };
    cartridge
}

#[derive(Debug)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    OneScreenLower,
    OneScreenUpper,
    FourScreen,
}

impl Mirroring {
    pub fn get_address(&self, addr: u16) -> u16 {
        let addr = addr & 0x2FFF;
        match self {
            Mirroring::Horizontal => match addr {
                0x2000..=0x23FF => addr - 0x2000,
                0x2400..=0x27FF => addr - 0x2400,
                0x2800..=0x2bFF => addr - 0x2800 + 0x400,
                0x2C00..=0x2FFF => addr - 0x2C00 + 0x400,
                _ => unreachable!("Invalid address for horizontal mirroring: {:#X}", addr),
            },

            Mirroring::Vertical => match addr {
                0x2000..=0x23FF => addr - 0x2000,
                0x2400..=0x27FF => addr - 0x2400 + 0x400,
                0x2800..=0x2bFF => addr - 0x2800,
                0x2C00..=0x2FFF => addr - 0x2C00 + 0x400,
                _ => unreachable!("Invalid address for vertical mirroring: {:#X}", addr),
            },

            Mirroring::OneScreenLower => match addr {
                0x2000..=0x23FF => addr - 0x2000,
                0x2400..=0x27FF => addr - 0x2400,
                0x2800..=0x2bFF => addr - 0x2800,
                0x2C00..=0x2FFF => addr - 0x2C00,
                _ => unreachable!(
                    "Invalid address for one screen lower mirroring: {:#X}",
                    addr
                ),
            },

            Mirroring::OneScreenUpper => match addr {
                0x2000..=0x23FF => addr - 0x2000 + 0x400,
                0x2400..=0x27FF => addr - 0x2400 + 0x400,
                0x2800..=0x2bFF => addr - 0x2800 + 0x400,
                0x2C00..=0x2FFF => addr - 0x2C00 + 0x400,
                _ => unreachable!(
                    "Invalid address for one screen upper mirroring: {:#X}",
                    addr
                ),
            },

            Mirroring::FourScreen => addr - 0x2000,
        }
    }
}
