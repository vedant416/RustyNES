use crate::rom::CartData;

pub struct NROM {}

impl super::Mapper for NROM {
    fn read(&mut self, cart: &mut CartData, addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, cart: &mut CartData, addr: u16, val: u8) {
        todo!()
    }
}
