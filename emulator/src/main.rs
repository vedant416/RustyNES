#![allow(dead_code)]
#![allow(unused_variables)]

use crate::rom::create_cartridge;
use std::{env::args, fs::read};

pub mod bus;
pub mod controller;
pub mod cpu;
pub mod mappers;
pub mod ppu;
pub mod rom;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!("Usage: emulator <path-to-rom-file>");
    }
    let rom_path = &args[1];
    let bytes = read(rom_path).expect("Failed to read ROM file");
    let cartridge = create_cartridge(bytes);
}
