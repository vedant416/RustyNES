pub mod nrom;
use crate::rom::Cart;
pub trait Mapper {
    fn read(&mut self, cart: &mut Cart, addr: u16) -> u8;
    fn write(&mut self, cart: &mut Cart, addr: u16, val: u8);
}