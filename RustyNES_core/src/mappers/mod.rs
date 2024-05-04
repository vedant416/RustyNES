pub mod nrom;
use crate::{buffer, rom::ROM};
pub trait Mapper {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
    fn get_rom(&self) -> &ROM;

    // only used in MMC3
    fn step(&mut self) {}
    fn irq_triggered(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn std::any::Any;

    fn encode(&self, buffer: &mut buffer::Buffer);
    fn decode(&mut self, buffer: &mut buffer::Buffer);
}