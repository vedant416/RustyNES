use crate::mappers::Mapper;

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
