pub mod nrom;
use crate::rom::CartData;
pub trait Mapper {
    fn read(&mut self, cart_data: &mut CartData, addr: u16) -> u8;
    fn write(&mut self, cart_data: &mut CartData, addr: u16, val: u8);
}