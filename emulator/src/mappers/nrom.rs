
pub struct NROM {}

impl super::Mapper for NROM {
    fn get_data(&self) -> &crate::rom::ROM {
        todo!()
    }

    fn read(&mut self, addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, addr: u16, val: u8) {
        todo!()
    }
}
