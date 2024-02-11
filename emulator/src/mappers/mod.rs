pub mod nrom;
use crate::rom::ROM;
pub trait Mapper {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
    fn get_data(&self) -> &ROM;
}