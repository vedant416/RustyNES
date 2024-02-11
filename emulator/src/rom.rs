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
